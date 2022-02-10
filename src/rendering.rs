use crate::data::{Camera, Three};
use crate::scene::{Scene, SceneTracer};
use crossbeam::channel;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{prelude::Rng, SeedableRng};
use rayon;
use rayon::prelude::*;
use std::thread;

pub fn render<T, R>(scene: Scene, camera: Camera, depth: usize, num_samples: usize) -> RgbImage
where
    T: SceneTracer + Sync + Default,
    R: Rng + SeedableRng,
{
    let num_pixels = camera.width * camera.height;
    let num_rays = num_pixels * num_samples;

    let (sender, receiver) = channel::unbounded();

    let t = thread::spawn(move || {
        (0..num_rays)
            .into_par_iter()
            .map(|ray_idx| {
                let mut rng = R::seed_from_u64(ray_idx as u64);
                let pixel_idx = ray_idx % num_pixels;
                let y = pixel_idx / camera.width;
                let x = pixel_idx % camera.width;
                let jx = x as f64 + rng.gen_range(0.0..1.0);
                let jy = y as f64 + rng.gen_range(0.0..1.0);
                let ray = camera.ray_through(jx, jy);
                let opt_color = T::trace(ray, &scene, depth, &mut rng);
                (pixel_idx, opt_color.unwrap_or(Three::new(0.0, 0.0, 0.0)))
            })
            .for_each_with(sender, |s, x| s.send(x).unwrap());
    });

    let pb = ProgressBar::new(num_rays as u64).with_style(
        ProgressStyle::default_bar().template("{bar:40} {elapsed_precise}<{eta} {per_sec}"),
    );
    pb.set_draw_rate(1); // NOTE: indicatif drawing is bottleneck with rayon because of high speeds

    let mut colors = camera.empty_image();
    for (pixel_idx, color) in receiver.iter() {
        colors[pixel_idx] += color;
        pb.inc(1);
    }

    t.join().unwrap();

    let mut img = RgbImage::new(camera.width as u32, camera.height as u32);
    for x in 0..camera.width {
        for y in 0..camera.height {
            let mean_color = &colors[y * camera.width + x] / num_samples as f64;
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
