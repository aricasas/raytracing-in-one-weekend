#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]

mod scenes;

fn main() {
    // Scene
    let scene = scenes::scene3()
        .image_width(1024)
        .samples_per_pixel(300)
        .max_depth(10)
        .build();

    // Render
    let start_time = std::time::Instant::now();

    let rendered_image = raytracing::render_chunked(&scene);

    let render_duration = start_time.elapsed();

    eprintln!(
        "Done. Rendering took {}",
        get_elapsed_time_message(render_duration)
    );

    // Output image
    rendered_image.save("out.png").unwrap();
}

fn get_elapsed_time_message(start_time: std::time::Duration) -> String {
    let mut seconds_passed = start_time.as_secs();

    let hours_passed = seconds_passed / 3600;
    seconds_passed %= 3600;

    let minutes_passed = seconds_passed / 60;
    seconds_passed %= 60;

    let hours_passed = if hours_passed > 0 {
        format!("{} hours, ", hours_passed)
    } else {
        String::new()
    };
    let minutes_passed = if minutes_passed > 0 {
        format!("{} minutes, and ", minutes_passed)
    } else {
        String::new()
    };

    format!(
        "{}{}{}.{:0>3} seconds.",
        hours_passed,
        minutes_passed,
        seconds_passed,
        start_time.subsec_millis()
    )
}
