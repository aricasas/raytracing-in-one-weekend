#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]

mod vec3;
use vec3::Vec3;
mod color;
use color::Color;
mod ray;
use ray::Ray;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = 270;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = 2.0 * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let LOWER_LEFT_CORNER: Vec3 =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let v = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);

            let ray = Ray::new(
                ORIGIN,
                LOWER_LEFT_CORNER + (HORIZONTAL * u) + (VERTICAL * v) - ORIGIN,
            );

            let pixel_color = ray.color();

            pixel_color.write();
        }
    }

    eprintln!("\nDone.");
}
