use rand::prelude::{thread_rng, Rng};

use crate::linalg::{Length, Vec3};

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
            (self[0].clamp(0.0, 1.0) * 255.0) as u8,
            (self[1].clamp(0.0, 1.0) * 255.0) as u8,
            (self[2].clamp(0.0, 1.0) * 255.0) as u8,
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
                let scattered = self.normal;
                let mut noise = Vec3::random_unit();
                if noise.dot(&self.normal) < 0.0 {
                    noise = -noise;
                }
                let direction = (scattered + noise).normalized();
                Some(Ray {
                    origin: self.position,
                    direction,
                })
                .filter(|ray| ray.direction.dot(&self.normal) > 0.0)
            }
            Material::Metal { color: _, fuzz } => {
                let scattered = ray.direction.reflect(&self.normal);
                let mut noise = Vec3::random_unit();
                if noise.dot(&self.normal) < 0.0 {
                    noise = -noise;
                }
                let direction = (scattered + noise * fuzz).normalized();
                Some(Ray {
                    origin: self.position,
                    direction,
                })
                .filter(|ray| ray.direction.dot(&self.normal) > 0.0)
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let ratio = 1.0 / index_of_refraction;
                let unit_direction = ray.direction.normalized();
                let cos_theta = (-unit_direction).dot(&self.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                let r0 = (1.0 - ratio) / (1.0 + ratio);
                let r1 = r0 * r0;
                let reflectance = r1 + (1.0 - r1) * (1.0 - cos_theta).powi(5);

                let mut rng = thread_rng();
                let direction = if ratio * sin_theta > 1.0 || reflectance > rng.gen_range(0.0..1.0)
                {
                    unit_direction.reflect(&self.normal)
                } else {
                    unit_direction.refract(self.normal, ratio)
                };
                Some(Ray {
                    origin: self.position,
                    direction,
                })
            }
            Material::DiffuseLight { color: _ } => None,
        }
    }
}

impl Vec3<f64> {
    fn reflect(&self, normal: &Self) -> Self {
        *self - (*normal * self.dot(normal)) * 2.0
    }

    fn refract(self, normal: Self, ratio: f64) -> Self {
        let cos_theta = (-self).dot(&normal).min(1.0);
        let r_out_perp = (self + normal * cos_theta) * ratio;
        let r_out_parallel = normal * -(1.0 - r_out_perp.length().powi(2)).abs().sqrt();
        r_out_parallel + r_out_perp
    }
}
