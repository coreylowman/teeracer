mod plane;
mod prism;
mod rectangle;
mod sphere;
mod triangle;

pub use plane::Plane;
pub use prism::Prism;
pub use rectangle::Rectangle;
pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::data::{CanHit, Hit, Ray};

pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
    Rectangle(Rectangle),
    Triangle(Triangle),
    Prism(Prism),
}

impl CanHit for Object {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Object::Plane(obj) => obj.hit_by(ray, t_min, t_max),
            Object::Sphere(obj) => obj.hit_by(ray, t_min, t_max),
            Object::Rectangle(obj) => obj.hit_by(ray, t_min, t_max),
            Object::Triangle(obj) => obj.hit_by(ray, t_min, t_max),
            Object::Prism(obj) => obj.hit_by(ray, t_min, t_max),
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

impl Into<Object> for Rectangle {
    fn into(self) -> Object {
        Object::Rectangle(self)
    }
}

impl Into<Object> for Triangle {
    fn into(self) -> Object {
        Object::Triangle(self)
    }
}

impl Into<Object> for Prism {
    fn into(self) -> Object {
        Object::Prism(self)
    }
}
