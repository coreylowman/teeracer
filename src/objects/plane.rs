use crate::linalg::Vec3;
use crate::ray::{CanIntersect, Intersection, Material, Ray};

pub(crate) struct Plane {
    pub(crate) center: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) material: Material,
}

impl CanIntersect for Plane {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denom = self.normal.dot(&ray.direction);
        let v = self.center - ray.origin;
        let distance = v.dot(&self.normal) / denom;
        if denom > 1e-6 && distance >= 0.0 {
            Some(Intersection {
                distance,
                result: Ray {
                    origin: ray.origin + ray.direction * distance,
                    direction: ray.direction, // TODO reflect around normal
                },
                material: self.material,
            })
        } else {
            None
        }
    }
}
