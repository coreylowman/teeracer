use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Three<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub rgb: Three<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub rgb: Three<f64>,
    pub fuzz: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub ior: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight {
    pub rgb: Three<f64>,
    pub power: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Three<f64>,
    pub direction: Three<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub position: Three<f64>,
    pub distance: f64,
    pub normal: Three<f64>,
    pub object_index: usize,
}

pub trait CanHit<T> {
    fn shoot_at(&self, obj: &T, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub enum FieldOfView {
    Degrees(f64),
    Radians(f64),
}

pub struct ImageShape {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct LinearTransform {
    pub scale: f64,
    pub offset: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub(crate) position: Three<f64>,
    pub(crate) x_transform: LinearTransform,
    pub(crate) y_transform: LinearTransform,
    pub(crate) width: usize,
    pub(crate) height: usize,
}
