use crate::linalg::Vec3;

#[derive(Clone, Copy)]
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

impl Ray {
    pub(crate) fn at(direction: Vec3) -> Self {
        let mut ray: Self = Default::default();
        ray.direction = direction;
        ray
    }
}

type Color = Vec3<u8>;

#[derive(Clone, Copy)]
pub(crate) enum Material {
    Lambertian { color: Color },
    Metal { color: Color, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
}

impl Material {
    pub(crate) fn scatter(&self, ray: Ray) -> (Ray, Color) {
        match self {
            Self::Lambertian { color } => (ray, *color),
            Self::Metal { color, fuzz } => (ray, *color),
            Self::Dielectric {
                index_of_refraction,
            } => (ray, (0, 0, 0).into()),
        }
    }
}

pub(crate) struct Intersection {
    pub(crate) distance: f64,
    pub(crate) normal: Vec3,
    pub(crate) material: Material,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub(crate) trait CanIntersect {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}
