use super::triangle::Triangle;
use crate::data::{CanHit, Hit, Ray, Three};

#[derive(Debug, Clone)]
pub struct Prism {
    triangles: [Triangle; 8],
}

impl Prism {
    /// Constructs a prism with equilateral triangle ends, and length 1.0 sides facing the positive z axis
    pub fn unit_facing_pos_z() -> Self {
        Self::new(Triangle::facing_pos_z(), 1.0)
    }

    /// Constructs a Prism by taking a Triangle and extending `length` along its negative normal.
    pub fn new(tri1: Triangle, length: f64) -> Self {
        let (v0, v1, v2) = tri1.vertices();
        let normal = tri1.normal();
        let v3 = v0 - normal * length;
        let v4 = v1 - normal * length;
        let v5 = v2 - normal * length;

        let tri2 = Triangle::new(v3, v5, v4);
        let tri3 = Triangle::new(v0, v3, v1);
        let tri4 = Triangle::new(v4, v1, v3);
        let tri5 = Triangle::new(v3, v0, v5);
        let tri6 = Triangle::new(v2, v5, v0);
        let tri7 = Triangle::new(v2, v1, v4);
        let tri8 = Triangle::new(v2, v4, v5);

        assert!((tri1.normal() + tri2.normal()).length_squared().abs() <= 1e-3);
        assert!(tri3.normal().dot(&tri4.normal()) > 0.0);
        assert!(tri3.normal().dot(&tri1.normal()) == 0.0);
        assert!(tri5.normal().dot(&tri6.normal()) > 0.0);
        assert!(tri7.normal().dot(&tri8.normal()) > 0.0);

        Self {
            triangles: [tri1, tri2, tri3, tri4, tri5, tri6, tri7, tri8],
        }
    }

    fn center(&self) -> Three<f64> {
        let (v0, v1, _v2) = self.triangles[0].vertices();
        let (v3, v4, _v5) = self.triangles[1].vertices();
        (v0 + v1 + v3 + v4) * 0.25
    }

    pub fn rotated_around(&self, axis: &Three<f64>, angle: f64) -> Self {
        let center = self.center();
        let mut t = self.shifted(-center);
        for tri in t.triangles.iter_mut() {
            *tri = tri.rotated_around(axis, angle);
        }
        t.shifted(center)
    }

    pub fn shifted(&self, offset: Three<f64>) -> Self {
        let mut t = self.clone();
        for tri in t.triangles.iter_mut() {
            *tri = tri.shifted(offset);
        }
        t
    }
}

impl CanHit<Prism> for Ray {
    fn shoot_at(&self, prism: &Prism, t_min: f64, mut t_max: f64) -> Option<Hit> {
        let mut opt_hit = None;
        for (i, obj) in prism.triangles.iter().enumerate() {
            if let Some(mut hit) = self.shoot_at(obj, t_min, t_max) {
                if hit.distance < t_max {
                    hit.object_index = i;
                    opt_hit = Some(hit);
                    t_max = hit.distance;
                }
            }
        }
        opt_hit
    }
}
