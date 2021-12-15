use crate::linalg::Vec3;

pub(crate) struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) length: f64,
    pub(crate) color: Vec3,
}

impl Ray {
    pub(crate) fn at(direction: Vec3) -> Self {
        let mut ray: Self = Default::default();
        ray.direction = direction;
        ray
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            direction: Default::default(),
            length: f64::INFINITY,
            color: (0.0, 0.0, 0.0).into(),
        }
    }
}

pub(crate) trait RayTransformer {
    fn transform(&self, ray: Ray) -> Ray;
}
