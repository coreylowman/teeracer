mod camera;
mod material;
mod objects;
mod ray;
mod scene;
mod scenes;
mod three;
mod tracer;

use crate::camera::Camera;
use crate::tracer::PathTracer;
use rand_xorshift::XorShiftRng;

fn main() {
    let scene = crate::scenes::glass_prism();

    let camera = Camera {
        position: (0.0, 0.0, 5.0).into(),
        width: 800,
        height: 600,
        fov: 45.0,
        samples: 1000,
        tracer: PathTracer::new(scene, 25),
    };
    let img = camera.render::<XorShiftRng>();
    img.save("output.png").expect("Failed to save image.");
}
