use crate::data::{CanHit, Dielectric, Diffuse, Hit, Light, Material, Mirror, Ray, Three};
use crate::pdf::{CosinePDF, PDF};
use crate::scene::{Scene, SceneTracer};
use num_traits::{Float, FloatConst, ToPrimitive};
use rand::prelude::*;
use rand_distr::uniform::SampleUniform;
use rand_distr::{Distribution, Standard};
use std::ops::MulAssign;

#[derive(Default, Debug, Clone, Copy)]
pub struct PathTracer;

impl<F> SceneTracer<F> for PathTracer
where
    F: Float + SampleUniform + MulAssign + FloatConst,
    Standard: Distribution<F>,
{
    fn trace<R>(mut ray: Ray<F>, scene: &Scene<F>, depth: usize, rng: &mut R) -> Option<Three<F>>
    where
        R: Rng,
    {
        let t_min = F::from(1e-3f64).unwrap();
        let t_max = F::infinity();

        let mut color = Three::ones();
        for _ in 0..depth {
            match ray.shoot_at(scene, t_min, t_max) {
                Some(hit) => {
                    let material = scene.material_for(hit.object_index);
                    let interaction = material_interaction(material, &ray, &hit, rng);
                    color *= interaction.attenuation;
                    match interaction.light_behavior {
                        LightBehavior::Scatter { direction } => {
                            ray.origin = hit.position;
                            ray.direction = direction;
                        }
                        LightBehavior::Absorb => return Some(color),
                    }
                }
                None => break,
            }
        }
        None
    }
}

pub(crate) struct MaterialInteraction<F> {
    attenuation: Three<F>,
    light_behavior: LightBehavior<F>,
}

#[derive(PartialEq, Eq)]
pub(crate) enum LightBehavior<F> {
    Scatter { direction: Three<F> },
    Absorb,
}

pub(crate) fn material_interaction<F, R>(
    material: &Material<F>,
    ray: &Ray<F>,
    hit: &Hit<F>,
    rng: &mut R,
) -> MaterialInteraction<F>
where
    R: Rng,
    F: Float + SampleUniform + MulAssign + FloatConst,
    Standard: Distribution<F>,
{
    match material {
        Material::Diffuse(m) => diffuse_interaction(m, hit, rng),
        Material::Mirror(m) => mirror_interaction(m, ray, hit),
        Material::Dielectric(m) => dielectric_interaction(m, ray, hit, rng),
        Material::Light(m) => light_interaction(m),
    }
}

pub(crate) fn diffuse_interaction<F, R>(
    diffuse: &Diffuse<F>,
    hit: &Hit<F>,
    rng: &mut R,
) -> MaterialInteraction<F>
where
    R: Rng,
    F: Float + SampleUniform + MulAssign + FloatConst,
    Standard: Distribution<F>,
{
    let pdf = CosinePDF::oriented_towards(hit.normal);
    let scatter_direction = pdf.sample(rng);
    let density = pdf.value(&scatter_direction);
    let lambertian_bsdf = (scatter_direction.dot(&hit.normal) * F::FRAC_1_PI()).abs();
    MaterialInteraction {
        attenuation: &diffuse.rgb * lambertian_bsdf * density.recip(),
        light_behavior: LightBehavior::Scatter {
            direction: scatter_direction,
        },
    }
}

pub(crate) fn mirror_interaction<F>(
    metal: &Mirror<F>,
    ray: &Ray<F>,
    hit: &Hit<F>,
) -> MaterialInteraction<F>
where
    F: Float + SampleUniform + MulAssign,
{
    MaterialInteraction {
        attenuation: metal.rgb,
        light_behavior: LightBehavior::Scatter {
            direction: reflect(&ray.direction, &hit.normal),
        },
    }
}

pub(crate) fn dielectric_interaction<F, R>(
    dielectric: &Dielectric<F>,
    ray: &Ray<F>,
    hit: &Hit<F>,
    rng: &mut R,
) -> MaterialInteraction<F>
where
    F: Float + ToPrimitive + SampleUniform,
    Standard: Distribution<F>,
    R: Rng,
{
    let cos_theta = ray.direction.dot(&hit.normal);
    let exiting = cos_theta > F::zero();
    let outward_normal = if exiting { -hit.normal } else { hit.normal };
    let ratio = if exiting {
        dielectric.ior
    } else {
        dielectric.ior.recip()
    };
    let cos_theta = cos_theta.abs();
    let sin_theta = (F::one() - cos_theta.powi(2)).sqrt();

    let direction = if ratio * sin_theta > F::one() {
        reflect(&ray.direction, &outward_normal)
    } else {
        // shclick approximation
        let r0 = (F::one() - ratio) / (F::one() + ratio);
        let r1 = r0 * r0;
        let reflectance = r1 + (F::one() - r1) * (F::one() - cos_theta).powi(5);

        if reflectance > Standard.sample(rng) {
            reflect(&ray.direction, &outward_normal)
        } else {
            // refract
            let perp = (&ray.direction + &(&outward_normal * cos_theta)) * ratio;
            let para = &outward_normal * -(F::one() - perp.length_squared()).abs().sqrt();
            (perp + para).normalized()
        }
    };

    MaterialInteraction {
        attenuation: Three::ones(),
        light_behavior: LightBehavior::Scatter { direction },
    }
}

pub(crate) fn light_interaction<F>(diffuse_light: &Light<F>) -> MaterialInteraction<F>
where
    F: Float,
{
    MaterialInteraction {
        attenuation: diffuse_light.rgb * diffuse_light.power,
        light_behavior: LightBehavior::Absorb,
    }
}

fn reflect<F>(d: &Three<F>, n: &Three<F>) -> Three<F>
where
    F: Float,
{
    d - &(n * (d.dot(&n) * F::from(2.0f64).unwrap()))
}
