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

use rand::Rng;

use raytracing::hittable::HittableList;
use raytracing::materials::{Dielectric, Lambertian, Metal};
use raytracing::scene::SceneBuilder;
use raytracing::surfaces::{BvhNode, MovingSphere, Sphere};
use raytracing::textures::{CheckerTexture, SolidColor};
use raytracing::Camera;
use raytracing::Color;
use raytracing::Vec3;

fn main() {
    // Scene
    const IMAGE_WIDTH: u32 = 1920;
    let scene = scene3()
        .image_size(IMAGE_WIDTH)
        .samples_per_pixel(500)
        .max_depth(50)
        .build();

    // Render
    let start_time = std::time::Instant::now();

    let rendered_colors = raytracing::render(&scene);

    eprintln!(
        "\nDone. Rendering took {}",
        get_elapsed_time_message(start_time.elapsed())
    );

    // Output image
    let mut rendered_image =
        image::RgbImage::from_fn(scene.image_size().0, scene.image_size().1, |x, y| {
            rendered_colors[(y * IMAGE_WIDTH + x) as usize]
        });

    image::imageops::flip_vertical_in_place(&mut rendered_image);

    rendered_image.save("out.png").unwrap();
}

fn scene1() -> SceneBuilder<BvhNode> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 20.0;
    const APERTURE: f64 = 0.1;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let camera = Camera::new(
        LOOK_FROM,
        LOOK_AT,
        VUP,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        DIST_TO_FOCUS,
        (0.0, 1.0),
    );

    let mut world = HittableList::new();

    // Ground
    let ground_material = Lambertian::new(SolidColor::new(0.5, 0.5, 0.5));
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    // Random spheres
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                0.9_f64 * rng.gen::<f64>() + f64::from(a),
                0.2,
                0.9 * rng.gen::<f64>() + f64::from(b),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    // Lambertian 80% chance
                    x if x < 0.8 => {
                        let sphere_material = Lambertian::new(SolidColor::from_color(
                            Color::random() * Color::random(),
                        ));

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }

                    // Metal 15% chance
                    x if x < 0.95 => {
                        let sphere_material = Metal::new(Color::random(), rng.gen_range(0.0..0.5));

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }

                    // Glass 5% chance
                    _ => {
                        let sphere_material = Dielectric::new(1.5);

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }
                };
            }
        }
    }

    // Three big spheres
    let material1 = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
fn scene2() -> SceneBuilder<BvhNode> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 20.0;
    const APERTURE: f64 = 0.1;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let camera = Camera::new(
        LOOK_FROM,
        LOOK_AT,
        VUP,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        DIST_TO_FOCUS,
        (0.0, 1.0),
    );

    let mut world = HittableList::new();

    // Ground
    let ground_material = Lambertian::new(SolidColor::new(0.5, 0.5, 0.5));
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    // Random spheres
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                0.9_f64 * rng.gen::<f64>() + f64::from(a),
                0.2,
                0.9 * rng.gen::<f64>() + f64::from(b),
            );
            let center2 = center + Vec3::new(0.0, rng.gen(), 0.0);

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    // Lambertian 80% chance
                    x if x < 0.8 => {
                        let sphere_material = Lambertian::new(SolidColor::from_color(
                            Color::random() * Color::random(),
                        ));

                        world.push(MovingSphere::new(
                            (center, center2),
                            0.2,
                            sphere_material,
                            (0.0, 1.0),
                        ));
                    }

                    // Metal 15% chance
                    x if x < 0.95 => {
                        let sphere_material = Metal::new(Color::random(), rng.gen_range(0.0..0.5));

                        world.push(MovingSphere::new(
                            (center, center2),
                            0.2,
                            sphere_material,
                            (0.0, 1.0),
                        ));
                    }

                    // Glass 5% chance
                    _ => {
                        let sphere_material = Dielectric::new(1.5);

                        world.push(MovingSphere::new(
                            (center, center2),
                            0.2,
                            sphere_material,
                            (0.0, 1.0),
                        ));
                    }
                };
            }
        }
    }

    // Three big spheres
    let material1 = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
fn scene3() -> SceneBuilder<BvhNode> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 20.0;
    const APERTURE: f64 = 0.1;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let camera = Camera::new(
        LOOK_FROM,
        LOOK_AT,
        VUP,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        DIST_TO_FOCUS,
        (0.0, 1.0),
    );

    let mut world = HittableList::new();

    // Ground
    let checker = CheckerTexture::from_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::new(checker);
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    // Random spheres
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                0.9_f64 * rng.gen::<f64>() + f64::from(a),
                0.2,
                0.9 * rng.gen::<f64>() + f64::from(b),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    // Lambertian 80% chance
                    x if x < 0.8 => {
                        let sphere_material = Lambertian::new(SolidColor::from_color(
                            Color::random() * Color::random(),
                        ));

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }

                    // Metal 15% chance
                    x if x < 0.95 => {
                        let sphere_material = Metal::new(Color::random(), rng.gen_range(0.0..0.5));

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }

                    // Glass 5% chance
                    _ => {
                        let sphere_material = Dielectric::new(1.5);

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }
                };
            }
        }
    }

    // Three big spheres
    let material1 = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
fn scene4() -> SceneBuilder<BvhNode> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 20.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let camera = Camera::new(
        LOOK_FROM,
        LOOK_AT,
        VUP,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        DIST_TO_FOCUS,
        (0.0, 1.0),
    );

    let checker = CheckerTexture::from_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let mat = Lambertian::new(checker);

    let sphere1 = Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, mat.clone());
    let sphere2 = Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, mat);

    let world = BvhNode::new(sphere1, sphere2, (0.0, 0.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
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
