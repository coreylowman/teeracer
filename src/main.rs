mod camera;
mod linalg;
mod objects;
mod ray;
mod trace;

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use crate::camera::Camera;
use crate::linalg::Three;
use crate::objects::{Object, Plane, Sphere};
use crate::ray::{Material, Refractor};

impl Into<Three<f64>> for Three<u8> {
    fn into(self) -> Three<f64> {
        (
            self[0] as f64 / 255.0,
            self[1] as f64 / 255.0,
            self[2] as f64 / 255.0,
        )
            .into()
    }
}

const RED: Three<u8> = Three::new(255, 102, 102);
const GREEN: Three<u8> = Three::new(102, 255, 102);
const BLUE: Three<u8> = Three::new(102, 102, 255);
const WHITE: Three<u8> = Three::new(255, 255, 255);

fn main() {
    let mut objects: Vec<Object> = Vec::new();
    objects.push(
        Sphere {
            center: (-2.5, 0.5, -3.0).into(),
            radius: 1.0,
            material: Material::Metal {
                rgb: GREEN.into(),
                fuzz: 0.0,
            },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (2.0, 0.5, -5.0).into(),
            radius: 1.5,
            material: Material::Lambertian { rgb: RED.into() },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (-2.0, 2.0, -6.0).into(),
            radius: 2.0,
            material: Material::Lambertian { rgb: BLUE.into() },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (-1.0, -0.5, -2.5).into(),
            radius: 0.5,
            material: Material::Dielectric(Refractor::Water),
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (0.0, -0.75, -2.5).into(),
            radius: 0.5,
            material: Material::Dielectric(Refractor::CrownGlass),
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (1.0, -1.0, -2.5).into(),
            radius: -0.5,
            material: Material::Dielectric(Refractor::Diamond),
        }
        .into(),
    );
    objects.push(
        Plane {
            // LEFT
            center: (-5.0, 0.0, 0.0).into(),
            normal: (1.0, 0.0, 0.0).into(),
            material: Material::Lambertian { rgb: WHITE.into() },
        }
        .into(),
    );
    objects.push(
        Plane {
            // RIGHT
            center: (5.0, 0.0, 0.0).into(),
            normal: (-1.0, 0.0, 0.0).into(),
            material: Material::Lambertian { rgb: WHITE.into() },
        }
        .into(),
    );
    objects.push(
        Plane {
            // BOTTOM
            center: (0.0, -2.0, 0.0).into(),
            normal: (0.0, 1.0, 0.0).into(),
            material: Material::Lambertian { rgb: WHITE.into() },
        }
        .into(),
    );
    objects.push(
        Plane {
            // TOP
            center: (0.0, 4.0, 0.0).into(),
            normal: (0.0, -1.0, 0.0).into(),
            material: Material::Lambertian { rgb: WHITE.into() },
        }
        .into(),
    );
    objects.push(
        // FRONT
        Plane {
            center: (0.0, 0.0, -7.0).into(),
            normal: (0.0, 0.0, 1.0).into(),
            material: Material::Lambertian { rgb: WHITE.into() },
        }
        .into(),
    );
    objects.push(
        // BACK
        Plane {
            center: (0.0, 0.0, 2.0).into(),
            normal: (0.0, 0.0, -1.0).into(),
            material: Material::Lambertian { rgb: WHITE.into() },
        }
        .into(),
    );
    objects.push(
        Sphere {
            center: (0.0, 3.0, -3.0).into(),
            radius: 1.0,
            material: Material::DiffuseLight { rgb: WHITE.into() },
        }
        .into(),
    );
    let mut rng = XorShiftRng::seed_from_u64(0);
    let camera = Camera {
        position: (0.0, 0.0, 0.5).into(),
        width: 800,
        height: 600,
        fov: 90.0,
        bounces: 25,
        samples: 100,
    };
    let img = camera.render(objects, &mut rng);
    img.save("output.png").expect("Failed to save image.");
}
