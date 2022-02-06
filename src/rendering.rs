use crate::data::{Camera, Three};
use crate::scene::{Scene, SceneTracer};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{prelude::Rng, SeedableRng};
use rayon;
use rayon::prelude::*;

pub fn render<T, R>(scene: Scene, camera: Camera, depth: usize, num_samples: usize) -> RgbImage
where
    T: SceneTracer + Sync + Default,
    R: Rng + SeedableRng,
{
    let tracer = T::default();

    let num_pixels = camera.width * camera.height;
    let num_rays = num_pixels * num_samples;
    let pb = ProgressBar::new((camera.width * camera.height * num_samples) as u64).with_style(
        ProgressStyle::default_bar().template("{bar:40} {elapsed_precise}<{eta} {per_sec}"),
    );
    pb.set_draw_rate(1); // NOTE: indicatif drawing is bottleneck with rayon because of high speeds
    let colors = (0..num_rays)
        .into_par_iter()
        .map(|ray_idx| {
            let mut rng = R::seed_from_u64(ray_idx as u64);
            let pixel_idx = ray_idx % num_pixels;
            let y = pixel_idx / camera.width;
            let x = pixel_idx % camera.width;
            let jx = x as f64 + rng.gen_range(0.0..1.0);
            let jy = y as f64 + rng.gen_range(0.0..1.0);
            let ray = camera.ray_through(jx, jy);
            let opt_color = tracer.trace(ray, &scene, depth, &mut rng);
            pb.inc(1);
            (x, y, opt_color.unwrap_or(0.0.into()))
        })
        .fold(
            || camera.empty_image(),
            |mut colors, (x, y, color)| {
                colors[y * camera.width + x] += color;
                colors
            },
        )
        .reduce_with(|mut accum, item| {
            for i in 0..accum.len() {
                accum[i] += item[i];
            }
            accum
        })
        .unwrap();

    let mut img = RgbImage::new(camera.width as u32, camera.height as u32);
    for x in 0..camera.width {
        for y in 0..camera.height {
            let total_color = &colors[y * camera.width + x];
            let mean_color = total_color / num_samples as f64;
            img.put_pixel(x as u32, y as u32, mean_color.into());
        }
    }
    img
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
