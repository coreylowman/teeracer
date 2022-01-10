use crate::linalg::{Length, Three};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::ray::{Hit, Interaction, Ray};
use crate::scene::Scene;
use rand::prelude::Rng;
use rand_distr::{Distribution, UnitBall, UnitSphere};

pub struct PathTracer {
    scene: Scene,
    interactions: Vec<Interaction>,
    depth: usize,
}

struct Scatter {
    direction: Option<Three<f64>>,
    color: Three<f64>,
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
            let opt_hit = self.scene.hit(&ray);
            let interaction = match opt_hit {
                None => {
                    opt_ray = None;
                    Interaction::Missed
                }
                Some((hit, obj_idx)) => {
                    let material = self.scene.material_for(obj_idx);
                    let scatter = scatter(&ray, &hit, material, rng);
                    opt_ray = scatter.direction.map(|direction| Ray {
                        origin: hit.position,
                        direction,
                    });
                    match scatter.direction {
                        Some(_) => Interaction::Bounced {
                            attenuation: scatter.color,
                        },
                        None => Interaction::Absorbed {
                            emission: scatter.color,
                        },
                    }
                }
            };
            self.interactions.push(interaction);
            if self.interactions.len() >= self.depth {
                break;
            }
        }

        let mut color: Three<f64> = (0.0, 0.0, 0.0).into();
        while let Some(interaction) = self.interactions.pop() {
            match interaction {
                Interaction::Bounced { attenuation } => color *= attenuation,
                Interaction::Absorbed { emission } => color += emission,
                Interaction::Missed => color.fill(0.0),
            }
        }
        color
    }
}

fn scatter<R: Rng>(ray: &Ray, hit: &Hit, material: &Material, rng: &mut R) -> Scatter {
    match material {
        Material::Lambertian(material) => material.scatter(ray, hit, rng),
        Material::Metal(material) => material.scatter(ray, hit, rng),
        Material::Dielectric(material) => material.scatter(ray, hit, rng),
        Material::DiffuseLight(material) => material.scatter(ray, hit, rng),
    }
}

impl Lambertian {
    fn scatter<R: Rng>(&self, _ray: &Ray, hit: &Hit, rng: &mut R) -> Scatter {
        let mut noise = random_unit(rng);
        noise *= noise.dot(&hit.normal).signum(); // NOTE: make noise face in same direction as normal
        let direction = (&hit.normal + &noise).normalized();
        let cos_theta = direction.dot(&hit.normal).max(0.0);
        Scatter {
            direction: Some(direction),
            color: self.rgb * cos_theta,
        }
    }
}

impl Metal {
    fn scatter<R: Rng>(&self, ray: &Ray, hit: &Hit, rng: &mut R) -> Scatter {
        let mut direction = reflect(&ray.direction, &hit.normal).normalized();
        if let Some(value) = self.fuzz {
            let noise = random_unit_in(rng);
            direction += noise * noise.dot(&hit.normal).signum() * value;
            direction.normalize();
        }
        Scatter {
            direction: Some(direction),
            color: self.rgb,
        }
    }
}

impl Dielectric {
    fn scatter<R: Rng>(&self, ray: &Ray, hit: &Hit, rng: &mut R) -> Scatter {
        let cos_theta = ray.direction.dot(&hit.normal);
        let exiting = cos_theta > 0.0;
        let outward_normal = if exiting { -hit.normal } else { hit.normal };
        let ratio = if exiting {
            self.ior.value()
        } else {
            1.0 / self.ior.value()
        };
        let cos_theta = cos_theta.abs();
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        // shclick approximation
        let r0 = (1.0 - ratio) / (1.0 + ratio);
        let r1 = r0 * r0;
        let reflectance = r1 + (1.0 - r1) * (1.0 - cos_theta).powi(5);

        let direction = if ratio * sin_theta > 1.0 || reflectance > rng.gen_range(0.0..1.0) {
            reflect(&ray.direction, &outward_normal).normalized()
        } else {
            let perp = (&ray.direction + &(&outward_normal * cos_theta)) * ratio;
            let para = &outward_normal * -(1.0 - perp.length_squared()).abs().sqrt();
            (perp + para).normalized()
        };

        Scatter {
            direction: Some(direction),
            color: Three::new(1.0, 1.0, 1.0),
        }
    }
}

impl DiffuseLight {
    fn scatter<R: Rng>(&self, _ray: &Ray, _hit: &Hit, _rng: &mut R) -> Scatter {
        Scatter {
            direction: None,
            color: self.rgb * self.power,
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
