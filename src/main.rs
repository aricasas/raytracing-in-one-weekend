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

use raytracing::hittable::{Hittable, HittableList};
use raytracing::instances::{RotateY, RotationY, Translate, Translation};
use raytracing::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use raytracing::scene::SceneBuilder;
use raytracing::surfaces::{
    AABox, BvhNode, ConstantMedium, MovingSphere, Sphere, XYRect, XZRect, YZRect,
};
use raytracing::textures::{CheckerTexture, Image, Noise, Solid};
use raytracing::Camera;
use raytracing::Color;
use raytracing::Vec3;

fn main() {
    // Scene
    const IMAGE_WIDTH: u32 = 600;
    let scene = scene13()
        .image_size(IMAGE_WIDTH)
        .samples_per_pixel(500)
        .max_depth(50)
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
    let ground_material = Lambertian::new(Solid::new(0.5, 0.5, 0.5));
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
                        let sphere_material =
                            Lambertian::new(Solid::from_color(Color::random() * Color::random()));

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

    let material2 = Lambertian::new(Solid::new(0.4, 0.2, 0.1));
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
    let ground_material = Lambertian::new(Solid::new(0.5, 0.5, 0.5));
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
                        let sphere_material =
                            Lambertian::new(Solid::from_color(Color::random() * Color::random()));

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

    let material2 = Lambertian::new(Solid::new(0.4, 0.2, 0.1));
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

    for a in -20..20 {
        for b in -20..20 {
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
                        let sphere_material =
                            Lambertian::new(Solid::from_color(Color::random() * Color::random()));

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

    let material2 = Lambertian::new(Solid::new(0.4, 0.2, 0.1));
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
fn scene5() -> SceneBuilder<BvhNode> {
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

    let perlin = Noise::new(4.0);
    let ground = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(perlin.clone()),
    );
    let sphere = Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(perlin));

    let world = BvhNode::new(ground, sphere, (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
fn scene6() -> SceneBuilder<Sphere<Lambertian<Image>>> {
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

    let earth_image = image::open("imgs/earthmap.jpg").unwrap();
    let earth_texture = Image::new(earth_image.into_rgb8());
    let globe = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Lambertian::new(earth_texture),
    );

    SceneBuilder::new(globe, camera, ASPECT_RATIO)
}
fn scene7() -> SceneBuilder<BvhNode> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(7.9, 3.0, 0.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, 1.5, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 50.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 4.0 / 3.0;

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

    // Light
    let light = DiffuseLight::new(Solid::new(10.0, 10.0, 10.0));
    world.push(Sphere::new(Vec3::new(0.0, 4.0, 2.0), 0.7, light));

    // Glass spheres
    let glass = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 1.0, glass));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(Color::new(0.0, 0.0, 0.0))
}
fn scene8() -> SceneBuilder<BvhNode> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(5.0, 3.0, 0.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 50.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 4.0 / 3.0;

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

    // Light
    // let light = DiffuseLight::new(Solid::new(10.0, 10.0, 10.0));
    // world.push(Sphere::new(Vec3::new(0.0, 4.0, 2.0), 0.7, light));

    // Glass spheres
    let glass = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, glass));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO) /* .background_color(Color::new(0.0, 0.0, 0.0)) */
}
fn scene9() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    const LOOK_AT: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 40.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 1.0;

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

    let red = Lambertian::new(Solid::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Solid::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Solid::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Solid::new(15.0, 15.0, 15.0));

    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green));
    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    world.push(XZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone()));
    world.push(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));
    world.push(XYRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white));
    world.push(XZRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(Color::new(0.0, 0.0, 0.0))
}
fn scene10() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    const LOOK_AT: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 40.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 1.0;

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

    let red = Lambertian::new(Solid::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Solid::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Solid::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Solid::new(15.0, 15.0, 15.0));

    // Walls
    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green));
    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    world.push(XZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone()));
    world.push(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));
    world.push(XYRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));

    // Cubes
    world.push(AABox::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white.clone(),
    ));
    world.push(AABox::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white,
    ));

    // Light
    world.push(XZRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(Color::new(0.0, 0.0, 0.0))
}
fn scene11() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    const LOOK_AT: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 40.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 1.0;

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

    let red = Lambertian::new(Solid::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Solid::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Solid::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Solid::new(15.0, 15.0, 15.0));

    // Walls
    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green));
    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    world.push(XZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone()));
    world.push(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));
    world.push(XYRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));

    // Cubes
    let box1 = AABox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )
    .rotate_y_by(15.0_f64.to_radians())
    .translate_by(Vec3::new(265.0, 0.0, 295.0));
    let box2 = AABox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    )
    .rotate_y_by(-18.0_f64.to_radians())
    .translate_by(Vec3::new(130.0, 0.0, 65.0));

    world.push(box1);
    world.push(box2);

    // Light
    world.push(XZRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(Color::new(0.0, 0.0, 0.0))
}
fn scene12() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    const LOOK_AT: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 40.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 1.0;

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

    let red = Lambertian::new(Solid::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Solid::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Solid::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Solid::new(15.0, 15.0, 15.0));

    // Walls
    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green));
    world.push(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    world.push(XZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone()));
    world.push(XZRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));
    world.push(XYRect::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));

    // Cubes
    let box1 = AABox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )
    .rotate_y_by(15.0_f64.to_radians())
    .translate_by(Vec3::new(265.0, 0.0, 295.0));
    let box2 = AABox::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    )
    .rotate_y_by(-18.0_f64.to_radians())
    .translate_by(Vec3::new(130.0, 0.0, 65.0));

    world.push(ConstantMedium::new(box1, Solid::new(0.0, 0.0, 0.0), 0.01));
    world.push(ConstantMedium::new(box2, Solid::new(1.0, 1.0, 1.0), 0.01));

    // Light
    world.push(XZRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(Color::new(0.0, 0.0, 0.0))
}
fn scene13() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(478.0, 278.0, -600.0);
    const LOOK_AT: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 40.0;
    const APERTURE: f64 = 0.0;
    const DIST_TO_FOCUS: f64 = 10.0;
    const ASPECT_RATIO: f64 = 1.0;

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

    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();

    //  Ground
    let mut ground_boxes = HittableList::new();
    let ground = Lambertian::new(Solid::new(0.48, 0.83, 0.53));

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let point0 = Vec3::new(-1000.0 + (w * i as f64), 0.0, -1000.0 + (w * j as f64));
            let point1 = Vec3::new(point0.x() + w, rng.gen_range(1.0..101.0), point0.z() + w);

            ground_boxes.push(AABox::new(point0, point1, ground.clone()));
        }
    }

    world.push(BvhNode::from_vec(ground_boxes.into_vec(), (0.0, 1.0)));

    let light = DiffuseLight::new(Solid::new(7.0, 7.0, 7.0));
    world.push(XZRect::new((123.0, 423.0), (147.0, 412.0), 554.0, light));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(Solid::new(0.7, 0.3, 0.1));
    world.push(MovingSphere::new(
        (center1, center2),
        50.0,
        moving_sphere_material,
        (0.0, 1.0),
    ));

    world.push(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    world.push(boundary.clone());
    world.push(ConstantMedium::new(
        boundary,
        Solid::new(0.2, 0.4, 0.9),
        0.2,
    ));
    let boundary = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    world.push(ConstantMedium::new(
        boundary,
        Solid::new(1.0, 1.0, 1.0),
        0.0001,
    ));

    let emat = Lambertian::new(Image::new(
        image::open("imgs/earthmap.jpg").unwrap().into_rgb8(),
    ));
    world.push(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = Noise::new(0.1);
    world.push(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(pertext),
    ));

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(Solid::new(0.73, 0.73, 0.73));
    for _ in 0..1000 {
        boxes2.push(Sphere::new(
            Vec3::random_min_max(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    world.push(Translate::new(
        RotateY::new(
            BvhNode::from_vec(boxes2.into_vec(), (0.0, 1.0)),
            15.0_f64.to_radians(),
        ),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let world = BvhNode::from_vec(world.into_vec(), (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(Color::new(0.0, 0.0, 0.0))
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
