use rand::Rng;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Three<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Diffuse<F> {
    pub rgb: Three<F>,
}

#[derive(Debug, Clone, Copy)]
pub struct Mirror<F> {
    pub rgb: Three<F>,
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric<F> {
    pub rgb: Three<F>,
    pub ior: F,
}

#[derive(Debug, Clone, Copy)]
pub struct Light<F> {
    pub rgb: Three<F>,
    pub power: F,
}

#[derive(Debug, Clone, Copy)]
pub enum Material<F> {
    Diffuse(Diffuse<F>),
    Mirror(Mirror<F>),
    Dielectric(Dielectric<F>),
    Light(Light<F>),
}

#[derive(Debug, Clone, Copy)]
pub struct Ray<F> {
    pub origin: Three<F>,
    pub direction: Three<F>,
}

#[derive(Debug, Clone, Copy)]
pub struct Hit<F> {
    pub position: Three<F>,
    pub distance: F,
    pub normal: Three<F>,
    pub object_index: usize,
}

pub trait CanHit<T, F> {
    fn shoot_at(&self, obj: &T, t_min: F, t_max: F) -> Option<Hit<F>>;
}

pub enum LightInteraction<F> {
    Scatter {
        direction: Three<F>,
        attenuation: Three<F>,
    },
    Emit {
        emission: Three<F>,
    },
}

pub trait Surface<F> {
    fn sample_point_on_surface<R: Rng>(&self, rng: &mut R) -> Three<F>;
    fn normal_at_point(&self, point: &Three<F>) -> Three<F>;
}

pub enum FieldOfView<F> {
    Degrees(F),
    Radians(F),
}

pub struct ImageShape {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct LinearTransform<F> {
    pub scale: F,
    pub offset: F,
}

#[derive(Copy, Clone, Debug)]
pub struct Camera<F> {
    pub(crate) position: Three<F>,
    pub(crate) x_transform: LinearTransform<F>,
    pub(crate) y_transform: LinearTransform<F>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}
