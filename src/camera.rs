use crate::linalg::Vec3;
use crate::ray::{Ray, RayTransformer};
use image::{DynamicImage, GenericImage, Pixel, Rgba};

struct ScreenCoord;
struct WorldCoord;

struct Coord<State> {
    x: f64,
    y: f64,
    state: State,
}

pub(crate) struct Camera {
    pub(crate) fov: f64,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Into<Rgba<u8>> for Vec3<u8> {
    fn into(self) -> Rgba<u8> {
        Rgba::from_channels(self[0], self[1], self[2], 255)
    }
}

impl Camera {
    pub fn render(&self, objects: Vec<Box<dyn RayTransformer>>) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.ray_through(x, y);
                let ray = objects.iter().fold(ray, |ray, t| t.transform(ray));
                img.put_pixel(x, y, ray.color.into());
            }
        }
        img
    }
}

impl Camera {
    fn ray_through(&self, x: u32, y: u32) -> Ray {
        let coord = self.to_world(Coord {
            x: x as f64,
            y: y as f64,
            state: ScreenCoord,
        });
        let direction: Vec3 = (coord.x, coord.y, -1.0).into();
        Ray::at(direction.normalized())
    }

    fn to_world(&self, coord: Coord<ScreenCoord>) -> Coord<WorldCoord> {
        assert!(self.width > self.height);
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        Coord {
            x: (2.0 * ((coord.x + 0.5) / (self.width as f64)) - 1.0)
                * aspect_ratio
                * fov_adjustment,
            y: (-(2.0 * ((coord.y + 0.5) / (self.height as f64)) - 1.0)) * fov_adjustment,
            state: WorldCoord,
        }
    }
}
