use crate::three::Three;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Three<f64>,
    pub direction: Three<f64>,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            direction: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub position: Three<f64>,
    pub distance: f64,
    pub normal: Three<f64>,
}

pub trait CanHit {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
