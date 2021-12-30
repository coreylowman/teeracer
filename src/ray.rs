use rand::prelude::{thread_rng, Rng};

use crate::linalg::{Length, Vec3};

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
            (self[0].clamp(0.0, 1.0) * 255.0).round() as u8,
            (self[1].clamp(0.0, 1.0) * 255.0).round() as u8,
            (self[2].clamp(0.0, 1.0) * 255.0).round() as u8,
        )
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Material {
    Lambertian { color: Color },
    Metal { color: Color, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
    DiffuseLight { color: Color },
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
        self.partial_cmp(other)
            .unwrap_or_else(|| panic!("{:?} {:?}", self.distance, other.distance))
    }
}

pub(crate) trait CanHit {
    fn hit_by(&self, ray: Ray) -> Option<Hit>;
}

impl Hit {
    pub(crate) fn attenuate(
        &self,
        light: &mut Vec3<f64>,
        in_dir: Vec3<f64>,
        out_dir: Option<Vec3<f64>>,
    ) {
        match self.material {
            Material::Lambertian { color: rgb } => {
                let color: Vec3<f64> = rgb.into();
                // let cos_theta = (self.normal).dot(&out_dir.unwrap());
                // assert!(cos_theta.is_sign_positive());
                *light *= color; // * cos_theta;
            }
            Material::Metal {
                color: rgb,
                fuzz: _,
            } => {
                let color: Vec3<f64> = rgb.into();
                *light *= color;
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
                if noise.dot(&self.normal).is_sign_negative() {
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
                if noise.dot(&self.normal).is_sign_negative() {
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
                let cos_theta = ray.direction.dot(&self.normal);
                let exiting = cos_theta.is_sign_positive();
                let outward_normal = if exiting { -self.normal } else { self.normal };
                let ratio = if exiting {
                    index_of_refraction
                } else {
                    1.0 / index_of_refraction
                };
                let cos_theta = cos_theta.abs();
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                // let r0 = (1.0 - ratio) / (1.0 + ratio);
                // let r1 = r0 * r0;
                // let reflectance = r1 + (1.0 - r1) * (1.0 - cos_theta).powi(5);
                // assert!(reflectance < 1.0 && reflectance > 0.0, "reflectance={}", reflectance);

                // let mut rng = thread_rng();
                let direction = if ratio * sin_theta > 1.0
                // || reflectance > rng.gen_range(0.0..1.0))
                {
                    ray.direction.reflect(&outward_normal).normalized()
                } else {
                    let perp = (ray.direction + outward_normal * cos_theta) * ratio;
                    let para = outward_normal * -(1.0 - perp.length_squared()).abs().sqrt();
                    (perp + para).normalized()
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
        *self - *normal * (self.dot(normal) * 2.0)
    }
}
