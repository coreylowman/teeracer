use crate::linalg::Vec3;
use crate::ray::{Ray, RayTransformer};

pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f64,
    pub(crate) color: Vec3,
}

impl RayTransformer for Sphere {
    fn transform(&self, mut ray: Ray) -> Ray {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return ray;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return ray;
        }

        let t = t0.min(t1);
        if t < ray.length {
            ray.length = t;
            ray.color = self.color;
        }
        return ray;
    }
}
