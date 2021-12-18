mod plane;
mod sphere;

use crate::ray::{CanHit, Hit, Ray};
pub(crate) use plane::Plane;
pub(crate) use sphere::Sphere;

pub(crate) enum Object {
    Plane(Plane),
    Sphere(Sphere),
}

impl CanHit for Object {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        match self {
            Object::Plane(plane) => plane.hit_by(ray),
            Object::Sphere(sphere) => sphere.hit_by(ray),
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
