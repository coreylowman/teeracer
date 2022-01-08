use crate::linalg::Three;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { rgb: Three<f64> },
    Metal { rgb: Three<f64>, fuzz: f64 },
    Dielectric(IndexOfRefraction),
    DiffuseLight { rgb: Three<f64>, power: f64 },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum IndexOfRefraction {
    Vacuum,
    Air,
    Ice,
    Water,
    CrownGlass,
    Diamond,
}

impl IndexOfRefraction {
    pub const fn value(&self) -> f64 {
        match self {
            Self::Vacuum => 1.0,
            Self::Air => 1.00029,
            Self::Ice => 1.31,
            Self::Water => 1.33,
            Self::CrownGlass => 1.52,
            Self::Diamond => 2.417,
        }
    }
}
