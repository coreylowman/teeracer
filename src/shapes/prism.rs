use super::triangle::Triangle;
use crate::data::{CanHit, Hit, Ray, Three};
use num_traits::Float;

#[derive(Debug, Clone)]
pub struct Prism<F> {
    pub(super) triangles: [Triangle<F>; 8],
}

impl<F> Prism<F>
where
    F: Float,
{
    /// Constructs a prism with equilateral triangle ends, and length 1.0 sides facing the positive z axis
    pub fn unit_facing_pos_z() -> Self {
        Self::new(Triangle::facing_pos_z(), F::one())
    }

    /// Constructs a Prism by taking a Triangle and extending `length` along its negative normal.
    pub fn new(tri1: Triangle<F>, length: F) -> Self {
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

        assert!(
            (tri1.normal() + tri2.normal()).length_squared().abs() <= F::from(1e-3f64).unwrap()
        );
        assert!(tri3.normal().dot(&tri4.normal()) > F::zero());
        assert!(tri3.normal().dot(&tri1.normal()) == F::zero());
        assert!(tri5.normal().dot(&tri6.normal()) > F::zero());
        assert!(tri7.normal().dot(&tri8.normal()) > F::zero());

        Self {
            triangles: [tri1, tri2, tri3, tri4, tri5, tri6, tri7, tri8],
        }
    }

    fn center(&self) -> Three<F> {
        let (v0, v1, _v2) = self.triangles[0].vertices();
        let (v3, v4, _v5) = self.triangles[1].vertices();
        (v0 + v1 + v3 + v4) * F::from(0.25f64).unwrap()
    }

    pub fn rotated_around(&self, axis: &Three<F>, angle: F) -> Self {
        let center = self.center();
        let mut t = self.shifted(-center);
        for tri in t.triangles.iter_mut() {
            *tri = tri.rotated_around(axis, angle);
        }
        t.shifted(center)
    }

    pub fn shifted(&self, offset: Three<F>) -> Self {
        let mut t = self.clone();
        for tri in t.triangles.iter_mut() {
            *tri = tri.shifted(offset);
        }
        t
    }
}

impl<F> CanHit<Prism<F>, F> for Ray<F>
where
    F: Float,
{
    fn shoot_at(&self, prism: &Prism<F>, t_min: F, mut t_max: F) -> Option<Hit<F>> {
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
