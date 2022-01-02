use crate::linalg::{Length, Three};
use crate::material::Material;
use crate::ray::{Absorb, Bounce, Hit, Interaction, Ray};
use crate::scene::{ObjectIdx, Scene};
use rand::prelude::Rng;
use rand_distr::{Distribution, UnitSphere};

pub struct PathTracer {
    scene: Scene,
    interactions: Vec<Option<Interaction>>,
    depth: usize,
}

impl PathTracer {
    pub fn new(scene: Scene, depth: usize) -> Self {
        Self {
            scene,
            interactions: Vec::with_capacity(depth),
            depth,
        }
    }

    pub fn trace<R: Rng>(&mut self, root_ray: Ray, rng: &mut R) -> Three<f64> {
        assert!(self.interactions.len() == 0);
        let mut opt_ray = Some(root_ray);
        while let Some(ray) = opt_ray {
            let opt_bounce = self
                .scene
                .objects()
                .iter()
                .enumerate()
                .map(|(i, obj)| obj.hit_by(ray).map(|h| (h, ObjectIdx(i))))
                .filter(|h| h.is_some())
                .min()
                .flatten()
                .map(|(hit, obj_idx)| {
                    match hit.scatter(ray, self.scene.material_for(obj_idx), rng) {
                        Some(direction) => Interaction::Bounced(Bounce {
                            incoming: ray,
                            hit,
                            obj_idx,
                            outgoing: Ray {
                                origin: hit.position,
                                direction,
                            },
                        }),
                        None => Interaction::Absorbed(Absorb {
                            incoming: ray,
                            hit,
                            obj_idx,
                        }),
                    }
                });
            self.interactions.push(opt_bounce);
            opt_ray = opt_bounce
                .map(|i| match i {
                    Interaction::Bounced(b) => Some(b.outgoing),
                    _ => None,
                })
                .flatten();
            if self.interactions.len() >= self.depth {
                break;
            }
        }

        let mut color: Three<f64> = (0.0, 0.0, 0.0).into();
        while let Some(interaction) = self.interactions.pop() {
            match interaction {
                Some(Interaction::Bounced(bounce)) => {
                    color *= bounce.albedo(self.scene.material_for(bounce.obj_idx))
                }
                Some(Interaction::Absorbed(absorb)) => {
                    color = absorb.emit(self.scene.material_for(absorb.obj_idx))
                }
                None => color.fill(0.0),
            }
        }
        color
    }
}

impl Bounce {
    pub fn albedo(&self, material: &Material) -> Three<f64> {
        match material {
            &Material::Lambertian { rgb } => {
                let cos_theta = self.outgoing.direction.dot(&self.hit.normal).max(0.0);
                rgb * cos_theta
            }
            &Material::Metal { rgb, fuzz: _ } => rgb,
            _ => Three::new(1.0, 1.0, 1.0),
        }
    }
}

impl Absorb {
    pub fn emit(&self, material: &Material) -> Three<f64> {
        match material {
            &Material::DiffuseLight { rgb } => rgb,
            _ => Three::new(0.0, 0.0, 0.0),
        }
    }
}

impl Hit {
    pub fn scatter<R: Rng>(
        &self,
        ray: Ray,
        material: &Material,
        rng: &mut R,
    ) -> Option<Three<f64>> {
        match material {
            &Material::Lambertian { rgb: _ } => {
                let scattered = self.normal;
                let mut noise = Three::random_unit(rng);
                if noise.dot(&self.normal).is_sign_negative() {
                    noise = -noise;
                }
                let direction = (scattered + noise).normalized();
                Some(direction).filter(|d| d.dot(&self.normal) > 0.0)
            }
            &Material::Metal { rgb: _, fuzz } => {
                let scattered = ray.direction.reflect(&self.normal);
                let mut noise = Three::random_unit(rng);
                if noise.dot(&self.normal).is_sign_negative() {
                    noise = -noise;
                }
                let direction = (scattered + noise * fuzz).normalized();
                Some(direction).filter(|d| d.dot(&self.normal) > 0.0)
            }
            &Material::Dielectric(index_of_refraction) => {
                let cos_theta = ray.direction.dot(&self.normal);
                let exiting = cos_theta.is_sign_positive();
                let outward_normal = if exiting { -self.normal } else { self.normal };
                let ratio = if exiting {
                    index_of_refraction.value()
                } else {
                    1.0 / index_of_refraction.value()
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

impl Three<f64> {
    fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * (self.dot(normal) * 2.0)
    }

    fn random_unit<R: Rng>(rng: &mut R) -> Self {
        UnitSphere.sample(rng).into()
    }
}
