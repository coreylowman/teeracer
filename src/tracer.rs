use crate::data::{CanHit, Dielectric, DiffuseLight, Hit, Lambertian, Material, Metal, Ray, Three};
use crate::scene::{Scene, SceneTracer};
use num_traits::{Float, ToPrimitive};
use rand::Rng;
use rand_distr::uniform::SampleUniform;
use rand_distr::{Distribution, Standard, UnitBall, UnitSphere};
use std::ops::MulAssign;

#[derive(Default, Debug, Clone, Copy)]
pub struct PathTracer;

impl<F> SceneTracer<F> for PathTracer
where
    F: Float + SampleUniform + MulAssign,
    Standard: Distribution<F>,
{
    fn trace<R>(mut ray: Ray<F>, scene: &Scene<F>, depth: usize, rng: &mut R) -> Option<Three<F>>
    where
        R: Rng,
    {
        let min_t = F::from(1e-3f64).unwrap();
        let max_t = F::infinity();

        let mut color = Three::ones();
        for _ in 0..depth {
            let opt_hit = ray.shoot_at(scene, min_t, max_t);
            if let Some(hit) = opt_hit {
                let material = scene.material_for(hit.object_index);
                let interaction = material.interact(&ray, &hit, rng);
                color *= interaction.attenuation;
                match interaction.light_behavior {
                    LightBehavior::Scatter { direction } => {
                        ray.origin = hit.position;
                        ray.direction = direction;
                    }
                    LightBehavior::Absorb => return Some(color),
                }
            } else {
                return None;
            }
        }
        None
    }
}

struct MaterialInteraction<F> {
    attenuation: Three<F>,
    light_behavior: LightBehavior<F>,
}

enum LightBehavior<F> {
    Scatter { direction: Three<F> },
    Absorb,
}

impl<F> Material<F>
where
    F: Float + SampleUniform + MulAssign,
    Standard: Distribution<F>,
{
    fn interact<R: Rng>(&self, ray: &Ray<F>, hit: &Hit<F>, rng: &mut R) -> MaterialInteraction<F> {
        match self {
            Material::Lambertian(material) => material.interact(hit, rng),
            Material::Metal(material) => material.interact(ray, hit, rng),
            Material::Dielectric(material) => material.interact(ray, hit, rng),
            Material::DiffuseLight(material) => material.interact(),
        }
    }
}

impl<F> Lambertian<F>
where
    F: Float + SampleUniform + MulAssign,
{
    fn interact<R: Rng>(&self, hit: &Hit<F>, rng: &mut R) -> MaterialInteraction<F> {
        let mut noise = Three::from(UnitSphere.sample(rng));
        noise *= noise.dot(&hit.normal).signum(); // NOTE: make noise face in same direction as normal
        let direction = (&hit.normal + &noise).normalized();
        let cos_theta = direction.dot(&hit.normal).max(F::zero());
        MaterialInteraction {
            attenuation: &self.rgb * cos_theta,
            light_behavior: LightBehavior::Scatter { direction },
        }
    }
}

impl<F> Metal<F>
where
    F: Float + SampleUniform + MulAssign,
{
    fn interact<R: Rng>(&self, ray: &Ray<F>, hit: &Hit<F>, rng: &mut R) -> MaterialInteraction<F> {
        let mut direction = reflect(&ray.direction, &hit.normal);
        if let Some(value) = self.fuzz {
            let mut noise = Three::from(UnitBall.sample(rng));
            noise *= noise.dot(&hit.normal).signum() * value;
            direction = (&direction + &noise).normalized();
        }
        MaterialInteraction {
            attenuation: self.rgb,
            light_behavior: LightBehavior::Scatter { direction },
        }
    }
}

impl<F> Dielectric<F>
where
    F: Float + ToPrimitive + SampleUniform,
    Standard: Distribution<F>,
{
    fn interact<R: Rng>(&self, ray: &Ray<F>, hit: &Hit<F>, rng: &mut R) -> MaterialInteraction<F> {
        let cos_theta = ray.direction.dot(&hit.normal);
        let exiting = cos_theta > F::zero();
        let outward_normal = if exiting { -hit.normal } else { hit.normal };
        let ratio = if exiting { self.ior } else { self.ior.recip() };
        let cos_theta = cos_theta.abs();
        let sin_theta = (F::one() - cos_theta.powi(2)).sqrt();

        // shclick approximation
        let r0 = (F::one() - ratio) / (F::one() + ratio);
        let r1 = r0 * r0;
        let reflectance = r1 + (F::one() - r1) * (F::one() - cos_theta).powi(5);

        let direction = if ratio * sin_theta > F::one() || reflectance > Standard.sample(rng) {
            reflect(&ray.direction, &outward_normal)
        } else {
            let perp = (&ray.direction + &(&outward_normal * cos_theta)) * ratio;
            let para = &outward_normal * -(F::one() - perp.length_squared()).abs().sqrt();
            (perp + para).normalized()
        };

        MaterialInteraction {
            attenuation: Three::ones(),
            light_behavior: LightBehavior::Scatter { direction },
        }
    }
}

impl<F> DiffuseLight<F>
where
    F: Float,
{
    fn interact(&self) -> MaterialInteraction<F> {
        MaterialInteraction {
            attenuation: self.rgb * self.power,
            light_behavior: LightBehavior::Absorb,
        }
    }
}

fn reflect<F>(d: &Three<F>, n: &Three<F>) -> Three<F>
where
    F: Float,
{
    d - &(n * (d.dot(&n) * F::from(2.0f64).unwrap()))
}
