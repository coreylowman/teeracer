use crate::linalg::Three;
use crate::ray::Ray;
use crate::tracer::Tracer;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{prelude::Rng, SeedableRng};

const FORWARD: Three<f64> = Three::new(0.0, 0.0, -1.0);
const UP: Three<f64> = Three::new(0.0, 1.0, 0.0);
const RIGHT: Three<f64> = Three::new(1.0, 0.0, 0.0);

pub struct Camera<T> {
    pub position: Three<f64>,
    pub fov: f64,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub tracer: T,
}

impl Into<Rgb<u8>> for Three<f64> {
    fn into(self) -> Rgb<u8> {
        Rgb([
            (self.x.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.y.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.z.clamp(0.0, 1.0) * 255.0).round() as u8,
        ])
    }
}

impl<T> Camera<T>
where
    T: Tracer,
{
    pub fn render<R: Rng + SeedableRng>(&self) -> RgbImage {
        let progress = ProgressBar::new((self.width * self.height * self.samples) as u64);
        progress.set_style(
            ProgressStyle::default_bar().template("{bar:40} {elapsed_precise}<{eta} {per_sec}"),
        );

        let mut colors: Vec<Three<f64>> = vec![(0.0, 0.0, 0.0).into(); self.width * self.height];
        let num_pixels = self.width * self.height;
        let num_rays = num_pixels * self.samples;
        (0..num_rays)
            .map(|ray_idx| {
                let mut rng = R::seed_from_u64(ray_idx as u64);
                let _sample_idx = ray_idx / num_pixels;
                let pixel_idx = ray_idx % num_pixels;
                let y = pixel_idx / self.width;
                let x = pixel_idx % self.width;
                let jx = x as f64 + rng.gen_range(0.0..1.0);
                let jy = y as f64 + rng.gen_range(0.0..1.0);
                let ray = self.ray_through(jx, jy);
                let color = self.tracer.trace(ray, &mut rng);
                progress.inc(1);
                (x, y, color)
            })
            .for_each(|(x, y, color)| {
                colors[y * self.width + x] += color;
            });

        let mut img = RgbImage::new(self.width as u32, self.height as u32);
        for x in 0..self.width {
            for y in 0..self.height {
                let total_color = &colors[y * self.width + x];
                let mean_color = total_color / self.samples as f64;
                img.put_pixel(x as u32, y as u32, mean_color.into());
            }
        }
        img
    }

    fn ray_through(&self, x: f64, y: f64) -> Ray {
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);
        let x = (2.0 * (x / self.width as f64) - 1.0) * aspect_ratio * fov_adjustment;
        let y = (-(2.0 * (y / self.height as f64) - 1.0)) * fov_adjustment;
        let direction = (RIGHT * x + UP * y + FORWARD).normalized();
        Ray {
            origin: self.position,
            direction,
        }
    }
}
