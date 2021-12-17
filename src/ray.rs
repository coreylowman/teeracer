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

#[derive(Clone, Copy)]
pub(crate) struct Material {
    pub(crate) color: Vec3<u8>,
}

pub(crate) struct Intersection {
    pub(crate) distance: f64,
    pub(crate) result: Ray,
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
