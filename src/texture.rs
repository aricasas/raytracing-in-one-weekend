pub trait Texture: Send + Sync {
    fn value(u: f64, v: f64, p: &Vec3) -> Color;
}
