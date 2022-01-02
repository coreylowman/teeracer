use crate::linalg::Three;
use crate::ray::{CanHit, Hit, Ray};

pub struct Rectangle {
    min: Three<f64>,
    max: Three<f64>,
    center: Three<f64>,
    normal: Three<f64>,
}

impl Rectangle {
    pub fn new<I: Into<Three<f64>>>(into_min: I, into_max: I, into_normal: I) -> Self {
        let min = into_min.into();
        let max = into_max.into();
        let center = (max + min) * 0.5;
        Self {
            min,
            max,
            center,
            normal: into_normal.into(),
        }
    }
}

impl CanHit for Rectangle {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        let origin_to_center = self.center - ray.origin;
        Some(origin_to_center.dot(&self.normal) / denom)
            .filter(|&v| v.is_finite() && v >= 1e-3)
            .map(|distance| {
                let position = ray.origin + ray.direction * distance;
                Hit {
                    position,
                    distance,
                    normal: self.normal * -denom.signum(),
                }
            })
            .filter(|hit| hit.position.between(&self.min, &self.max))
    }
}

impl Three<f64> {
    fn between(&self, min: &Self, max: &Self) -> bool {
        (0..3).all(|i| self[i] - min[i] >= -1e-3 && self[i] - max[i] <= 1e-3)
    }
}
