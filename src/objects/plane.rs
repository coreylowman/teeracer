use crate::linalg::Three;
use crate::ray::{CanHit, Hit, Material, Ray};

pub(crate) struct Plane {
    pub(crate) center: Three<f64>,
    pub(crate) normal: Three<f64>,
    pub(crate) material: Material,
}

impl CanHit for Plane {
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
                    normal: self.normal,
                    material: self.material,
                }
            })
    }
}
