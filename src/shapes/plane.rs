use crate::data::{CanHit, Hit, Ray, Three};

pub struct Plane {
    center: Three<f64>,
    normal: Three<f64>,
}

impl Plane {
    pub fn facing_pos_x() -> Self {
        Self::new(Three::new(1.0, 0.0, 0.0))
    }

    pub fn facing_neg_x() -> Self {
        Self::new(Three::new(-1.0, 0.0, 0.0))
    }

    pub fn facing_pos_y() -> Self {
        Self::new(Three::new(0.0, 1.0, 0.0))
    }

    pub fn facing_neg_y() -> Self {
        Self::new(Three::new(0.0, -1.0, 0.0))
    }

    pub fn facing_pos_z() -> Self {
        Self::new(Three::new(0.0, 0.0, 1.0))
    }

    pub fn facing_neg_z() -> Self {
        Self::new(Three::new(0.0, 0.0, -1.0))
    }

    pub fn new(normal: Three<f64>) -> Self {
        Self {
            center: Three::new(0.0, 0.0, 0.0),
            normal: normal.normalized(),
        }
    }

    pub fn shifted_back(&self, dist: f64) -> Self {
        Self {
            center: self.center - self.normal * dist,
            normal: self.normal,
        }
    }
}

impl CanHit<Plane> for Ray {
    fn shoot_at(&self, plane: &Plane, t_min: f64, t_max: f64) -> Option<Hit> {
        let denom = plane.normal.dot(&self.direction);
        let origin_to_center = &plane.center - &self.origin;
        Some(origin_to_center.dot(&plane.normal) / denom)
            .filter(|&v| v.is_finite() && t_min <= v && v < t_max)
            .map(|distance| {
                let offset = &self.direction * distance;
                let position = &self.origin + &offset;
                Hit {
                    position,
                    distance,
                    normal: plane.normal,
                    object_index: 0,
                }
            })
    }
}
