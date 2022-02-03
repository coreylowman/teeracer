use rand_xorshift::XorShiftRng;
use teeracer::*;

const RED: Three<f64> = Three::new(1.0, 0.25, 0.25);
const BLUE: Three<f64> = Three::new(0.25, 0.25, 1.0);
const WHITE: Three<f64> = Three::new(1.0, 1.0, 1.0);
const VIOLET: Three<f64> = Three::new(0.5, 0.0, 1.0);
const Y: Three<f64> = Three::new(0.0, 1.0, 0.0);

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // materials
    let white_lam = scene.add_material(Lambertian::new(WHITE));
    let violet_lam = scene.add_material(Lambertian::new(VIOLET));
    let red_lam = scene.add_material(Lambertian::new(RED));
    let blue_lam = scene.add_material(Lambertian::new(BLUE));
    let white_light = scene.add_material(DiffuseLight::new(WHITE, 5.0));
    let crown_glass = scene.add_material(IndexOfRefraction::CrownGlass);

    // lights
    scene.add_object(Sphere::new((0.0, 3.0, -3.0), 1.0), white_light);

    // objects
    scene.add_object(
        Prism::new(
            (-0.5, -1.5, -2.0),
            (0.5, -1.5, -2.0),
            (0.0, -0.5, -2.0),
            1.0,
        )
        .rotated(&Y, 45.0),
        crown_glass,
    );

    // box
    scene.add_object(Plane::new((-5.0, 0.0, 0.0), (1.0, 0.0, 0.0)), red_lam); // LEFT
    scene.add_object(Plane::new((5.0, 0.0, 0.0), (-1.0, 0.0, 0.0)), blue_lam); // RIGHT
    scene.add_object(Plane::new((0.0, -2.0, 0.0), (0.0, 1.0, 0.0)), white_lam); // BOTTOM
    scene.add_object(Plane::new((0.0, 4.0, 0.0), (0.0, -1.0, 0.0)), white_lam); // TOP
    scene.add_object(Plane::new((0.0, 0.0, -7.0), (0.0, 0.0, 1.0)), violet_lam); // FRONT

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
    img.save("glass-prism.png").expect("Failed to save image.");
}
