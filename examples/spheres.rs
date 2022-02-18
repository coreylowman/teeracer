use rand_xorshift::XorShiftRng;
use teeracer::*;

fn main() -> Result<(), image::error::ImageError> {
    let camera = Camera::new(
        FieldOfView::Degrees(45.0),
        ImageShape {
            width: 800,
            height: 600,
        },
    )
    .at(0.0, 0.0, 5.0);

    let mut scene = Scene::new();

    // materials
    let white = scene.add_material(Diffuse::rgb(1.0, 1.0, 1.0));
    let green_mirror = scene.add_material(Mirror::tinted(0.25, 1.0, 0.25));
    let red = scene.add_material(Diffuse::rgb(1.0, 0.25, 0.25));
    let blue = scene.add_material(Diffuse::rgb(0.25, 0.25, 1.0));
    let water = scene.add_material(Dielectric { ior: 1.33 });
    let crown_glass = scene.add_material(Dielectric { ior: 1.52 });
    let diamond = scene.add_material(Dielectric { ior: 2.417 });
    let white_light = scene.add_material(Light {
        rgb: Three::new(1.0, 1.0, 1.0),
        power: 5.0,
    });

    // lights
    scene.add_object(Sphere::unit_at(0.0, 3.0, -3.0), white_light);

    // objects
    scene.add_object(Sphere::unit_at(-2.5, 0.5, -3.0), green_mirror);
    scene.add_object(Sphere::unit_at(2.0, 0.5, -5.0).scaled(1.5), red);
    scene.add_object(Sphere::unit_at(-2.0, 2.0, -6.0).scaled(2.0), blue);
    scene.add_object(Sphere::unit_at(-1.0, -0.5, -2.5).scaled(0.5), water);
    scene.add_object(Sphere::unit_at(0.0, -0.75, -2.5).scaled(0.5), crown_glass);
    scene.add_object(Sphere::unit_at(1.0, -1.0, -2.5).scaled(0.5), diamond);

    // surrounding box
    scene.add_object(Plane::facing_pos_x().shifted_back(5.0), red); // LEFT
    scene.add_object(Plane::facing_neg_x().shifted_back(5.0), blue); // RIGHT
    scene.add_object(Plane::facing_pos_y().shifted_back(2.0), white); // BOTTOM
    scene.add_object(Plane::facing_neg_y().shifted_back(4.0), white); // TOP
    scene.add_object(Plane::facing_pos_z().shifted_back(7.0), white); // FRONT
    scene.add_object(Plane::facing_neg_z().shifted_back(7.0), white); // BACK

    let tracer = PathTracer { depth: 10 };
    render::<PathTracer, f32, XorShiftRng>(tracer, scene, camera, 1000).save("spheres.png")?;

    Ok(())
}
