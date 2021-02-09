#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]
use rand::Rng;
use rayon::prelude::*;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

mod color;
use color::Color;
mod hittable;
use hittable::HittableList;
mod ray;
use ray::Ray;
mod sphere;
use sphere::Sphere;
mod vec3;
use vec3::Vec3;
mod camera;
use camera::Camera;
mod material;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
mod utilities;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 420;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 20.0;
    const APERTURE: f64 = 0.1;
    const DIST_TO_FOCUS: f64 = 10.0;

    let camera = Camera::new(
        LOOK_FROM,
        LOOK_AT,
        VUP,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        DIST_TO_FOCUS,
    );

    // World
    let world = random_scene();

    // Render
    let start_time = std::time::Instant::now();

    let mut image_colors = vec![Color::new(0.0, 0.0, 0.0); (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];

    image_colors
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, pixel_color)| {
            let mut rng = rand::thread_rng();

            let (x, y) = get_image_coordinates(i as u32, IMAGE_WIDTH, IMAGE_HEIGHT);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);

                let ray = camera.get_ray(u, v);
                *pixel_color += Ray::calculate_color(&ray, &world, MAX_DEPTH);
            }
        });

    eprintln!(
        "\nDone. Rendering took {:.3}s",
        start_time.elapsed().as_secs_f32()
    );

    // Render
    let start_time = std::time::Instant::now();

    let image_colors = render(
        &camera,
        &world,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );

    eprintln!(
        "\nDone. Rendering took {:.3}s",
        start_time.elapsed().as_secs_f32()
    );

    // Output image
    let image_colors: Vec<u8> = image_colors
        .par_iter()
        .map(|c| c.to_writeable_ints(SAMPLES_PER_PIXEL))
        .collect::<Vec<[u8; 3]>>()
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();

    output_png(&image_colors, IMAGE_WIDTH, IMAGE_HEIGHT);
}

fn render(
    camera: &Camera,
    world: &HittableList,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Vec<Color> {
    let mut image_colors = vec![Color::new(0.0, 0.0, 0.0); (image_width * image_height) as usize];

    image_colors
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, pixel_color)| {
            let mut rng = rand::thread_rng();

            let (x, y) = get_image_coordinates(i as u32, image_width, image_height);

            for _ in 0..samples_per_pixel {
                let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(image_width - 1);
                let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(image_height - 1);

                let ray = camera.get_ray(u, v);
                *pixel_color += Ray::calculate_color(&ray, world, max_depth);
            }
        });

    image_colors
}

/// Writes the image data to a png file called 'out.png'
fn output_png(image_data: &[u8], image_width: u32, image_height: u32) {
    // code taken from https://docs.rs/png/0.16.8/png/index.html#encoder
    let path = Path::new("out.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(image_data).unwrap();
}

const fn get_image_coordinates(i: u32, width: u32, height: u32) -> (u32, u32) {
    let x = i as u32 % width;
    let y = height - (i / width);

    (x, y)
}
fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                0.9_f64.mul_add(rng.gen::<f64>(), f64::from(a)),
                0.2,
                0.9_f64.mul_add(rng.gen::<f64>(), f64::from(b)),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync> = match choose_mat {
                    // Lambertian
                    x if x < 0.8 => Arc::new(Lambertian::new(Color::random() * Color::random())),

                    // Metal
                    x if x < 0.95 => Arc::new(Metal::new(Color::random(), rng.gen_range(0.0..0.5))),

                    // Glass
                    _ => Arc::new(Dielectric::new(1.5)),
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_image_coordinates() {
        assert_eq!(get_image_coordinates(0, 20, 10), (0, 10));
        assert_eq!(get_image_coordinates(24, 20, 10), (4, 9));
        assert_eq!(get_image_coordinates(199, 20, 10), (19, 1));
    }
}
