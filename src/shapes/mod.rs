mod plane;
mod prism;
mod sphere;
mod triangle;

pub use plane::Plane;
pub use prism::Prism;
pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::data::{CanHit, Hit, Ray};
use num_traits::Float;

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
