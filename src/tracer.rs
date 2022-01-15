use crate::linalg::Three;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::ray::{Hit, Ray};
use crate::scene::Scene;
use rand::prelude::Rng;
use rand_distr::{Distribution, UnitBall, UnitSphere};

pub trait Tracer {
    fn trace<R: Rng>(&self, ray: Ray, rng: &mut R) -> Three<f64>;
}

pub struct PathTracer {
    scene: Scene,
    depth: usize,
}

enum LightBehavior {
    Scatter { direction: Three<f64> },
    Absorb,
}

struct MaterialInteraction {
    attenuation: Three<f64>,
    light_behavior: LightBehavior,
}

impl PathTracer {
    pub fn new(scene: Scene, depth: usize) -> Self {
        Self { scene, depth }
    }
}

impl Tracer for PathTracer {
    fn trace<R: Rng>(&self, mut ray: Ray, rng: &mut R) -> Three<f64> {
        let mut color: Three<f64> = 1.0.into();
        for _ in 0..self.depth {
            match self.scene.cast(&ray) {
                Some((hit, material)) => {
                    let interaction = material.interact(&ray, &hit, rng);
                    color *= interaction.attenuation;
                    match interaction.light_behavior {
                        LightBehavior::Scatter { direction } => {
                            ray.origin = hit.position;
                            ray.direction = direction;
                        }
                        LightBehavior::Absorb => return color,
                    }
                }
                None => return 0.0.into(),
            };
        }
        0.0.into()
    }
}

impl Material {
    fn interact<R: Rng>(&self, ray: &Ray, hit: &Hit, rng: &mut R) -> MaterialInteraction {
        match self {
            Material::Lambertian(material) => material.interact(ray, hit, rng),
            Material::Metal(material) => material.interact(ray, hit, rng),
            Material::Dielectric(material) => material.interact(ray, hit, rng),
            Material::DiffuseLight(material) => material.interact(ray, hit, rng),
        }
    }
}

impl Lambertian {
    fn interact<R: Rng>(&self, _ray: &Ray, hit: &Hit, rng: &mut R) -> MaterialInteraction {
        let mut noise = random_unit(rng);
        noise *= noise.dot(&hit.normal).signum(); // NOTE: make noise face in same direction as normal
        let direction = (&hit.normal + &noise).normalized();
        let cos_theta = direction.dot(&hit.normal).max(0.0);
        MaterialInteraction {
            attenuation: &self.rgb * cos_theta,
            light_behavior: LightBehavior::Scatter { direction },
        }
    }
}

impl Metal {
    fn interact<R: Rng>(&self, ray: &Ray, hit: &Hit, rng: &mut R) -> MaterialInteraction {
        let mut direction = reflect(&ray.direction, &hit.normal);
        if let Some(value) = self.fuzz {
            let mut noise = random_unit_in(rng);
            noise *= noise.dot(&hit.normal).signum() * value;
            direction = (&direction + &noise).normalized();
        }
        MaterialInteraction {
            attenuation: self.rgb,
            light_behavior: LightBehavior::Scatter { direction },
        }
    }
}

impl Dielectric {
    fn interact<R: Rng>(&self, ray: &Ray, hit: &Hit, rng: &mut R) -> MaterialInteraction {
        let cos_theta = ray.direction.dot(&hit.normal);
        let exiting = cos_theta > 0.0;
        let outward_normal = if exiting { -hit.normal } else { hit.normal };
        let ratio = if exiting {
            self.ior.value()
        } else {
            self.ior.value().recip()
        };
        let cos_theta = cos_theta.abs();
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        // shclick approximation
        let r0 = (1.0 - ratio) / (1.0 + ratio);
        let r1 = r0 * r0;
        let reflectance = r1 + (1.0 - r1) * (1.0 - cos_theta).powi(5);

        let direction = if ratio * sin_theta > 1.0 || reflectance > rng.gen_range(0.0..1.0) {
            reflect(&ray.direction, &outward_normal)
        } else {
            let perp = (&ray.direction + &(&outward_normal * cos_theta)) * ratio;
            let para = &outward_normal * -(1.0 - perp.length_squared()).abs().sqrt();
            (perp + para).normalized()
        };

        MaterialInteraction {
            attenuation: Three::new(1.0, 1.0, 1.0),
            light_behavior: LightBehavior::Scatter { direction },
        }
    }
}

impl DiffuseLight {
    fn interact<R: Rng>(&self, _ray: &Ray, _hit: &Hit, _rng: &mut R) -> MaterialInteraction {
        MaterialInteraction {
            attenuation: self.rgb * self.power,
            light_behavior: LightBehavior::Absorb,
        }
    }
}

fn reflect(d: &Three<f64>, n: &Three<f64>) -> Three<f64> {
    d - &(n * (d.dot(&n) * 2.0))
}

fn random_unit<R: Rng>(rng: &mut R) -> Three<f64> {
    UnitSphere.sample(rng).into()
}

fn random_unit_in<R: Rng>(rng: &mut R) -> Three<f64> {
    UnitBall.sample(rng).into()
}
