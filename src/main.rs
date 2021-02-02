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
use std::rc::Rc;

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
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
mod utilities;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = 270;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        Rc::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    )));

    // Camera
    let camera = Camera::new();

    // Render
    let mut rng = rand::thread_rng();

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:>width$}", j, width = 6);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for s in 0..SAMPLES_PER_PIXEL {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);

                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + Ray::calculate_color(&ray, &world, MAX_DEPTH);
            }

            pixel_color.write(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
// https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics
