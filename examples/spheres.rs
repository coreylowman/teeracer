use rand_xorshift::XorShiftRng;
use teeracer::*;

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // materials
    let white_lam = scene.add_material(Lambertian {
        rgb: Three::new(1.0, 1.0, 1.0),
    });
    let green_metal = scene.add_material(Metal {
        rgb: Three::new(0.25, 1.0, 0.25),
        fuzz: None,
    });
    let red_lam = scene.add_material(Lambertian {
        rgb: Three::new(1.0, 0.25, 0.25),
    });
    let blue_lam = scene.add_material(Lambertian {
        rgb: Three::new(0.25, 0.25, 1.0),
    });
    let water = scene.add_material(Dielectric { ior: 1.33 });
    let crown_glass = scene.add_material(Dielectric { ior: 1.52 });
    let diamond = scene.add_material(Dielectric { ior: 2.417 });
    let white_light = scene.add_material(DiffuseLight {
        rgb: Three::new(1.0, 1.0, 1.0),
        power: 5.0,
    });

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

fn main() -> Result<(), image::error::ImageError> {
    let camera = Camera {
        position: (0.0, 0.0, 5.0).into(),
        width: 800,
        height: 600,
        fov: 45.0,
    };
    let scene = build_scene();
    render::<PathTracer, XorShiftRng>(scene, camera, 25, 1000).save("spheres.png")?;
    Ok(())
}
