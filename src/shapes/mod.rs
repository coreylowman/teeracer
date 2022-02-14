mod plane;
mod prism;
mod sphere;
mod triangle;

pub use plane::Plane;
pub use prism::Prism;
pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::data::{CanHit, Hit, Ray, Surface, Three};
use num_traits::Float;
use rand::Rng;
use rand_distr::{uniform::SampleUniform, Distribution, Standard};

#[derive(Debug, Clone)]
pub enum Object<F> {
    Plane(Plane<F>),
    Sphere(Sphere<F>),
    Triangle(Triangle<F>),
    Prism(Prism<F>),
}

impl<F> CanHit<Object<F>, F> for Ray<F>
where
    F: Float,
{
    fn shoot_at(&self, obj: &Object<F>, t_min: F, t_max: F) -> Option<Hit<F>> {
        match obj {
            Object::Plane(obj) => self.shoot_at(obj, t_min, t_max),
            Object::Sphere(obj) => self.shoot_at(obj, t_min, t_max),
            Object::Triangle(obj) => self.shoot_at(obj, t_min, t_max),
            Object::Prism(obj) => self.shoot_at(obj, t_min, t_max),
        }
    }
}

impl<F> Surface<F> for Object<F>
where
    F: Float + SampleUniform,
    Standard: Distribution<F>,
{
    fn sample_point_on_surface<R: Rng>(&self, rng: &mut R) -> Three<F> {
        match self {
            Object::Plane(obj) => obj.sample_point_on_surface(rng),
            Object::Sphere(obj) => obj.sample_point_on_surface(rng),
            Object::Triangle(obj) => obj.sample_point_on_surface(rng),
            Object::Prism(obj) => obj.sample_point_on_surface(rng),
        }
    }

    fn normal_at_point(&self, point: &Three<F>) -> Three<F> {
        match self {
            Object::Plane(obj) => obj.normal_at_point(point),
            Object::Sphere(obj) => obj.normal_at_point(point),
            Object::Triangle(obj) => obj.normal_at_point(point),
            Object::Prism(obj) => obj.normal_at_point(point),
        }
    }
}

impl<F> Into<Object<F>> for Plane<F> {
    fn into(self) -> Object<F> {
        Object::Plane(self)
    }
}

impl<F> Into<Object<F>> for Sphere<F> {
    fn into(self) -> Object<F> {
        Object::Sphere(self)
    }
}

impl<F> Into<Object<F>> for Triangle<F> {
    fn into(self) -> Object<F> {
        Object::Triangle(self)
    }
}

impl<F> Into<Object<F>> for Prism<F> {
    fn into(self) -> Object<F> {
        Object::Prism(self)
    }
}
