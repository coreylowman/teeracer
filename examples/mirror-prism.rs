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
    let mirror = scene.add_material(Mirror::new());
    let red = scene.add_material(Diffuse::rgb(1.0, 0.25, 0.25));
    let blue = scene.add_material(Diffuse::rgb(0.25, 0.25, 1.0));
    let white_light = scene.add_material(Light {
        rgb: Three::new(1.0, 1.0, 1.0),
        power: 5.0,
    });

    // lights
    scene.add_object(Sphere::unit_at(0.0, 3.0, -3.0), white_light);

    // objects
    scene.add_object(
        Prism::unit_facing_pos_z()
            .rotated_around(&Three::new(0.0, 1.0, 0.0), 45.0)
            .shifted(Three::new(0.0, -2.0, -2.0)),
        mirror,
    );

    // box
    scene.add_object(Plane::facing_pos_x().shifted_back(5.0), red); // LEFT
    scene.add_object(Plane::facing_neg_x().shifted_back(5.0), blue); // RIGHT
    scene.add_object(Plane::facing_pos_y().shifted_back(2.0), white); // BOTTOM
    scene.add_object(Plane::facing_neg_y().shifted_back(4.0), white); // TOP
    scene.add_object(Plane::facing_pos_z().shifted_back(7.0), mirror); // FRONT

    render::<PathTracer, f32, XorShiftRng>(scene, camera, 10, 1000).save("mirror-prism.png")?;

    Ok(())
}
