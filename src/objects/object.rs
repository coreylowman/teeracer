use super::plane::Plane;
use super::rectangle::Rectangle;
use super::sphere::Sphere;
use crate::ray::{CanHit, Hit, Ray};

pub enum Object {
    Plane(Plane),
    Sphere(Sphere),
    Rectangle(Rectangle),
}

impl CanHit for Object {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        match self {
            Object::Plane(obj) => obj.hit_by(ray),
            Object::Sphere(obj) => obj.hit_by(ray),
            Object::Rectangle(obj) => obj.hit_by(ray),
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
