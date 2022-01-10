mod plane;
mod pyramid;
mod rectangle;
mod sphere;
mod triangle;

pub use plane::Plane;
pub use pyramid::Pyramid;
pub use rectangle::Rectangle;
pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::ray::{CanHit, Hit, Ray};

pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
}

impl CanHit for Object {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Object::Plane(obj) => obj.hit_by(ray, t_min, t_max),
            Object::Sphere(obj) => obj.hit_by(ray, t_min, t_max),
        }
    }
}

impl Into<Object> for Plane {
    fn into(self) -> Object {
        Object::Plane(self)
    }
}

impl Into<Object> for Sphere {
    fn into(self) -> Object {
        Object::Sphere(self)
    }
}
