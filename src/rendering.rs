use crate::data::{Camera, Three};
use crate::scene::{Scene, SceneTracer};
use crossbeam::channel;
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::{cast, Float};
use rand::{prelude::Rng, SeedableRng};
use rand_distr::{uniform::SampleUniform, Distribution, Standard};
use rayon::prelude::*;
use std::ops::AddAssign;

pub fn render<T, F, R>(
    scene: Scene<F>,
    camera: Camera<F>,
    depth: usize,
    num_samples: usize,
) -> RgbImage
where
    T: SceneTracer<F> + Sync + Default,
    F: Float + SampleUniform + Send + Sync + AddAssign + 'static,
    R: Rng + SeedableRng,
    Standard: Distribution<F>,
    Three<F>: Into<Rgb<u8>>,
{
    let num_pixels = camera.width * camera.height;
    let num_rays = num_pixels * num_samples;

    let (sender, receiver) = channel::unbounded();

    let t = std::thread::spawn(move || {
        (0..num_rays)
            .into_par_iter()
            .map(|ray_idx| {
                let mut rng = R::seed_from_u64(ray_idx as u64);
                let pixel_idx = ray_idx % num_pixels;
                let y: F = cast(pixel_idx / camera.width).unwrap();
                let x: F = cast(pixel_idx % camera.width).unwrap();
                let jx = x + Standard.sample(&mut rng);
                let jy = y + Standard.sample(&mut rng);
                let ray = camera.ray_through(jx, jy);
                let opt_color = T::trace(ray, &scene, depth, &mut rng);
                (pixel_idx, opt_color.unwrap_or(Three::zeros()))
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
            let mean_color = &colors[y * camera.width + x] / cast(num_samples).unwrap();
            img.put_pixel(x as u32, y as u32, mean_color.into());
        }
    }
    img
}

impl Into<Rgb<u8>> for Three<f32> {
    fn into(self) -> Rgb<u8> {
        Rgb([
            (self.x.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.y.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.z.clamp(0.0, 1.0) * 255.0).round() as u8,
        ])
    }
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
