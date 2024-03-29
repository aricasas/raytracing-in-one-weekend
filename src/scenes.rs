use rand::Rng;

use raytracing::color;
use raytracing::hittable::{Hittable, HittableList};
use raytracing::instances::{RotateY, RotationY, Translate, Translation};
use raytracing::materials::{Dielectric, DiffuseLight};
use raytracing::scene::SceneBuilder;
use raytracing::surfaces::{
    AABox, Bowl, BvhNode, ConstantMedium, MovingSphere, ParabolaX, Sphere, XYRect, XZRect, YZRect,
};
use raytracing::textures::{CheckerTexture, Image, Noise, Texture};
use raytracing::Camera;
use raytracing::Color;
use raytracing::Vec3;

pub fn scene1() -> SceneBuilder<impl Hittable> {
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
    let ground_material = Color::new(0.5, 0.5, 0.5).lambertian();
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
                        let sphere_material = (Color::random() * Color::random()).lambertian();

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }

                    // Metal 15% chance
                    x if x < 0.95 => {
                        let sphere_material = Color::random().metal(rng.gen_range(0.0..0.5));

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

    let material2 = Color::new(0.4, 0.2, 0.1).lambertian();
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Color::new(0.7, 0.6, 0.5).metal(0.0);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
pub fn scene2() -> SceneBuilder<impl Hittable> {
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
    let ground_material = Color::new(0.5, 0.5, 0.5).lambertian();
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
                        let sphere_material = (Color::random() * Color::random()).lambertian();

                        world.push(MovingSphere::new(
                            (center, center2),
                            0.2,
                            sphere_material,
                            (0.0, 1.0),
                        ));
                    }

                    // Metal 15% chance
                    x if x < 0.95 => {
                        let sphere_material = Color::random().metal(rng.gen_range(0.0..0.5));

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

    let material2 = Color::new(0.4, 0.2, 0.1).lambertian();
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Color::new(0.7, 0.6, 0.5).metal(0.0);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
pub fn scene3() -> SceneBuilder<impl Hittable> {
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
    let checker = CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = checker.lambertian();
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
                        let sphere_material = (Color::random() * Color::random()).lambertian();

                        world.push(Sphere::new(center, 0.2, sphere_material));
                    }

                    // Metal 15% chance
                    x if x < 0.95 => {
                        let sphere_material = Color::random().metal(rng.gen_range(0.0..0.5));

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

    let material2 = Color::new(0.4, 0.2, 0.1).lambertian();
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Color::new(0.7, 0.6, 0.5).metal(0.0);
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
pub fn scene4() -> SceneBuilder<impl Hittable> {
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

    let checker =
        CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)).lambertian();

    let sphere1 = Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, checker.clone());
    let sphere2 = Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, checker);

    let world = BvhNode::new(sphere1, sphere2, (0.0, 0.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
pub fn scene5() -> SceneBuilder<impl Hittable> {
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

    let perlin = Noise::new(4.0).lambertian();
    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, perlin.clone());
    let sphere = Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, perlin);

    let world = BvhNode::new(ground, sphere, (0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
pub fn scene6() -> SceneBuilder<impl Hittable> {
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
    let globe = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_texture.lambertian());

    SceneBuilder::new(globe, camera, ASPECT_RATIO)
}
pub fn scene7() -> SceneBuilder<impl Hittable> {
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
    let checker = CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = checker.lambertian();
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    // Light
    let light = DiffuseLight::new(Color::new(10.0, 10.0, 10.0));
    world.push(Sphere::new(Vec3::new(0.0, 4.0, 2.0), 0.7, light));

    // Glass spheres
    let glass = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 1.0, glass));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(color::BLACK)
}
pub fn scene8() -> SceneBuilder<impl Hittable> {
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
    let checker = CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = checker.lambertian();
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    // Glass spheres
    let glass = Dielectric::new(1.5);
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, glass));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
pub fn scene9() -> SceneBuilder<impl Hittable> {
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

    let red = color::RED.lambertian();
    let white = color::WHITISH.lambertian();
    let green = color::GREEN.lambertian();
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

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

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(color::BLACK)
}
pub fn scene10() -> SceneBuilder<impl Hittable> {
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

    let red = color::RED.lambertian();
    let white = color::WHITISH.lambertian();
    let green = color::GREEN.lambertian();
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

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

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(color::BLACK)
}
pub fn scene11() -> SceneBuilder<impl Hittable> {
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

    let red = color::RED.lambertian();
    let white = color::WHITISH.lambertian();
    let green = color::GREEN.lambertian();
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

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

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(color::BLACK)
}
pub fn scene12() -> SceneBuilder<impl Hittable> {
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

    let red = color::RED.lambertian();
    let white = color::WHITISH.lambertian();
    let green = color::GREEN.lambertian();
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

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

    world.push(ConstantMedium::new(box1, Color::new(0.0, 0.0, 0.0), 0.01));
    world.push(ConstantMedium::new(box2, Color::new(1.0, 1.0, 1.0), 0.01));

    // Light
    world.push(XZRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(color::BLACK)
}
pub fn scene13() -> SceneBuilder<impl Hittable> {
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
    let ground = Color::new(0.48, 0.83, 0.53).lambertian();

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let point0 = Vec3::new(
                -1000.0 + (w * f64::from(i)),
                0.0,
                -1000.0 + (w * f64::from(j)),
            );
            let point1 = Vec3::new(point0.x() + w, rng.gen_range(1.0..101.0), point0.z() + w);

            ground_boxes.push(AABox::new(point0, point1, ground.clone()));
        }
    }

    world.push(ground_boxes.into_bvh((0.0, 1.0)));

    let light = DiffuseLight::new(Color::new(7.0, 7.0, 7.0));
    world.push(XZRect::new((123.0, 423.0), (147.0, 412.0), 554.0, light));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Color::new(0.7, 0.3, 0.1).lambertian();
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
        Color::new(0.8, 0.8, 0.9).metal(1.0),
    ));

    let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    world.push(boundary.clone());
    world.push(ConstantMedium::new(
        boundary,
        Color::new(0.2, 0.4, 0.9),
        0.2,
    ));
    let boundary = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    world.push(ConstantMedium::new(boundary, color::WHITE, 0.0001));

    let emat = Image::new(image::open("imgs/earthmap.jpg").unwrap().into_rgb8()).lambertian();
    world.push(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = Noise::new(0.1);
    world.push(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        pertext.lambertian(),
    ));

    let mut boxes2 = HittableList::new();
    let white = color::WHITISH.lambertian();
    for _ in 0..1000 {
        boxes2.push(Sphere::new(
            Vec3::random_min_max(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    world.push(Translate::new(
        RotateY::new(boxes2.into_bvh((0.0, 1.0)), 15.0_f64.to_radians()),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(color::BLACK)
}
pub fn scene14() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(-7.47, 5.0, -0.7);
    // const LOOK_FROM: Vec3 = Vec3::new(-7.47, 5.0, -4.0);
    const LOOK_AT: Vec3 = Vec3::new(15.0, 5.0, -0.7);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 67.4;
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

    let mut world = HittableList::new();

    let blue = color::BLUE.lambertian();
    let light = DiffuseLight::new(Color::new(10.0, 13.0, 10.0));
    let mirror = Color::new(0.7, 0.7, 0.7).metal(0.1);
    let checker = CheckerTexture::new(color::WHITISH, color::BLACKISH).lambertian();

    // Ground
    world.push(XZRect::new((0.0, 15.0), (-10.0, 10.0), 0.01, checker));

    // Roof
    world.push(XZRect::new((0.0, 15.0), (-10.0, 10.0), 10.0, blue.clone()));

    // Normal walls
    // world.push(YZRect::new((0.0, 10.0), (-10.0, 10.0), 0.0, white.clone()));
    world.push(YZRect::new((0.0, 10.0), (-10.0, 10.0), 12.0, blue.clone()));
    world.push(XYRect::new((0.0, 15.0), (0.0, 10.0), -10.0, blue.clone()));
    world.push(XYRect::new((0.0, 15.0), (0.0, 10.0), 10.0, blue.clone()));

    // Wall with hole
    world.push(XYRect::new((0.0, 4.5), (0.0, 10.0), 0.0, blue.clone()));
    world.push(XYRect::new((7.5, 12.0), (0.0, 10.0), 0.0, blue.clone()));
    world.push(XYRect::new((4.5, 7.5), (0.0, 3.5), 0.0, blue.clone()));
    world.push(XYRect::new((4.5, 7.5), (5.5, 10.0), 0.0, blue));

    // Glass balls
    world.push(Sphere::new(
        Vec3::new(3.0, 1.5, 3.0),
        1.5,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Vec3::new(3.0, 1.5, 7.0),
        1.5,
        Dielectric::new(1.5),
    ));

    // Mirror balls
    world.push(Sphere::new(Vec3::new(9.0, 3.0, 5.0), 3.0, mirror));

    // Fog
    let fog = ConstantMedium::new(
        AABox::new(
            Vec3::new(0.0, 0.0, -10.0),
            Vec3::new(15.0, 10.0, 10.0),
            color::WHITE.lambertian(),
        ),
        Color::new(1.0, 1.0, 1.0),
        0.02,
    );
    world.push(fog);

    // Light
    world.push(Sphere::new(Vec3::new(6.0, 10.0, 5.0), 2.0, light));

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(Color::new(0.7, 0.8, 1.0) * 0.1)
}
pub fn scene15() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(2.0, 3.0, 15.0);
    const LOOK_AT: Vec3 = Vec3::new(0.0, -2.0, 0.0);
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

    let green = color::GREEN.lambertian();
    let checker = CheckerTexture::new(color::BLACKISH, color::WHITISH).lambertian();
    let mirror = color::WHITISH.metal(0.001);
    let cakey = image::open("imgs/cakey_boss.jpg").unwrap();
    let boss = Image::new(cakey.into_rgb8()).lambertian();

    let mut world = HittableList::new();

    let parabola = ParabolaX::new((-2.5, 2.5), (-4.0, 4.0), (-0.2, 0.0, -3.0), boss);
    world.push(parabola);
    let parabola = ParabolaX::new((-2.5, 2.5), (-4.0, 2.5), (1.0, 0.0, -2.0), mirror);
    world.push(parabola);

    let sphere = Sphere::new(Vec3::new(0.0, -2.0 * -2.0 * 0.5 - 3.0, -1.0), 0.7, green);
    world.push(sphere);
    let wall = XYRect::new(
        (f64::NEG_INFINITY, f64::INFINITY),
        (f64::NEG_INFINITY, f64::INFINITY),
        -4.0,
        checker,
    );
    world.push(wall);

    SceneBuilder::new(world, camera, ASPECT_RATIO)
}
pub fn scene16() -> SceneBuilder<impl Hittable> {
    // Camera
    const LOOK_FROM: Vec3 = Vec3::new(-0.3, 1.0, 0.0);
    const LOOK_AT: Vec3 = Vec3::new(1.0, 1.46, 0.0);
    const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const FOV: f64 = 38.0;
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

    let mut world = HittableList::new();

    // Ground
    let ground_material = color::WHITISH.lambertian();
    world.push(XZRect::new(
        (-1000.0, 1000.0),
        (-1000.0, 1000.0),
        0.0,
        ground_material,
    ));

    // Glass spheres
    let glass = Dielectric::new(1.5);

    for x in 0..100 {
        for y in 0..15 {
            for z in 0..100 {
                world.push(Sphere::new(
                    Vec3::new(
                        -50.0 + f64::from(x),
                        0.5 + f64::from(y),
                        -50.0 + f64::from(z),
                    ),
                    0.2,
                    glass.clone(),
                ));
            }
        }
    }

    let world = world.into_bvh((0.0, 1.0));

    SceneBuilder::new(world, camera, ASPECT_RATIO).background_color(color::BLUE_SKY)
    // Use max depth=10
}
