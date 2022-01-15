use super::Plane;
use crate::data::{CanHit, Hit, Ray, Three};

pub struct Rectangle {
    pub(super) min: Three<f64>,
    pub(super) max: Three<f64>,
    pub(super) plane: Plane,
}

impl Rectangle {
    pub fn new<I: Into<Three<f64>>>(min: I, max: I, normal: I) -> Self {
        let min = min.into();
        let max = max.into();
        Self::raw(min, max, normal.into())
    }

    pub fn raw(min: Three<f64>, max: Three<f64>, normal: Three<f64>) -> Self {
        let center = (max + min) * 0.5;
        Self {
            min,
            max,
            plane: Plane::raw(center, normal.normalized()),
        }
    }
}

impl CanHit for Rectangle {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.plane
            .hit_by(ray, t_min, t_max)
            .filter(|hit| hit.position.between(&self.min, &self.max))
    }
}

impl Three<f64> {
    fn between(&self, min: &Self, max: &Self) -> bool {
        self.x >= min.x
            && self.x <= max.x
            && self.y >= min.y
            && self.y <= max.y
            && self.z >= min.z
            && self.z <= max.z
    }
}
