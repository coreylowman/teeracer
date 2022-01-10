use crate::linalg::Three;

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
    pub ior: IndexOfRefraction,
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

impl Into<Material> for Lambertian {
    fn into(self) -> Material {
        Material::Lambertian(self)
    }
}

impl Into<Material> for Metal {
    fn into(self) -> Material {
        Material::Metal(self)
    }
}

impl Into<Material> for Dielectric {
    fn into(self) -> Material {
        Material::Dielectric(self)
    }
}

impl Into<Material> for DiffuseLight {
    fn into(self) -> Material {
        Material::DiffuseLight(self)
    }
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
