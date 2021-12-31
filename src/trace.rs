use crate::linalg::{Length, Vec3};
use crate::objects::Object;
use crate::ray::{Absorb, Bounce, CanHit, Hit, Interaction, Material, Ray};
use rand::prelude::{Distribution, Rng};
use rand_distr::StandardNormal;

pub(crate) struct LightDynamics {
    objects: Vec<Object>,
    interactions: Vec<Option<Interaction>>,
    num_bounces: usize,
}

impl LightDynamics {
    pub(crate) fn new(objects: Vec<Object>, num_bounces: usize) -> Self {
        Self {
            objects,
            interactions: Vec::with_capacity(num_bounces),
            num_bounces,
        }
    }

    pub(crate) fn trace<R: Rng>(&mut self, root_ray: Ray, rng: &mut R) -> Vec3<f64> {
        assert!(self.interactions.len() == 0);
        let mut opt_ray = Some(root_ray);
        while let Some(ray) = opt_ray {
            let opt_bounce = self
                .objects
                .iter()
                .map(|obj| obj.hit_by(ray))
                .filter(|h| h.is_some())
                .min()
                .flatten()
                .map(|hit| match hit.scatter(ray, rng) {
                    Some(direction) => Interaction::Bounced(Bounce {
                        incoming: ray,
                        hit,
                        outgoing: Ray {
                            origin: hit.position,
                            direction,
                        },
                    }),
                    None => Interaction::Absorbed(Absorb { incoming: ray, hit }),
                });
            self.interactions.push(opt_bounce);
            opt_ray = opt_bounce
                .map(|i| match i {
                    Interaction::Bounced(b) => Some(b.outgoing),
                    _ => None,
                })
                .flatten();
            if self.interactions.len() >= self.num_bounces {
                break;
            }
        }

        let mut color: Vec3<f64> = (0.0, 0.0, 0.0).into();
        while let Some(interaction) = self.interactions.pop() {
            match interaction {
                Some(Interaction::Bounced(bounce)) => color *= bounce.albedo(),
                Some(Interaction::Absorbed(absorb)) => color = absorb.emit(),
                None => color.fill(0.0),
            }
        }
        color
    }
}

impl Bounce {
    pub(crate) fn albedo(&self) -> Vec3<f64> {
        match self.hit.material {
            Material::Lambertian { rgb } => {
                // let cos_theta = self.outgoing.direction.dot(&self.hit.normal).max(0.0);
                rgb
            }
            Material::Metal { rgb, fuzz: _ } => rgb,
            _ => Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Absorb {
    pub(crate) fn emit(&self) -> Vec3<f64> {
        match self.hit.material {
            Material::DiffuseLight { rgb } => rgb,
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Hit {
    pub(crate) fn scatter<R: Rng>(&self, ray: Ray, rng: &mut R) -> Option<Vec3<f64>> {
        match self.material {
            Material::Lambertian { rgb: _ } => {
                let scattered = self.normal;
                let mut noise = Vec3::random_unit(rng);
                if noise.dot(&self.normal).is_sign_negative() {
                    noise = -noise;
                }
                let direction = (scattered + noise).normalized();
                Some(direction).filter(|d| d.dot(&self.normal) > 0.0)
            }
            Material::Metal { rgb: _, fuzz } => {
                let scattered = ray.direction.reflect(&self.normal);
                let mut noise = Vec3::random_unit(rng);
                if noise.dot(&self.normal).is_sign_negative() {
                    noise = -noise;
                }
                let direction = (scattered + noise * fuzz).normalized();
                Some(direction).filter(|d| d.dot(&self.normal) > 0.0)
            }
            Material::Dielectric(refractor) => {
                let cos_theta = ray.direction.dot(&self.normal);
                let exiting = cos_theta.is_sign_positive();
                let outward_normal = if exiting { -self.normal } else { self.normal };
                let ratio = if exiting {
                    refractor.index()
                } else {
                    1.0 / refractor.index()
                };
                let cos_theta = cos_theta.abs();
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                // shclick approximation
                let r0 = (1.0 - ratio) / (1.0 + ratio);
                let r1 = r0 * r0;
                let reflectance = r1 + (1.0 - r1) * (1.0 - cos_theta).powi(5);

                if ratio * sin_theta > 1.0 || reflectance > rng.gen_range(0.0..1.0) {
                    Some(ray.direction.reflect(&outward_normal).normalized())
                } else {
                    let perp = (ray.direction + outward_normal * cos_theta) * ratio;
                    let para = outward_normal * -(1.0 - perp.length_squared()).abs().sqrt();
                    Some((perp + para).normalized())
                }
            }
            Material::DiffuseLight { rgb: _ } => None,
        }
    }
}

impl Vec3<f64> {
    fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * (self.dot(normal) * 2.0)
    }

    fn random_unit<R: Rng>(rng: &mut R) -> Self {
        let mut v = Self::new(
            StandardNormal.sample(rng),
            StandardNormal.sample(rng),
            StandardNormal.sample(rng),
        );
        v.normalize();
        v
    }
}
