mod camera;
mod linalg;
mod objects;
mod ray;
mod trace;

use crate::camera::Camera;
use crate::objects::{Object, Plane, Sphere};
use crate::ray::Material;

const RED: (u8, u8, u8) = (255, 102, 102);
const GREEN: (u8, u8, u8) = (102, 255, 102);
const BLUE: (u8, u8, u8) = (102, 102, 255);
const WHITE: (u8, u8, u8) = (255, 255, 255);

mod refraction {
    pub const VACUUM: f64 = 1.0;
    pub const AIR: f64 = 1.00029;
    pub const ICE: f64 = 1.31;
    pub const WATER: f64 = 1.33;
    pub const CROWN_GLASS: f64 = 1.52;
    pub const DIAMOND: f64 = 2.417;
}

fn main() {
    let mut objects: Vec<Object> = Vec::new();
    objects.push(
        Sphere {
            center: (-2.5, 0.5, -4.0).into(),
            radius: 1.0,
            material: Material::Metal {
                color: GREEN.into(),
                fuzz: 0.0,
            },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (3.0, 2.0, -4.0).into(),
            radius: 2.0,
            material: Material::Lambertian { color: RED.into() },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (-2.0, 2.0, -6.0).into(),
            radius: 2.0,
            material: Material::Lambertian { color: BLUE.into() },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (-1.0, 0.0, -2.5).into(),
            radius: 0.5,
            material: Material::Dielectric {
                index_of_refraction: refraction::CROWN_GLASS,
            },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (0.0, 0.0, -2.5).into(),
            radius: 0.5,
            material: Material::Dielectric {
                index_of_refraction: refraction::CROWN_GLASS,
            },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (1.0, 0.0, -2.5).into(),
            radius: -0.5,
            material: Material::Dielectric {
                index_of_refraction: refraction::CROWN_GLASS,
            },
        }
        .into(),
    );
    objects.push(
        Plane {
            // LEFT
            center: (-4.0, 0.0, 0.0).into(),
            normal: (1.0, 0.0, 0.0).into(),
            material: Material::Lambertian {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    objects.push(
        Plane {
            // RIGHT
            center: (4.0, 0.0, 0.0).into(),
            normal: (-1.0, 0.0, 0.0).into(),
            material: Material::Lambertian {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    objects.push(
        Plane {
            // BOTTOM
            center: (0.0, -2.0, 0.0).into(),
            normal: (0.0, 1.0, 0.0).into(),
            material: Material::Lambertian {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    objects.push(
        Plane {
            // TOP
            center: (0.0, 4.0, 0.0).into(),
            normal: (0.0, -1.0, 0.0).into(),
            material: Material::Lambertian {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    objects.push(
        // FRONT
        Plane {
            center: (0.0, 0.0, -7.0).into(),
            normal: (0.0, 0.0, 1.0).into(),
            material: Material::Lambertian {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    objects.push(
        // BACK
        Plane {
            center: (0.0, 0.0, 0.0).into(),
            normal: (0.0, 0.0, -1.0).into(),
            material: Material::Lambertian {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (0.0, 1.0, -3.0).into(),
            radius: 0.75,
            material: Material::DiffuseLight {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (0.0, 10.0, -3.0).into(),
            radius: 0.75,
            material: Material::DiffuseLight {
                color: WHITE.into(),
            },
        }
        .into(),
    );
    let camera = Camera {
        width: 500,
        height: 500,
        fov: 90.0,
        bounces: 20,
        samples: 10,
    };
    let img = camera.render(objects);
    img.save("output.png").expect("Failed to save image.");
}
