mod camera;
mod linalg;
mod material;
mod objects;
mod ray;
mod scene;
mod tracer;

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use crate::camera::Camera;
use crate::linalg::Three;
use crate::material::{Dielectric, DiffuseLight, IndexOfRefraction, Lambertian, Metal};
use crate::objects::{Plane, Sphere};
use crate::scene::Scene;
use crate::tracer::PathTracer;

impl Into<Three<f64>> for Three<u8> {
    fn into(self) -> Three<f64> {
        Three::new(
            self.x as f64 / 255.0,
            self.y as f64 / 255.0,
            self.z as f64 / 255.0,
        )
    }
}

const RED: Three<u8> = Three::new(255, 102, 102);
const GREEN: Three<u8> = Three::new(102, 255, 102);
const BLUE: Three<u8> = Three::new(102, 102, 255);
const WHITE: Three<u8> = Three::new(255, 255, 255);

fn main() {
    let mut scene = Scene::new();

    // materials
    let white_lam = scene.add_material(Lambertian { rgb: WHITE.into() });
    let green_metal = scene.add_material(Metal {
        rgb: GREEN.into(),
        fuzz: None,
    });
    let red_lam = scene.add_material(Lambertian { rgb: RED.into() });
    let blue_lam = scene.add_material(Lambertian { rgb: BLUE.into() });
    let water = scene.add_material(Dielectric {
        ior: IndexOfRefraction::Water,
    });
    let crown_glass = scene.add_material(Dielectric {
        ior: IndexOfRefraction::CrownGlass,
    });
    let diamond = scene.add_material(Dielectric {
        ior: IndexOfRefraction::Diamond,
    });
    let white_light = scene.add_material(DiffuseLight {
        rgb: WHITE.into(),
        power: 3.0,
    });

    // box
    scene.add_object(Plane::new((-5.0, 0.0, 0.0), (1.0, 0.0, 0.0)), red_lam); // LEFT
    scene.add_object(Plane::new((5.0, 0.0, 0.0), (-1.0, 0.0, 0.0)), blue_lam); // RIGHT
    scene.add_object(Plane::new((0.0, -2.0, 0.0), (0.0, 1.0, 0.0)), white_lam); // BOTTOM
    scene.add_object(Plane::new((0.0, 4.0, 0.0), (0.0, -1.0, 0.0)), white_lam); // TOP
    scene.add_object(Plane::new((0.0, 0.0, -7.0), (0.0, 0.0, 1.0)), white_lam); // FRONT
    scene.add_object(Plane::new((0.0, 0.0, 7.0), (0.0, 0.0, -1.0)), white_lam); // BACK

    // lights
    scene.add_object(Sphere::new((0.0, 3.0, -3.0), 1.0), white_light);

    // objects
    scene.add_object(Sphere::new((-2.5, 0.5, -3.0), 1.0), green_metal);
    scene.add_object(Sphere::new((2.0, 0.5, -5.0), 1.5), red_lam);
    scene.add_object(Sphere::new((-2.0, 2.0, -6.0), 2.0), blue_lam);
    scene.add_object(Sphere::new((-1.0, -0.5, -2.5), 0.5), water);
    scene.add_object(Sphere::new((0.0, -0.75, -2.5), 0.5), crown_glass);
    scene.add_object(Sphere::new((1.0, -1.0, -2.5), -0.5), diamond);

    let tracer = PathTracer::new(scene, 25);

    let camera = Camera {
        position: (0.0, 0.0, 5.0).into(),
        width: 800,
        height: 600,
        fov: 45.0,
        samples: 100,
    };
    let rng = XorShiftRng::seed_from_u64(0);
    let img = camera.render(tracer, rng);
    img.save("output.png").expect("Failed to save image.");
}
