use crate::linalg::{Length, Three};
use crate::ray::{CanHit, Hit, Material, Ray};

pub(crate) struct Sphere {
    pub(crate) center: Three<f64>,
    pub(crate) radius: f64,
    pub(crate) material: Material,
}

impl CanHit for Sphere {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let center_to_origin = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = center_to_origin.dot(&ray.direction);
        let c = center_to_origin.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let near_root = Some((-half_b - sqrtd) / a).filter(|&v| v >= 1e-3);
        let far_root = Some((-half_b + sqrtd) / a).filter(|&v| v >= 1e-3);
        near_root.or(far_root).map(|distance| {
            let position = ray.origin + ray.direction * distance;
            let normal = (position - self.center).normalized();
            Hit {
                position,
                distance,
                normal,
                material: self.material,
            }
        })
    }
}
