mod linalg;
mod ray;
mod sphere;
use std::iter::{FlatMap, Map};

use image::{DynamicImage, GenericImage, Pixel, Rgba};

use crate::linalg::Vec3;
use crate::ray::{Ray, RayTransformer};
use crate::sphere::Sphere;

struct Screen;
struct World;

struct Coord<State> {
    x: f64,
    y: f64,
    state: State,
}

pub(crate) struct Scene {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) fov: f64,
    pub(crate) sphere: Sphere,
}

impl Scene {
    fn to_world(&self, coord: Coord<Screen>) -> Coord<World> {
        assert!(self.width > self.height);
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        Coord {
            x: (2.0 * ((coord.x + 0.5) / (self.width as f64)) - 1.0) * aspect_ratio * fov_adjustment,
            y: (-(2.0 * ((coord.y + 0.5) / (self.height as f64)) - 1.0)) * fov_adjustment,
            state: World,
        }
    }

    fn shoot_at(&self, coord: Coord<Screen>) -> Ray {
        let coord = self.to_world(coord);
        let direction: Vec3 = (coord.x, coord.y, -1.0).into();
        Ray::at(direction.normalized())
    }

    fn render(&self) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.shoot_at(Coord {
                    x: x as f64,
                    y: y as f64,
                    state: Screen,
                });
                let ray = self.sphere.transform(ray);
                let pixel = Rgba::from_channels(
                    (255.0 * ray.color[0]) as u8,
                    (255.0 * ray.color[1]) as u8,
                    (255.0 * ray.color[2]) as u8,
                    255,
                );
                img.put_pixel(x, y, pixel);
            }
        }
        img
    }
}

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: (0.0, 0.0, -5.0).into(),
            radius: 1.0,
            color: (0.4, 1.0, 0.4).into(),
        },
    };
    let img = scene.render();
    img.save("output.png").expect("Failed to save image.");
}
