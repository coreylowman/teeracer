use crate::linalg::Vec3;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
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
pub(crate) struct Hit {
    pub(crate) position: Vec3,
    pub(crate) distance: f64,
    pub(crate) normal: Vec3,
    pub(crate) material: Material,
}

impl PartialEq for Hit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Eq for Hit {}

impl PartialOrd for Hit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Hit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub(crate) trait CanHit {
    fn hit_by(&self, ray: Ray) -> Option<Hit>;
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bounce {
    pub(crate) incoming: Ray,
    pub(crate) hit: Hit,
    pub(crate) outgoing: Ray,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Absorb {
    pub(crate) incoming: Ray,
    pub(crate) hit: Hit,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Interaction {
    Bounced(Bounce),
    Absorbed(Absorb),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Material {
    Lambertian { rgb: Vec3<f64> },
    Metal { rgb: Vec3<f64>, fuzz: f64 },
    Dielectric(Refractor),
    DiffuseLight { rgb: Vec3<f64> },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Refractor {
    Vacuum,
    Air,
    Ice,
    Water,
    CrownGlass,
    Diamond,
}

impl Refractor {
    pub(crate) const fn index(self) -> f64 {
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
