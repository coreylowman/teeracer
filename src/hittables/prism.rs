use super::triangle::Triangle;
use crate::data::{CanHit, Hit, Ray, Three};

#[derive(Debug, Clone)]
pub struct Prism {
    triangles: [Triangle; 8],
}

impl Prism {
    pub fn new<I: Into<Three<f64>> + Copy>(v0: I, v1: I, v2: I, length: f64) -> Self {
        let v0: Three<f64> = v0.into();
        let v1: Three<f64> = v1.into();
        let v2: Three<f64> = v2.into();
        let tri1 = Triangle::from_points(v0, v1, v2);
        let normal = tri1.normal();

        let v3 = v0 - normal * length;
        let v4 = v1 - normal * length;
        let v5 = v2 - normal * length;
        let tri2 = Triangle::from_points(v3, v5, v4);
        assert!((tri1.normal() + tri2.normal()).length_squared().abs() <= 1e-3);

        // bottom
        let tri3 = Triangle::from_points(v0, v3, v1);
        let tri4 = Triangle::from_points(v4, v1, v3);
        assert!(tri3.normal().dot(&tri4.normal()) > 0.0);
        assert!(tri3.normal().dot(&tri1.normal()) == 0.0);

        let tri5 = Triangle::from_points(v3, v0, v5);
        let tri6 = Triangle::from_points(v2, v5, v0);
        assert!(tri5.normal().dot(&tri6.normal()) > 0.0);

        let tri7 = Triangle::from_points(v2, v1, v4);
        let tri8 = Triangle::from_points(v2, v4, v5);
        assert!(tri7.normal().dot(&tri8.normal()) > 0.0);

        Self {
            triangles: [tri1, tri2, tri3, tri4, tri5, tri6, tri7, tri8],
        }
    }

    pub fn rotated(&self, axis: &Three<f64>, angle: f64) -> Self {
        let origin = (self.triangles[0].v0 * 2.0
            + self.triangles[0].v01
            + self.triangles[1].v0 * 2.0
            + self.triangles[1].v01)
            * 0.25;

        let mut t = self.clone();
        for side in t.triangles.iter_mut() {
            *side = side.rotated_around(&origin, axis, angle);
        }
        t
    }
}

impl CanHit for Prism {
    fn hit_by(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<Hit> {
        let mut opt_hit = None;
        for (i, obj) in self.triangles.iter().enumerate() {
            if let Some(mut hit) = obj.hit_by(&ray, t_min, t_max) {
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
