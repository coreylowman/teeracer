mod camera;
mod linalg;
mod objects;
mod ray;

use crate::camera::Camera;
use crate::objects::{Plane, Sphere};
use crate::ray::{CanIntersect, Material};

fn main() {
    let mut objects: Vec<Box<dyn CanIntersect>> = Vec::new();
    objects.push(Box::new(Sphere {
        center: (0.0, 0.0, -5.0).into(),
        radius: 1.0,
        material: Material {
            color: (102, 255, 102).into(),
        },
    }));
    objects.push(Box::new(Sphere {
        center: (3.0, 2.0, -4.0).into(),
        radius: 2.0,
        material: Material {
            color: (255, 102, 102).into(),
        },
    }));
    objects.push(Box::new(Sphere {
        center: (-2.0, 2.0, -6.0).into(),
        radius: 2.0,
        material: Material {
            color: (102, 102, 255).into(),
        },
    }));
    objects.push(Box::new(Plane {
        center: (0.0, -2.0, 0.0).into(),
        normal: (0.0, -1.0, 0.0).into(),
        material: Material {
            color: (150, 75, 0).into(),
        },
    }));
    objects.push(Box::new(Plane {
        center: (0.0, 0.0, -20.0).into(),
        normal: (0.0, 0.0, -1.0).into(),
        material: Material {
            color: (135, 206, 235).into(),
        },
    }));
    let camera = Camera {
        width: 800,
        height: 600,
        fov: 90.0,
    };
    let img = camera.render(objects);
    img.save("output.png").expect("Failed to save image.");
}
