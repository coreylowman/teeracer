use crate::linalg::{Length, Vec3};
use crate::ray::{CanHit, Hit, Material, Ray};

pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f64,
    pub(crate) material: Material,
}

impl CanHit for Sphere {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut distance = (-half_b - sqrtd) / a;
        if distance < 1e-3 {
            distance = (-half_b + sqrtd) / a;
            if distance < 1e-3 {
                return None;
            }
        }

        let position = ray.origin + ray.direction * distance;
        let normal = (position - self.center).normalized();
        Some(Hit {
            position,
            distance,
            normal,
            material: self.material,
        })
    }
}
