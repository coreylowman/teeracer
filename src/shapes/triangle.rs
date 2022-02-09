use crate::data::{CanHit, Hit, Ray, Three};

#[derive(Debug, Clone)]
pub struct Triangle {
    v0: Three<f64>,
    v01: Three<f64>,
    v02: Three<f64>,
}

impl Triangle {
    pub fn new(v0: Three<f64>, v1: Three<f64>, v2: Three<f64>) -> Self {
        Self {
            v0,
            v01: v1 - v0,
            v02: v2 - v0,
        }
    }

    /// Constructs an equialateral triangle around the origin with the normal facing the positive z axis
    /// NOTE: The center of the bottom side of the triangle is the origin.
    pub fn facing_pos_z() -> Self {
        Self::new(
            Three::new(-0.5, 0.0, 0.0),
            Three::new(0.5, 0.0, 0.0),
            Three::new(0.0, 3.0f64.sqrt() / 2.0, 0.0),
        )
    }

    pub fn shifted(&self, offset: Three<f64>) -> Self {
        Self {
            v0: self.v0 + offset,
            v01: self.v01,
            v02: self.v02,
        }
    }

    pub fn scaled(&self, scalar: f64) -> Self {
        let (v0, v1, v2) = self.vertices();
        Self::new(v0 * scalar, v1 * scalar, v2 * scalar)
    }

    pub fn rotated_around(&self, axis: &Three<f64>, angle: f64) -> Self {
        let (v0, v1, v2) = self.vertices();
        let v0 = v0.rotate(axis, angle);
        let v1 = v1.rotate(axis, angle);
        let v2 = v2.rotate(axis, angle);
        Self::new(v0, v1, v2)
    }
}

impl Triangle {
    pub fn vertices(&self) -> (Three<f64>, Three<f64>, Three<f64>) {
        (self.v0, self.v01 + self.v0, self.v02 + self.v0)
    }

    pub fn normal(&self) -> Three<f64> {
        self.v01.cross(&self.v02).normalized()
    }
}

impl CanHit<Triangle> for Ray {
    // source: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
    fn shoot_at(&self, triangle: &Triangle, t_min: f64, t_max: f64) -> Option<Hit> {
        let pvec = self.direction.cross(&triangle.v02);

        let determinant = triangle.v01.dot(&pvec);
        if determinant.abs() < 1e-3 {
            // ray and triangle are parallel
            return None;
        }

        let tvec = self.origin - triangle.v0;
        let u = tvec.dot(&pvec) / determinant;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&triangle.v01);
        let v = self.direction.dot(&qvec) / determinant;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let distance = triangle.v02.dot(&qvec) / determinant;
        if distance < t_min || distance >= t_max {
            return None;
        }
        let position = self.origin + self.direction * distance;
        let normal = triangle.normal();
        Some(Hit {
            position,
            distance,
            normal,
            object_index: 0,
        })
    }
}
