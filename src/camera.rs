use crate::linalg::Vec3;
use crate::objects::Object;
use crate::ray::{CanHit, Ray};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use indicatif::ProgressBar;
use rand::prelude::{thread_rng, Rng};

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
    pub(crate) bounces: usize,
    pub(crate) samples: usize,
}

impl Into<Rgba<u8>> for Vec3<u8> {
    fn into(self) -> Rgba<u8> {
        Rgba::from_channels(self[0], self[1], self[2], 255)
    }
}

impl Camera {
    pub fn render(&self, objects: Vec<Object>) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        let progress = ProgressBar::new((self.width * self.height) as u64);
        for x in 0..self.width {
            for y in 0..self.height {
                let mut avg_color: Vec3<f64> = (0.0, 0.0, 0.0).into();
                for _ in 0..self.samples {
                    let mut ray = self.ray_through(x, y);
                    let mut hits = Vec::with_capacity(self.bounces);
                    while hits.len() < self.bounces {
                        let opt_hit = objects
                            .iter()
                            .map(|obj| obj.hit_by(ray))
                            .filter(|h| h.is_some())
                            .min()
                            .flatten();
                        hits.push(opt_hit);
                        match opt_hit.map(|hit| hit.scatter(ray)).flatten() {
                            Some(scattered_ray) => ray = scattered_ray,
                            None => break,
                        };
                    }

                    let mut color: Vec3<f64> = (0.0, 0.0, 0.0).into();
                    while let Some(opt_hit) = hits.pop() {
                        match opt_hit {
                            Some(hit) => hit.attenuate(&mut color),
                            None => color.fill(0.0),
                        }
                    }
                    avg_color += color;
                }
                avg_color = avg_color / self.samples as f64;
                let color: Vec3<u8> = avg_color.into();
                // println!("x={}, y={} c={:?} p={:?}", x, y, avg_color, color);
                img.put_pixel(x, y, color.into());
                progress.inc(1);
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
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        let mut rng = thread_rng();
        let x: f64 = coord.x + rng.gen_range(0.0..1.0);
        let y: f64 = coord.y + rng.gen_range(0.0..1.0);
        Coord {
            x: (2.0 * (x / (self.width as f64 - 1.0)) - 1.0) * aspect_ratio * fov_adjustment,
            y: (-(2.0 * (y / (self.height as f64 - 1.0)) - 1.0)) * fov_adjustment,
            state: WorldCoord,
        }
    }
}
