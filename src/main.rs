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

fn write_color(color: &Color) {
    println!(
        "{} {} {}",
        (255.0 * color.r()) as u32,
        (255.0 * color.g()) as u32,
        (255.0 * color.b()) as u32
    );
}

fn main() {
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = 270;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                f64::from(i) / f64::from(IMAGE_WIDTH - 1),
                f64::from(j) / f64::from(IMAGE_HEIGHT - 1),
                0.25,
            );

            write_color(&pixel_color);
        }
    }

    eprintln!("\nDone.");
}
