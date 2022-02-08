use crate::data::{CanHit, Hit, Ray, Three};

pub struct Plane {
    pub(super) center: Three<f64>,
    pub(super) normal: Three<f64>,
}

impl Plane {
    pub fn new<I: Into<Three<f64>>>(center: I, normal: I) -> Self {
        Self::raw(center.into(), normal.into())
    }

    pub fn raw(center: Three<f64>, normal: Three<f64>) -> Self {
        Self {
            center,
            normal: normal.normalized(),
        }
    }
}

impl CanHit<Plane> for Ray {
    fn shoot_at(&self, plane: &Plane, t_min: f64, t_max: f64) -> Option<Hit> {
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
