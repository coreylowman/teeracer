use crate::linalg::Three;
use crate::ray::{CanHit, Hit, Ray};

pub struct Plane {
    center: Three<f64>,
    normal: Three<f64>,
}

impl Plane {
    pub fn new<I: Into<Three<f64>>>(into_center: I, into_normal: I) -> Self {
        Self::raw(into_center.into(), into_normal.into())
    }

    pub fn raw(center: Three<f64>, normal: Three<f64>) -> Self {
        Self { center, normal }
    }
}

impl CanHit for Plane {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        let origin_to_center = &self.center - &ray.origin;
        Some(origin_to_center.dot(&self.normal) / denom)
            .filter(|&v| v.is_finite() && t_min <= v && v < t_max)
            .map(|distance| {
                let offset = &ray.direction * distance;
                let position = &ray.origin + &offset;
                Hit {
                    position,
                    distance,
                    normal: self.normal,
                }
            })
    }
}
