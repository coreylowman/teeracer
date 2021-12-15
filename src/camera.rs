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

impl Camera {
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

    fn shoot_at(&self, coord: Coord<ScreenCoord>) -> Ray {
        let coord = self.to_world(coord);
        let direction: Vec3 = (coord.x, coord.y, -1.0).into();
        Ray::at(direction.normalized())
    }

    pub fn render(&self, objects: Vec<Box<dyn RayTransformer>>) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.shoot_at(Coord {
                    x: x as f64,
                    y: y as f64,
                    state: ScreenCoord,
                });
                let ray = objects.iter().fold(ray, |ray, t| t.transform(ray));
                let pixel = Rgba::from_channels(ray.color[0], ray.color[1], ray.color[2], 255);
                img.put_pixel(x, y, pixel);
            }
        }
        img
    }
}
