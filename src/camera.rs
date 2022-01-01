use crate::linalg::Three;
use crate::ray::Ray;
use crate::tracer::PathTracer;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::Rng;

struct ScreenCoord;
struct WorldCoord;

#[allow(dead_code)]
struct Coord<State> {
    x: f64,
    y: f64,
    state: State,
}

pub struct Camera {
    pub position: Three<f64>,
    pub fov: f64,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
}

impl Into<Rgb<u8>> for Three<f64> {
    fn into(self) -> Rgb<u8> {
        Rgb([
            (self[0].clamp(0.0, 1.0) * 255.0).round() as u8,
            (self[1].clamp(0.0, 1.0) * 255.0).round() as u8,
            (self[2].clamp(0.0, 1.0) * 255.0).round() as u8,
        ])
    }
}

impl Camera {
    pub fn render<R: Rng>(&self, mut tracer: PathTracer, mut rng: R) -> RgbImage {
        let progress = ProgressBar::new((self.width * self.height * self.samples) as u64);
        progress.set_style(
            ProgressStyle::default_bar().template("{bar:40} {elapsed_precise}<{eta} {per_sec}"),
        );

        let mut colors: Vec<Three<f64>> =
            Vec::with_capacity(self.width as usize * self.height as usize);
        for _ in 0..(self.width * self.height) {
            colors.push((0.0, 0.0, 0.0).into());
        }
        for _ in 0..self.samples {
            let mut i = 0;
            for y in 0..self.height {
                for x in 0..self.width {
                    let jx = x as f64 + rng.gen_range(0.0..1.0);
                    let jy = y as f64 + rng.gen_range(0.0..1.0);
                    let ray = self.ray_through(jx, jy);
                    colors[i] += tracer.trace(ray, &mut rng);
                    progress.inc(1);
                    i += 1;
                }
            }
        }

        let mut img = RgbImage::new(self.width as u32, self.height as u32);
        for x in 0..self.width {
            for y in 0..self.height {
                let total_color = colors[y * self.width + x];
                let mean_color = total_color / self.samples as f64;
                img.put_pixel(x as u32, y as u32, mean_color.into());
            }
        }
        img
    }

    fn ray_through(&self, x: f64, y: f64) -> Ray {
        let coord = self.to_world(Coord {
            x,
            y,
            state: ScreenCoord,
        });
        let forward: Three<f64> = (0.0, 0.0, -1.0).into();
        let up: Three<f64> = (0.0, 1.0, 0.0).into();
        let right: Three<f64> = (1.0, 0.0, 0.0).into();
        let direction = (right * coord.x + up * coord.y + forward).normalized();
        Ray {
            origin: self.position,
            direction,
        }
    }

    fn to_world(&self, coord: Coord<ScreenCoord>) -> Coord<WorldCoord> {
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        Coord {
            x: (2.0 * (coord.x / self.width as f64) - 1.0) * aspect_ratio * fov_adjustment,
            y: (-(2.0 * (coord.y / self.height as f64) - 1.0)) * fov_adjustment,
            state: WorldCoord,
        }
    }
}
