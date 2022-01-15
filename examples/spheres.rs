use rand_xorshift::XorShiftRng;
use teeracer::camera::Camera;
use teeracer::tracer::PathTracer;

use teeracer::hittables::{Plane, Sphere};
use teeracer::material::{DiffuseLight, IndexOfRefraction, Lambertian, Metal};
use teeracer::scene::Scene;
use teeracer::three::Three;

const RED: Three<f64> = Three::new(255.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0);
const GREEN: Three<f64> = Three::new(102.0 / 255.0, 255.0 / 255.0, 102.0 / 255.0);
const BLUE: Three<f64> = Three::new(102.0 / 255.0, 102.0 / 255.0, 255.0 / 255.0);
const WHITE: Three<f64> = Three::new(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0);

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // materials
    let white_lam = scene.add_material(Lambertian::new(WHITE));
    let green_metal = scene.add_material(Metal::new(GREEN));
    let red_lam = scene.add_material(Lambertian::new(RED));
    let blue_lam = scene.add_material(Lambertian::new(BLUE));
    let water = scene.add_material(IndexOfRefraction::Water);
    let crown_glass = scene.add_material(IndexOfRefraction::CrownGlass);
    let diamond = scene.add_material(IndexOfRefraction::Diamond);
    let white_light = scene.add_material(DiffuseLight::new(WHITE, 5.0));

    // lights
    scene.add_object(Sphere::new((0.0, 3.0, -3.0), 1.0), white_light);

    // objects
    scene.add_object(Sphere::new((-2.5, 0.5, -3.0), 1.0), green_metal);
    scene.add_object(Sphere::new((2.0, 0.5, -5.0), 1.5), red_lam);
    scene.add_object(Sphere::new((-2.0, 2.0, -6.0), 2.0), blue_lam);
    scene.add_object(Sphere::new((-1.0, -0.5, -2.5), 0.5), water);
    scene.add_object(Sphere::new((0.0, -0.75, -2.5), 0.5), crown_glass);
    scene.add_object(Sphere::new((1.0, -1.0, -2.5), -0.5), diamond);

    // box
    scene.add_object(Plane::new((-5.0, 0.0, 0.0), (1.0, 0.0, 0.0)), red_lam); // LEFT
    scene.add_object(Plane::new((5.0, 0.0, 0.0), (-1.0, 0.0, 0.0)), blue_lam); // RIGHT
    scene.add_object(Plane::new((0.0, -2.0, 0.0), (0.0, 1.0, 0.0)), white_lam); // BOTTOM
    scene.add_object(Plane::new((0.0, 4.0, 0.0), (0.0, -1.0, 0.0)), white_lam); // TOP
    scene.add_object(Plane::new((0.0, 0.0, -7.0), (0.0, 0.0, 1.0)), white_lam); // FRONT
    scene.add_object(Plane::new((0.0, 0.0, 7.0), (0.0, 0.0, -1.0)), white_lam); // BACK

    scene
}

fn main() {
    let scene = build_scene();

    let camera = Camera {
        position: (0.0, 0.0, 5.0).into(),
        width: 800,
        height: 600,
        fov: 45.0,
        samples: 1000,
        tracer: PathTracer::new(scene, 25),
    };
    let img = camera.render::<XorShiftRng>();
    img.save("spheres.png").expect("Failed to save image.");
}
