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
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

pub use camera::Camera;
pub use color::Color;
use hittable::Hittable;
pub use ray::Ray;
use scene::Scene;
pub use vec3::Vec3;

mod camera;
mod color;
pub mod hittable;
pub mod instances;
pub mod materials;
mod ray;
pub mod scene;
pub mod surfaces;
pub mod textures;
pub mod vec3;

pub fn render<T: Hittable>(scene: &Scene<T>) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let (image_width, image_height) = scene.image_size();

    let bar = ProgressBar::new(u64::from(image_width * image_height));
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] Rendering {percent}% done. ETA: {eta_precise}"),
    );

    let rendered_colors = (0..(image_width * image_height))
        .into_par_iter()
        .progress_with(bar)
        .map(|i| {
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            let (x, y) = get_image_coordinates(i as u32, image_width);

            for _ in 0..scene.samples_per_pixel() {
                let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(image_width - 1);
                let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(image_height - 1);

                let ray = scene.camera().get_ray(u, v);
                pixel_color +=
                    ray.calculate_color(scene.world(), scene.background_color(), scene.max_depth());
            }

            pixel_color
        })
        .map(|pixel| image::Rgb(pixel.to_writeable_ints(scene.samples_per_pixel())))
        .collect::<Vec<image::Rgb<u8>>>();

    let mut rendered_image =
        image::RgbImage::from_fn(scene.image_size().0, scene.image_size().1, |x, y| {
            rendered_colors[(y * image_width + x) as usize]
        });

    image::imageops::flip_vertical_in_place(&mut rendered_image);

    rendered_image
}

pub const fn get_image_coordinates(i: u32, width: u32) -> (u32, u32) {
    let x = i as u32 % width;
    let y = i / width;

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image_coordinates() {
        let width = 20;

        assert_eq!(get_image_coordinates(0, width,), (0, 0));
        assert_eq!(get_image_coordinates(24, width,), (4, 1));
        assert_eq!(get_image_coordinates(99, width,), (19, 4));
        assert_eq!(get_image_coordinates(100, width,), (0, 5));
        assert_eq!(get_image_coordinates(199, width,), (19, 9));

        //     y
        //
        //   9 ^  180   181   182   183   184   185   186   187   188   189   190   191   192   193   194   195   196   197   198   199
        //   8 |  160   161   162   163   164   165   166   167   168   169   170   171   172   173   174   175   176   177   178   179
        //   7 |  140   141   142   143   144   145   146   147   148   149   150   151   152   153   154   155   156   157   158   159
        //   6 |  120   121   122   123   124   125   126   127   128   129   130   131   132   133   134   135   136   137   138   139
        //   5 |  100   101   102   103   104   105   106   107   108   109   110   111   112   113   114   115   116   117   118   119
        //   4 |  80    81    82    83    84    85    86    87    88    89    90    91    92    93    94    95    96    97    98    99
        //   3 |  60    61    62    63    64    65    66    67    68    69    70    71    72    73    74    75    76    77    78    79
        //   2 |  40    41    42    43    44    45    46    47    48    49    50    51    52    53    54    55    56    57    58    59
        //   1 |  20    21    22    23    24    25    26    27    28    29    30    31    32    33    34    35    36    37    38    39
        //   0 |  0     1     2     3     4     5     6     7     8     9     10    11    12    13    14    15    16    17    18    19
        //      ------------------------------------------------------------------------------------------------------------------------> x
        //         0     1     2     3     4     5     6     7     8     9     10    11    12    13    14    15    16    17    18    19
    }
}
