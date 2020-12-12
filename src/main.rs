#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let g: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let b: f64 = 0.25;

            let r: u32 = (255.0 * r) as u32;
            let g: u32 = (255.0 * g) as u32;
            let b: u32 = (255.0 * b) as u32;

            println!("{} {} {}", r, g, b);
        }
    }

    eprintln!("\nDone.")
}
