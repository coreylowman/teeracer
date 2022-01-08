use crate::linalg::{Length, Three};
use crate::ray::{CanHit, Hit, Ray};

pub struct Sphere {
    center: Three<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new<I: Into<Three<f64>>>(into_center: I, radius: f64) -> Self {
        Self {
            center: into_center.into(),
            radius,
        }
    }
}

impl CanHit for Sphere {
    fn hit_by(&self, ray: &Ray) -> Option<Hit> {
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
            }
        })
    }
}
