mod linalg;
mod plane;
mod ray;
mod sphere;

use image::{DynamicImage, GenericImage, Pixel, Rgba};

use crate::linalg::Vec3;
use crate::plane::Plane;
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
    pub(crate) transforms: Vec<Box<dyn RayTransformer>>,
}

impl Scene {
    fn to_world(&self, coord: Coord<Screen>) -> Coord<World> {
        assert!(self.width > self.height);
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        Coord {
            x: (2.0 * ((coord.x + 0.5) / (self.width as f64)) - 1.0)
                * aspect_ratio
                * fov_adjustment,
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
                let ray = self.transforms.iter().fold(ray, |ray, t| t.transform(ray));
                let pixel = Rgba::from_channels(ray.color[0], ray.color[1], ray.color[2], 255);
                img.put_pixel(x, y, pixel);
            }
        }
        img
    }
}

fn main() {
    let mut transforms: Vec<Box<dyn RayTransformer>> = Vec::new();
    transforms.push(Box::new(Sphere {
        center: (0.0, 0.0, -5.0).into(),
        radius: 1.0,
        color: (102, 255, 102).into(),
    }));
    transforms.push(Box::new(Sphere {
        center: (3.0, 2.0, -4.0).into(),
        radius: 2.0,
        color: (255, 102, 102).into(),
    }));
    transforms.push(Box::new(Sphere {
        center: (-2.0, 2.0, -6.0).into(),
        radius: 2.0,
        color: (102, 102, 255).into(),
    }));
    transforms.push(Box::new(Plane {
        center: (0.0, -2.0, 0.0).into(),
        normal: (0.0, -1.0, 0.0).into(),
        color: (150, 75, 0).into(),
    }));
    transforms.push(Box::new(Plane {
        center: (0.0, 0.0, -20.0).into(),
        normal: (0.0, 0.0, -1.0).into(),
        color: (135, 206, 235).into(),
    }));
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        transforms,
    };
    let img = scene.render();
    img.save("output.png").expect("Failed to save image.");
}
