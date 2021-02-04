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
    let mut rng = rand::thread_rng();

    let start_time = std::time::Instant::now();

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:>width$}", j, width = 6);
        for i in 0..IMAGE_WIDTH {
            let mut rays = Vec::new();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);

                let ray = camera.get_ray(u, v);
                rays.push(ray);
            }

            let pixel_color = rays
                .par_iter()
                .map(|r| Ray::calculate_color(r, &world, MAX_DEPTH))
                .sum::<Color>();
            pixel_color.write(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!(
        "\nDone. Rendering took {:.3}s",
        start_time.elapsed().as_secs_f32()
    );
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
