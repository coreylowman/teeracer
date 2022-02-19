use crate::data::{
    CanHit, Dielectric, Diffuse, Light, LightInteraction, Material, Mirror, Ray, Three,
};
use crate::pdf::{CosineHemisphereDistribution, HemisphereDistribution};
use crate::scene::{Scene, SceneTracer};
use num_traits::{Float, FloatConst, ToPrimitive};
use rand::prelude::*;
use rand_distr::uniform::SampleUniform;
use rand_distr::{Distribution, Standard};
use std::ops::{Mul, MulAssign};

#[derive(Default, Debug, Clone, Copy)]
pub struct PathTracer {
    pub depth: usize,
}

impl<F> SceneTracer<F> for PathTracer
where
    F: Float + SampleUniform + MulAssign + Mul + FloatConst,
    Standard: Distribution<F>,
{
    fn trace<R>(&self, mut ray: Ray<F>, scene: &Scene<F>, rng: &mut R) -> Option<Three<F>>
    where
        R: Rng,
    {
        let t_min = F::from(1e-3f64).unwrap();
        let t_max = F::infinity();

        let mut light_attenuation: Three<F> = Three::ones();
        for _ in 0..self.depth {
            match ray.shoot_at(scene, t_min, t_max) {
                Some(hit) => {
                    let material = scene.material_for(hit.object_index);
                    match material_interaction(material, &ray.direction, &hit.normal, rng) {
                        LightInteraction::Scatter {
                            direction,
                            attenuation,
                        } => {
                            light_attenuation *= attenuation;
                            ray.origin = hit.position;
                            ray.direction = direction;
                        }
                        LightInteraction::Emit { emission } => {
                            return Some(light_attenuation * emission)
                        }
                    }
                }
                None => break,
            }
        }
        None
    }
}

pub(crate) fn material_interaction<F, R>(
    material: &Material<F>,
    in_direction: &Three<F>,
    normal: &Three<F>,
    rng: &mut R,
) -> LightInteraction<F>
where
    R: Rng,
    F: Float + SampleUniform + MulAssign + FloatConst,
    Standard: Distribution<F>,
{
    match material {
        Material::Diffuse(m) => diffuse_interaction(m, normal, rng),
        Material::Mirror(m) => mirror_interaction(m, in_direction, normal),
        Material::Dielectric(m) => dielectric_interaction(m, in_direction, normal, rng),
        Material::Light(m) => light_interaction(m),
    }
}

pub(crate) fn diffuse_interaction<F, R>(
    diffuse: &Diffuse<F>,
    normal: &Three<F>,
    rng: &mut R,
) -> LightInteraction<F>
where
    R: Rng,
    F: Float + SampleUniform + MulAssign + FloatConst,
    Standard: Distribution<F>,
{
    let dist = CosineHemisphereDistribution::oriented_towards(*normal);
    let direction = dist.sample(rng);
    let pdf = dist.pdf(&direction);
    let f = &diffuse.rgb * F::FRAC_1_PI();
    let cos_theta = direction.dot(normal).abs();
    LightInteraction::Scatter {
        attenuation: &f * (cos_theta / pdf),
        direction,
    }
}

pub(crate) fn mirror_interaction<F>(
    mirror: &Mirror<F>,
    in_direction: &Three<F>,
    normal: &Three<F>,
) -> LightInteraction<F>
where
    F: Float + SampleUniform + MulAssign,
{
    // dist = delta distribution(reflected ray)
    // direction = reflected ray
    // pdf = 1
    // f = mirror.rgb / cos_theta
    // cos_theta = ...
    LightInteraction::Scatter {
        attenuation: mirror.rgb,
        direction: reflect(in_direction, normal),
    }
}

pub(crate) fn dielectric_interaction<F, R>(
    dielectric: &Dielectric<F>,
    in_direction: &Three<F>,
    normal: &Three<F>,
    rng: &mut R,
) -> LightInteraction<F>
where
    F: Float + ToPrimitive + SampleUniform,
    Standard: Distribution<F>,
    R: Rng,
{
    let cos_theta = in_direction.dot(normal);
    let exiting = cos_theta > F::zero();
    let outward_normal = &if exiting { -*normal } else { *normal };
    let ratio = if exiting {
        dielectric.ior
    } else {
        dielectric.ior.recip()
    };
    let cos_theta = cos_theta.abs();
    let sin_theta = (F::one() - cos_theta.powi(2)).sqrt();

    let direction = if ratio * sin_theta > F::one() {
        reflect(in_direction, outward_normal)
    } else {
        // shclick approximation
        let r0 = (F::one() - ratio) / (F::one() + ratio);
        let r1 = r0 * r0;
        let reflectance = r1 + (F::one() - r1) * (F::one() - cos_theta).powi(5);

        if reflectance > Standard.sample(rng) {
            reflect(in_direction, outward_normal)
        } else {
            // refract
            let perp = (in_direction + &(outward_normal * cos_theta)) * ratio;
            let para = outward_normal * -(F::one() - perp.length_squared()).abs().sqrt();
            (perp + para).normalized()
        }
    };

    LightInteraction::Scatter {
        attenuation: dielectric.rgb,
        direction,
    }
}

pub(crate) fn light_interaction<F>(diffuse_light: &Light<F>) -> LightInteraction<F>
where
    F: Float,
{
    LightInteraction::Emit {
        emission: diffuse_light.rgb * diffuse_light.power,
    }
}

fn reflect<F>(d: &Three<F>, n: &Three<F>) -> Three<F>
where
    F: Float,
{
    d - &(n * (d.dot(&n) * F::from(2.0f64).unwrap()))
}
