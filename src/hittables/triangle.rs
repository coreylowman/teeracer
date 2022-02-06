use crate::data::{CanHit, Hit, Ray, Three};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub(super) v0: Three<f64>,
    pub(super) v01: Three<f64>,
    pub(super) v02: Three<f64>,
}

impl Triangle {
    pub fn new<I: Into<Three<f64>>>(into_v0: I, into_v1: I, into_v2: I) -> Self {
        let v0 = into_v0.into();
        let v1 = into_v1.into();
        let v2 = into_v2.into();
        Self::from_points(v0, v1, v2)
    }

    pub fn from_points(v0: Three<f64>, v1: Three<f64>, v2: Three<f64>) -> Self {
        Self {
            v0,
            v01: v1 - v0,
            v02: v2 - v0,
        }
    }

    pub fn normal(&self) -> Three<f64> {
        self.v01.cross(&self.v02).normalized()
    }

    pub fn rotated_around(&self, origin: &Three<f64>, axis: &Three<f64>, angle: f64) -> Self {
        let v1 = self.v01 + self.v0;
        let v2 = self.v02 + self.v0;
        let v0 = (self.v0 - origin).rotate(axis, angle) + origin;
        let v1 = (v1 - origin).rotate(axis, angle) + origin;
        let v2 = (v2 - origin).rotate(axis, angle) + origin;
        Self::from_points(v0, v1, v2)
    }
}

impl CanHit for Triangle {
    // source: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let pvec = ray.direction.cross(&self.v02);

        let determinant = self.v01.dot(&pvec);
        if determinant.abs() < 1e-3 {
            // ray and triangle are parallel
            return None;
        }

        let tvec = ray.origin - self.v0;
        let u = tvec.dot(&pvec) / determinant;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&self.v01);
        let v = ray.direction.dot(&qvec) / determinant;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let distance = self.v02.dot(&qvec) / determinant;
        if distance < t_min || distance >= t_max {
            return None;
        }
        let position = ray.origin + ray.direction * distance;
        let normal = self.normal();
        Some(Hit {
            position,
            distance,
            normal,
            sub_object_index: None,
        })
    }
}
