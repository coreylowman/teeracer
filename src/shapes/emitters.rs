use super::{Object, Plane};
use crate::{
    data::{Ray, RayEmitter, Three},
    Prism, Sphere, Triangle,
};
use num_traits::Float;
use rand::prelude::{Rng, SliceRandom};
use rand_distr::{uniform::SampleUniform, Distribution, UnitSphere};
use std::ops::MulAssign;

impl<F> RayEmitter<F> for Object<F>
where
    F: Float + MulAssign + SampleUniform,
{
    fn emit_ray<R>(&self, rng: &mut R) -> Ray<F>
    where
        R: Rng,
    {
        match self {
            Object::Plane(obj) => obj.emit_ray(rng),
            Object::Sphere(obj) => obj.emit_ray(rng),
            Object::Triangle(obj) => obj.emit_ray(rng),
            Object::Prism(obj) => obj.emit_ray(rng),
        }
    }
}

impl<F> RayEmitter<F> for Plane<F>
where
    F: Float + MulAssign + SampleUniform,
{
    fn emit_ray<R>(&self, rng: &mut R) -> Ray<F>
    where
        R: Rng,
    {
        let mut direction = Three::from(UnitSphere.sample(rng));
        direction *= direction.dot(&self.normal).signum(); // NOTE: make noise face in same direction as normal
        Ray {
            origin: self.center,
            direction,
        }
    }
}

impl<F> RayEmitter<F> for Sphere<F>
where
    F: Float + MulAssign + SampleUniform,
{
    fn emit_ray<R>(&self, rng: &mut R) -> Ray<F>
    where
        R: Rng,
    {
        Ray {
            origin: self.center,
            direction: Three::from(UnitSphere.sample(rng)) * self.radius_squared.sqrt(),
        }
    }
}

impl<F> RayEmitter<F> for Triangle<F>
where
    F: Float + MulAssign + SampleUniform,
{
    fn emit_ray<R>(&self, rng: &mut R) -> Ray<F>
    where
        R: Rng,
    {
        let normal = self.normal();
        let mut direction = Three::from(UnitSphere.sample(rng));
        direction *= direction.dot(&normal).signum(); // NOTE: make noise face in same direction as normal
        Ray {
            origin: self.v0,
            direction,
        }
    }
}

impl<F> RayEmitter<F> for Prism<F>
where
    F: Float + MulAssign + SampleUniform,
{
    fn emit_ray<R>(&self, rng: &mut R) -> Ray<F>
    where
        R: Rng,
    {
        self.triangles.choose(rng).unwrap().emit_ray(rng)
    }
}
