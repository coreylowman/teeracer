mod camera;
mod linalg;
mod objects;
mod ray;

use crate::camera::Camera;
use crate::objects::{Object, Plane, Sphere};
use crate::ray::Material;

const RED: (u8, u8, u8) = (255, 102, 102);
const GREEN: (u8, u8, u8) = (102, 255, 102);
const BLUE: (u8, u8, u8) = (102, 102, 255);

fn main() {
    let mut objects: Vec<Object> = Vec::new();
    objects.push(
        Sphere {
            center: (-3.0, 1.0, -4.0).into(),
            radius: 1.0,
            material: Material::Metal {
                color: GREEN.into(),
                fuzz: 1.0,
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
            center: (0.0, 1.0, -3.0).into(),
            radius: 0.75,
            material: Material::DiffuseLight {
                color: (255, 255, 255).into(),
            },
        }
        .into(),
    );
    objects.push(
        Plane {
            center: (0.0, -2.0, 0.0).into(),
            normal: (0.0, -1.0, 0.0).into(),
            material: Material::Lambertian {
                color: (255, 255, 255).into(),
            },
        }
        .into(),
    );
    objects.push(
        Plane {
            center: (0.0, 0.0, -7.0).into(),
            normal: (0.0, 0.0, -1.0).into(),
            material: Material::Lambertian {
                color: (135, 206, 235).into(),
            },
        }
        .into(),
    );
    let camera = Camera {
        width: 800,
        height: 600,
        fov: 90.0,
        bounces: 50,
        samples: 100,
    };
    let img = camera.render(objects);
    img.save("output.png").expect("Failed to save image.");
}
