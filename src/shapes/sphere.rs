use crate::data::{CanHit, Hit, Ray, Three};

pub struct Sphere {
    center: Three<f64>,
    radius_squared: f64,
}

impl Sphere {
    pub fn unit_at(x: f64, y: f64, z: f64) -> Self {
        Self {
            center: Three::new(x, y, z),
            radius_squared: 1.0,
        }
    }

    pub fn scaled(&self, scalar: f64) -> Self {
        Self {
            center: self.center,
            radius_squared: self.radius_squared * scalar.powi(2),
        }
    }
}

impl CanHit<Sphere> for Ray {
    fn shoot_at(&self, sphere: &Sphere, t_min: f64, t_max: f64) -> Option<Hit> {
        let center_to_origin = &self.origin - &sphere.center;
        let a: f64 = 1.0; // self.direction.length_squared();
        let half_b = center_to_origin.dot(&self.direction);
        let c = center_to_origin.length_squared() - sphere.radius_squared;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let near_root = Some((-half_b - sqrtd) * a.recip()).filter(|&v| t_min <= v && v < t_max);
        let far_root = Some((-half_b + sqrtd) * a.recip()).filter(|&v| t_min <= v && v < t_max);
        near_root.or(far_root).map(|distance| {
            let offset = &self.direction * distance;
            let position = &self.origin + &offset;
            let normal = (&position - &sphere.center).normalized();
            Hit {
                position,
                distance,
                normal,
                object_index: 0,
            }
        })
    }
}
