mod plane;
mod prism;
mod sphere;
mod triangle;

pub use plane::Plane;
pub use prism::Prism;
pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::data::{CanHit, Hit, Ray};

pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
    Triangle(Triangle),
    Prism(Prism),
}

impl CanHit<Object> for Ray {
    fn shoot_at(&self, obj: &Object, t_min: f64, t_max: f64) -> Option<Hit> {
        match obj {
            Object::Plane(obj) => self.shoot_at(obj, t_min, t_max),
            Object::Sphere(obj) => self.shoot_at(obj, t_min, t_max),
            Object::Triangle(obj) => self.shoot_at(obj, t_min, t_max),
            Object::Prism(obj) => self.shoot_at(obj, t_min, t_max),
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
