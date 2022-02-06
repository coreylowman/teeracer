use rand_xorshift::XorShiftRng;
use teeracer::*;

pub fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // materials
    let white_lam = scene.add_material(Lambertian::new(1.0.into()));
    let mirror = scene.add_material(Metal::new(1.0.into()));
    let red_lam = scene.add_material(Lambertian::new((1.0, 0.25, 0.25).into()));
    let blue_lam = scene.add_material(Lambertian::new((0.25, 0.25, 1.0).into()));
    let white_light = scene.add_material(DiffuseLight::new(1.0.into(), 5.0));

    // lights
    scene.add_object(Sphere::new((0.0, 3.0, -3.0), 1.0), white_light);

    // objects
    const Y: Three<f64> = Three::new(0.0, 1.0, 0.0);
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

fn main() -> Result<(), image::error::ImageError> {
    let camera = Camera {
        position: (0.0, 0.0, 5.0).into(),
        width: 800,
        height: 600,
        fov: 45.0,
    };
    let scene = build_scene();
    render::<PathTracer, XorShiftRng>(scene, camera, 25, 1000).save("mirror-prism.png")?;
    Ok(())
}
