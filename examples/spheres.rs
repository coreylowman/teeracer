use rand_xorshift::XorShiftRng;
use teeracer::*;

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // materials
    let white_lam = scene.add_material(Lambertian::rgb(1.0, 1.0, 1.0));
    let green_metal = scene.add_material(Metal {
        rgb: Three::new(0.25, 1.0, 0.25),
        fuzz: None,
    });
    let red_lam = scene.add_material(Lambertian::rgb(1.0, 0.25, 0.25));
    let blue_lam = scene.add_material(Lambertian::rgb(0.25, 0.25, 1.0));
    let water = scene.add_material(Dielectric { ior: 1.33 });
    let crown_glass = scene.add_material(Dielectric { ior: 1.52 });
    let diamond = scene.add_material(Dielectric { ior: 2.417 });
    let white_light = scene.add_material(DiffuseLight {
        rgb: Three::new(1.0, 1.0, 1.0),
        power: 5.0,
    });

    // lights
    scene.add_object(Sphere::unit_at(0.0, 3.0, -3.0), white_light);

    // objects
    scene.add_object(Sphere::unit_at(-2.5, 0.5, -3.0), green_metal);
    scene.add_object(Sphere::unit_at(2.0, 0.5, -5.0).scaled(1.5), red_lam);
    scene.add_object(Sphere::unit_at(-2.0, 2.0, -6.0).scaled(2.0), blue_lam);
    scene.add_object(Sphere::unit_at(-1.0, -0.5, -2.5).scaled(0.5), water);
    scene.add_object(Sphere::unit_at(0.0, -0.75, -2.5).scaled(0.5), crown_glass);
    scene.add_object(Sphere::unit_at(1.0, -1.0, -2.5).scaled(0.5), diamond);

    // surrounding box
    scene.add_object(Plane::facing_pos_x().shifted_back(5.0), red_lam); // LEFT
    scene.add_object(Plane::facing_neg_x().shifted_back(5.0), blue_lam); // RIGHT
    scene.add_object(Plane::facing_pos_y().shifted_back(2.0), white_lam); // BOTTOM
    scene.add_object(Plane::facing_neg_y().shifted_back(4.0), white_lam); // TOP
    scene.add_object(Plane::facing_pos_z().shifted_back(7.0), white_lam); // FRONT
    scene.add_object(Plane::facing_neg_z().shifted_back(7.0), white_lam); // BACK

    scene
}

fn main() -> Result<(), image::error::ImageError> {
    let camera = Camera::new(
        FieldOfView::Degrees(45.0),
        ImageShape {
            width: 800,
            height: 600,
        },
    )
    .at(0.0, 0.0, 5.0);
    let scene = build_scene();
    render::<PathTracer, XorShiftRng>(scene, camera, 25, 1000).save("spheres.png")?;
    Ok(())
}
