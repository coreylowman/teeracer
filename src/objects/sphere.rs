use crate::linalg::Vec3;
use crate::ray::{CanHit, Hit, Material, Ray};

pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f64,
    pub(crate) material: Material,
}

impl CanHit for Sphere {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 1e-3 && t1 < 1e-3 {
            return None;
        }

        let distance = t0.min(t1);
        let position = ray.origin + ray.direction * distance;
        let outward_normal = (position - self.center) / self.radius;
        let normal = if outward_normal.dot(&ray.direction) < 0.0 {
            outward_normal
        } else {
            -outward_normal
        };
        Some(Hit {
            position,
            distance,
            normal,
            material: self.material,
        })
    }
}
