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

pub(crate) type Color = Vec3<u8>;

impl Into<Vec3<f64>> for Color {
    fn into(self) -> Vec3<f64> {
        (
            self[0] as f64 / 255.0,
            self[1] as f64 / 255.0,
            self[2] as f64 / 255.0,
        )
            .into()
    }
}

impl Into<Color> for Vec3<f64> {
    fn into(self) -> Color {
        (
            (self[0] * 255.0) as u8,
            (self[1] * 255.0) as u8,
            (self[2] * 255.0) as u8,
        )
            .into()
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Material {
    Lambertian { color: Color },
    Metal { color: Color, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
    DiffuseLight { color: Color },
}

#[derive(Clone, Copy)]
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

impl Hit {
    pub(crate) fn attenuate(&self, light: &mut Vec3<f64>) {
        match self.material {
            Material::Lambertian { color } => {
                let color: Vec3<f64> = color.into();
                *light = *light * color;
            }
            Material::Metal { color, fuzz: _ } => {
                let color: Vec3<f64> = color.into();
                *light = *light * color;
            }
            Material::Dielectric {
                index_of_refraction: _,
            } => {}
            Material::DiffuseLight { color } => {
                *light = color.into();
            }
        }
    }

    pub(crate) fn scatter(&self, ray: Ray) -> Option<Ray> {
        match self.material {
            Material::Lambertian { color: _ } => {
                let mut scatter_direction = self.normal + Vec3::random_unit();
                if scatter_direction.near_zero() {
                    scatter_direction = self.normal;
                }
                Some(Ray {
                    origin: self.position,
                    direction: scatter_direction,
                })
            }
            Material::Metal { color: _, fuzz } => {
                let reflected = ray.direction.reflect(&self.normal);
                let mut noise = Vec3::random_unit();
                if noise.dot(&self.normal) <= 0.0 {
                    noise = -noise;
                }
                let direction = reflected + noise * fuzz;
                if direction.dot(&self.normal) > 0.0 {
                    Some(Ray {
                        origin: self.position,
                        direction,
                    })
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction: _,
            } => None,
            Material::DiffuseLight { color: _ } => None,
        }
    }
}
