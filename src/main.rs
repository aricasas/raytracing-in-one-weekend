#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]
#![allow(clippy::must_use_candidate)]

mod scenes;
fn main() {
    // Scene
    let scene = scenes::scene14()
        .image_width(2560)
        .samples_per_pixel(30_000)
        .max_depth(5)
        .build();

    // Render
    let start_time = std::time::Instant::now();

    let rendered_image = raytracing::render(&scene);

    let render_duration = start_time.elapsed();

    eprintln!(
        "\nDone. Rendering took {}",
        get_elapsed_time_message(render_duration)
    );

    // Output image
    rendered_image.save("out.png").unwrap();
}

pub fn get_elapsed_time_message(start_time: std::time::Duration) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_get_elapsed_time_message() {
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(0)),
            String::from("0.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs_f32(0.001)),
            String::from("0.001 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs_f32(0.5)),
            String::from("0.500 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs_f32(0.999)),
            String::from("0.999 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(15)),
            String::from("15.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(59)),
            String::from("59.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(60)),
            String::from("1 minutes, and 0.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(61)),
            String::from("1 minutes, and 1.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(1000)),
            String::from("16 minutes, and 40.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(3599)),
            String::from("59 minutes, and 59.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(3600)),
            String::from("1 hours, 0.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(3601)),
            String::from("1 hours, 1.000 seconds.")
        );
        assert_eq!(
            get_elapsed_time_message(Duration::from_secs(50_000)),
            String::from("13 hours, 53 minutes, and 20.000 seconds.")
        );
    }
}
