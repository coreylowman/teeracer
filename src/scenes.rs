use crate::material::{DiffuseLight, IndexOfRefraction, Lambertian, Metal};
use crate::hittables::{Plane, Prism, Sphere};
use crate::scene::Scene;
use crate::three::Three;

const RED: Three<f64> = Three::new(255.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0);
const GREEN: Three<f64> = Three::new(102.0 / 255.0, 255.0 / 255.0, 102.0 / 255.0);
const BLUE: Three<f64> = Three::new(102.0 / 255.0, 102.0 / 255.0, 255.0 / 255.0);
const WHITE: Three<f64> = Three::new(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0);
const VIOLET: Three<f64> = Three::new(0.5, 0.0, 1.0);
// const X: Three<f64> = Three::new(1.0, 0.0, 0.0);
const Y: Three<f64> = Three::new(0.0, 1.0, 0.0);
// const Z: Three<f64> = Three::new(0.0, 0.0, 1.0);

pub fn spheres() -> Scene {
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

pub fn mirrored_prism() -> Scene {
    let mut scene = Scene::new();

    // materials
    let white_lam = scene.add_material(Lambertian::new(WHITE));
    let mirror = scene.add_material(Metal::new(1.0.into()));
    let red_lam = scene.add_material(Lambertian::new(RED));
    let blue_lam = scene.add_material(Lambertian::new(BLUE));
    let white_light = scene.add_material(DiffuseLight::new(WHITE, 5.0));

    // lights
    scene.add_object(Sphere::new((0.0, 3.0, -3.0), 1.0), white_light);

    // objects
    scene.add_object(
        Prism::new(
            (-0.5, -2.0, -2.0),
            (0.5, -2.0, -2.0),
            (0.0, -1.0, -2.0),
            1.0,
        )
        .rotated(&Y, 45.0),
        mirror,
    );

    // box
    scene.add_object(Plane::new((-5.0, 0.0, 0.0), (1.0, 0.0, 0.0)), red_lam); // LEFT
    scene.add_object(Plane::new((5.0, 0.0, 0.0), (-1.0, 0.0, 0.0)), blue_lam); // RIGHT
    scene.add_object(Plane::new((0.0, -2.0, 0.0), (0.0, 1.0, 0.0)), white_lam); // BOTTOM
    scene.add_object(Plane::new((0.0, 4.0, 0.0), (0.0, -1.0, 0.0)), white_lam); // TOP
    scene.add_object(Plane::new((0.0, 0.0, -7.0), (0.0, 0.0, 1.0)), mirror); // FRONT

    scene
}

pub fn glass_prism() -> Scene {
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
