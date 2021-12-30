use crate::linalg::Vec3;
use crate::ray::{CanHit, Hit, Material, Ray};

pub(crate) struct Plane {
    pub(crate) center: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) material: Material,
}

impl CanHit for Plane {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        let v = self.center - ray.origin;
        let distance = v.dot(&self.normal) / denom;
        if distance < 1e-3 || !distance.is_finite() {
            return None;
        }
        let position = ray.origin + ray.direction * distance;
        Some(Hit {
            position,
            distance,
            normal: self.normal,
            material: self.material,
        })
    }
}
