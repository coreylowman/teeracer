use crate::ray::{CanHit, Hit, Ray};
use crate::three::Three;

pub struct Sphere {
    center: Three<f64>,
    radius_squared: f64,
}

impl Sphere {
    pub fn new<I: Into<Three<f64>>>(center: I, radius: f64) -> Self {
        Self {
            center: center.into(),
            radius_squared: radius.powi(2),
        }
    }
}

impl CanHit for Sphere {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let center_to_origin = &ray.origin - &self.center;
        let a: f64 = 1.0; // ray.direction.length_squared();
        let half_b = center_to_origin.dot(&ray.direction);
        let c = center_to_origin.length_squared() - self.radius_squared;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let near_root = Some((-half_b - sqrtd) * a.recip()).filter(|&v| t_min <= v && v < t_max);
        let far_root = Some((-half_b + sqrtd) * a.recip()).filter(|&v| t_min <= v && v < t_max);
        near_root.or(far_root).map(|distance| {
            let offset = &ray.direction * distance;
            let position = &ray.origin + &offset;
            let normal = (&position - &self.center).normalized();
            Hit {
                position,
                distance,
                normal,
            }
        })
    }
}
