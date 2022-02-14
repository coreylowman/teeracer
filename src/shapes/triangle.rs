use crate::data::{CanHit, Hit, Ray, Surface, Three};
use num_traits::Float;
use rand::Rng;
use rand_distr::{uniform::SampleUniform, Distribution, Standard};

#[derive(Debug, Clone)]
pub struct Triangle<F> {
    pub(super) v0: Three<F>,
    pub(super) v01: Three<F>,
    pub(super) v02: Three<F>,
}

impl<F> Triangle<F>
where
    F: Float,
{
    pub fn new(v0: Three<F>, v1: Three<F>, v2: Three<F>) -> Self {
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
            Three::new(F::from(-0.5f64).unwrap(), F::zero(), F::zero()),
            Three::new(F::from(0.5f64).unwrap(), F::zero(), F::zero()),
            Three::new(
                F::zero(),
                F::from(3.0f64.sqrt() / 2.0f64).unwrap(),
                F::zero(),
            ),
        )
    }

    pub fn shifted(&self, offset: Three<F>) -> Self {
        Self {
            v0: self.v0 + offset,
            v01: self.v01,
            v02: self.v02,
        }
    }

    pub fn scaled(&self, scalar: F) -> Self {
        let (v0, v1, v2) = self.vertices();
        Self::new(v0 * scalar, v1 * scalar, v2 * scalar)
    }

    pub fn rotated_around(&self, axis: &Three<F>, angle: F) -> Self {
        let (v0, v1, v2) = self.vertices();
        let v0 = v0.rotate(axis, angle);
        let v1 = v1.rotate(axis, angle);
        let v2 = v2.rotate(axis, angle);
        Self::new(v0, v1, v2)
    }

    pub fn vertices(&self) -> (Three<F>, Three<F>, Three<F>) {
        (self.v0, self.v01 + self.v0, self.v02 + self.v0)
    }

    pub fn normal(&self) -> Three<F> {
        self.v01.cross(&self.v02).normalized()
    }
}

impl<F> CanHit<Triangle<F>, F> for Ray<F>
where
    F: Float,
{
    // source: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
    fn shoot_at(&self, triangle: &Triangle<F>, t_min: F, t_max: F) -> Option<Hit<F>> {
        let pvec = self.direction.cross(&triangle.v02);

        let determinant = triangle.v01.dot(&pvec);
        if determinant.abs() < F::from(1e-3f64).unwrap() {
            // ray and triangle are parallel
            return None;
        }

        let tvec = self.origin - triangle.v0;
        let u = tvec.dot(&pvec) / determinant;
        if u < F::zero() || u > F::one() {
            return None;
        }

        let qvec = tvec.cross(&triangle.v01);
        let v = self.direction.dot(&qvec) / determinant;
        if v < F::zero() || u + v > F::one() {
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

impl<F> Surface<F> for Triangle<F>
where
    F: Float + SampleUniform,
    Standard: Distribution<F>,
{
    fn sample_point_on_surface<R: Rng>(&self, rng: &mut R) -> Three<F> {
        let (v0, v1, v2) = self.vertices();

        let t1: F = Standard.sample(rng);
        let p1 = v0 * (F::one() - t1) + v1 * t1;
        let t2: F = Standard.sample(rng);
        let p2 = v0 * (F::one() - t2) + v2 * t2;
        let t3: F = Standard.sample(rng);
        let p3 = v1 * (F::one() - t3) * v2 * t3;

        (p1 + p2 + p3) / F::from(3.0f64).unwrap()
    }

    fn normal_at_point(&self, _point: &Three<F>) -> Three<F> {
        self.normal()
    }
}
