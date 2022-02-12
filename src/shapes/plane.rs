use crate::data::{CanHit, Hit, Ray, Three};
use num_traits::Float;

pub struct Plane<F> {
    center: Three<F>,
    normal: Three<F>,
}

impl<F> Plane<F>
where
    F: Float,
{
    pub fn facing_pos_x() -> Self {
        Self::new(Three::new(F::one(), F::zero(), F::zero()))
    }

    pub fn facing_neg_x() -> Self {
        Self::new(Three::new(-F::one(), F::zero(), F::zero()))
    }

    pub fn facing_pos_y() -> Self {
        Self::new(Three::new(F::zero(), F::one(), F::zero()))
    }

    pub fn facing_neg_y() -> Self {
        Self::new(Three::new(F::zero(), -F::one(), F::zero()))
    }

    pub fn facing_pos_z() -> Self {
        Self::new(Three::new(F::zero(), F::zero(), F::one()))
    }

    pub fn facing_neg_z() -> Self {
        Self::new(Three::new(F::zero(), F::zero(), -F::one()))
    }

    pub fn new(normal: Three<F>) -> Self {
        Self {
            center: Three::zeros(),
            normal: normal.normalized(),
        }
    }

    pub fn shifted_back(&self, dist: F) -> Self {
        Self {
            center: self.center - self.normal * dist,
            normal: self.normal,
        }
    }
}

impl<F> CanHit<Plane<F>, F> for Ray<F>
where
    F: Float,
{
    fn shoot_at(&self, plane: &Plane<F>, t_min: F, t_max: F) -> Option<Hit<F>> {
        let denom = plane.normal.dot(&self.direction);
        let origin_to_center = &plane.center - &self.origin;
        Some(origin_to_center.dot(&plane.normal) / denom)
            .filter(|&v| v.is_finite() && t_min <= v && v < t_max)
            .map(|distance| {
                let offset = &self.direction * distance;
                let position = &self.origin + &offset;
                Hit {
                    position,
                    distance,
                    normal: plane.normal,
                    object_index: 0,
                }
            })
    }
}
