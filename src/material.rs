use crate::linalg::Three;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub rgb: Three<f64>,
}

impl Lambertian {
    pub fn new(rgb: Three<f64>) -> Self {
        Self { rgb }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub rgb: Three<f64>,
    pub fuzz: Option<f64>,
}

impl Metal {
    pub fn new(rgb: Three<f64>) -> Self {
        Self { rgb, fuzz: None }
    }

    pub fn fuzzy(rgb: Three<f64>, fuzz: f64) -> Self {
        Self {
            rgb,
            fuzz: Some(fuzz),
        }
    }
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

impl DiffuseLight {
    pub fn new(rgb: Three<f64>, power: f64) -> Self {
        Self { rgb, power }
    }
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

impl Into<Material> for IndexOfRefraction {
    fn into(self) -> Material {
        Material::Dielectric(Dielectric { ior: self })
    }
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
