use crate::linalg::Vec3;
use crate::ray::{Ray, RayTransformer};

pub(crate) struct Plane {
    pub(crate) center: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) color: Vec3<u8>,
}

impl RayTransformer for Plane {
    fn transform(&self, mut ray: Ray) -> Ray {
        let denom = self.normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.center - ray.origin;
            let distance = v.dot(&self.normal) / denom;
            if distance >= 0.0 && distance < ray.length {
                ray.length = distance;
                ray.color = self.color;
            }
        }
        ray
    }
}
