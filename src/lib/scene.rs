use crate::{color, Camera, Color, Hittable};

pub struct Scene<T: Hittable> {
    world: T,
    background_color: Color,
    camera: Camera,
    image_size: (u32, u32),
    samples_per_pixel: u32,
    max_depth: u32,
}
impl<T: Hittable> Scene<T> {
    pub fn new(
        world: T,
        background_color: Color,
        camera: Camera,
        image_size: (u32, u32),
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        Self {
            world,
            background_color,
            camera,
            image_size,
            samples_per_pixel,
            max_depth,
        }
    }
    pub fn world(&self) -> &T {
        &self.world
    }
    pub fn background_color(&self) -> &Color {
        &self.background_color
    }
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn image_size(&self) -> (u32, u32) {
        self.image_size
    }
    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }
    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }
}

pub struct SceneBuilder<T: Hittable> {
    world: T,
    background_color: Option<Color>,
    camera: Camera,
    aspect_ratio: f64,
    image_size: Option<(u32, u32)>,
    samples_per_pixel: Option<u32>,
    max_depth: Option<u32>,
}

impl<T: Hittable> SceneBuilder<T> {
    pub fn new(world: T, camera: Camera, aspect_ratio: f64) -> Self {
        Self {
            world,
            background_color: None,
            camera,
            aspect_ratio,
            image_size: None,
            samples_per_pixel: None,
            max_depth: None,
        }
    }

    pub fn build(self) -> Scene<T> {
        let world = self.world;
        let background_color = self.background_color.unwrap_or(color::BLUE_SKY);
        let camera = self.camera;
        let image_size = self
            .image_size
            .unwrap_or((640, get_height(640, self.aspect_ratio)));
        let samples_per_pixel = self.samples_per_pixel.unwrap_or(50);
        let max_depth = self.max_depth.unwrap_or(50);

        Scene::new(
            world,
            background_color,
            camera,
            image_size,
            samples_per_pixel,
            max_depth,
        )
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }
    pub fn image_width(mut self, width: u32) -> Self {
        self.image_size = Some((width, get_height(width, self.aspect_ratio)));
        self
    }
    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = Some(samples_per_pixel);
        self
    }
    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = Some(max_depth);
        self
    }
}

fn get_height(width: u32, aspect_ratio: f64) -> u32 {
    (f64::from(width) / aspect_ratio).round() as u32
}
