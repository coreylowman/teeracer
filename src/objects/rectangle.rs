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
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        let origin_to_center = self.center - ray.origin;
        Some(origin_to_center.dot(&self.normal) / denom)
            .filter(|&v| v.is_finite() && t_min <= v && v < t_max)
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
        self.x - min.x >= -1e-3
            && self.x - max.x <= 1e-3
            && self.y - min.y >= -1e-3
            && self.y - max.y <= 1e-3
            && self.z - min.z >= -1e-3
            && self.z - max.z <= 1e-3
    }
}
