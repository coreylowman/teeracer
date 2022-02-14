use crate::data::{CanHit, Hit, Ray, Surface, Three};
use num_traits::Float;
use rand::Rng;
use rand_distr::{uniform::SampleUniform, Distribution, UnitSphere};

#[derive(Debug, Clone)]
pub struct Sphere<F> {
    pub(super) center: Three<F>,
    pub(super) radius_squared: F,
}

impl<F> Sphere<F>
where
    F: Float,
{
    pub fn unit_at(x: F, y: F, z: F) -> Self {
        Self {
            center: Three::new(x, y, z),
            radius_squared: F::one(),
        }
    }

    pub fn scaled(&self, scalar: F) -> Self {
        Self {
            center: self.center,
            radius_squared: self.radius_squared * scalar.powi(2),
        }
    }
}

impl<F> CanHit<Sphere<F>, F> for Ray<F>
where
    F: Float,
{
    fn shoot_at(&self, sphere: &Sphere<F>, t_min: F, t_max: F) -> Option<Hit<F>> {
        let center_to_origin = &self.origin - &sphere.center;
        let a = F::one(); // self.direction.length_squared();
        let half_b = center_to_origin.dot(&self.direction);
        let c = center_to_origin.length_squared() - sphere.radius_squared;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < F::zero() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let near_root = Some((-half_b - sqrtd) * a.recip()).filter(|&v| t_min <= v && v < t_max);
        let far_root = Some((-half_b + sqrtd) * a.recip()).filter(|&v| t_min <= v && v < t_max);
        near_root.or(far_root).map(|distance| {
            let offset = &self.direction * distance;
            let position = &self.origin + &offset;
            let normal = (&position - &sphere.center).normalized();
            Hit {
                position,
                distance,
                normal,
                object_index: 0,
            }
        })
    }
}

impl<F> Surface<F> for Sphere<F>
where
    F: Float + SampleUniform,
{
    fn sample_point_on_surface<R: Rng>(&self, rng: &mut R) -> Three<F> {
        let distance = self.radius_squared.sqrt();
        let direction = Three::from(UnitSphere.sample(rng));
        &self.center + &(&direction * distance)
    }

    fn normal_at_point(&self, point: &Three<F>) -> Three<F> {
        (point - &self.center).normalized()
    }
}
