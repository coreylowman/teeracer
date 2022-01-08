use super::triangle::Triangle;
use crate::linalg::Three;
use crate::ray::{CanHit, Hit, Ray};

pub struct Pyramid {
    pub sides: [Triangle; 4],
}

impl Pyramid {
    pub fn new<I: Into<Three<f64>> + Copy>(
        into_root: I,
        into_up: I,
        into_front: I,
        height: f64,
    ) -> Self {
        let root = into_root.into();
        let up = into_up.into().normalized();
        let front = into_front.into().normalized();
        let right = front.rotate(&up, 120.0).normalized();
        let left = front.rotate(&up, 240.0).normalized();
        let v_top = root + up * height;
        let v_front = root + front * height;
        let v_right = root + right * height;
        let v_left = root + left * height;
        let sides = [
            Triangle::from_points(v_right, v_front, v_top),
            Triangle::from_points(v_front, v_left, v_top),
            Triangle::from_points(v_left, v_right, v_top),
            Triangle::from_points(v_front, v_right, v_left),
        ];
        assert!(front.dot(&up) == 0.0);
        // assert!((front.dot(&right) - -0.5).abs() < 1e-6);
        // assert!((front.dot(&left) - -0.5).abs() < 1e-6);
        // assert!((right.dot(&left) - -0.5).abs() < 1e-6);
        assert!(sides[0].normal().dot(&up) > 0.0);
        assert!(sides[1].normal().dot(&up) > 0.0);
        assert!(sides[2].normal().dot(&up) > 0.0);
        assert!(sides[3].normal().dot(&up) == -1.0);
        assert!(sides[0].normal().dot(&sides[1].normal()) < 0.0);
        assert!(sides[2].normal().dot(&sides[1].normal()) < 0.0);
        assert!(sides[0].normal().dot(&sides[2].normal()) < 0.0,);
        Self { sides }
    }
}

impl CanHit for Pyramid {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        self.sides
            .iter()
            .map(|side| side.hit_by(ray))
            .filter(|hit| hit.is_some())
            .min()
            .flatten()
    }
}

impl Three<f64> {
    fn rotate(&self, axis: &Self, angle: f64) -> Self {
        let theta = angle.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        *self * cos_theta
            + (self.cross(axis) * sin_theta + *axis * self.dot(axis) * (1.0 - cos_theta))
    }
}
