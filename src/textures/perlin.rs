use rand::Rng;

use super::Texture;
use crate::{Color, Vec3};

#[derive(Clone)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        let mut noise = (p.z() * self.scale) + (10.0 * self.noise.turbulence(p, 7));
        noise = 0.5 * (1.0 + f64::sin(noise));

        Color::new(1.0, 1.0, 1.0) * noise
    }
}

#[derive(Clone)]
pub struct Perlin {
    ranvec: [Vec3; Self::POINT_COUNT],
    perm_x: [u8; Self::POINT_COUNT],
    perm_y: [u8; Self::POINT_COUNT],
    perm_z: [u8; Self::POINT_COUNT],
}
impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranvec: [Vec3; Self::POINT_COUNT] = [Vec3::default(); Self::POINT_COUNT];

        for vec in ranvec.iter_mut() {
            *vec = Vec3::random_min_max(-1.0, 1.0);
        }

        let mut perm_x = [0; Self::POINT_COUNT];
        let mut perm_y = [0; Self::POINT_COUNT];
        let mut perm_z = [0; Self::POINT_COUNT];

        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    fn perlin_generate_perm(p: &mut [u8; Self::POINT_COUNT]) {
        for (i, num) in (0_u8..).zip(p.iter_mut()) {
            *num = i;
        }

        Self::permute(p);
    }
    fn permute(p: &mut [u8; Self::POINT_COUNT]) {
        for i in (1..Self::POINT_COUNT).rev() {
            let target: usize = rand::thread_rng().gen_range(0..i);
            p.swap(i, target);
        }
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let (c, u, v, w) = self.generate_c_u_v_w(point);

        Self::trilinear_interpolation(c, u, v, w)
    }
    pub fn generate_c_u_v_w(&self, point: &Vec3) -> ([Vec3; 8], f64, f64, f64) {
        let mut u = point.x() - point.x().floor();
        let mut v = point.y() - point.y().floor();
        let mut w = point.z() - point.z().floor();

        let i = point.x().floor() as i64;
        let j = point.y().floor() as i64;
        let k = point.z().floor() as i64;

        let mut c: [Vec3; 8] = Default::default();

        for (index, vec) in c.iter_mut().enumerate() {
            let di = index / 4;
            let dj = (index % 4) / 2;
            let dk = index % 2;

            let mut ran_index = self.perm_x[cast_to_range(i + di as i64)];
            ran_index ^= self.perm_y[cast_to_range(j + dj as i64)];
            ran_index ^= self.perm_z[cast_to_range(k + dk as i64)];

            *vec = self.ranvec[ran_index as usize];
        }

        (c, u, v, w)
    }
    pub fn trilinear_interpolation(values_array: [Vec3; 8], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        values_array
            .iter()
            .enumerate()
            .map(|(index, val)| {
                // All 8 combinations of true/false with three values will end up being generated
                let i = index >= 4;
                let j = index % 4 >= 2;
                let k = index % 2 != 0;

                let weight_v = Vec3::new(
                    if i { u - 1.0 } else { u },
                    if j { v - 1.0 } else { v },
                    if k { w - 1.0 } else { w },
                );

                let uuu = if i { uu } else { 1.0 - uu };
                let vvv = if j { vv } else { 1.0 - vv };
                let www = if k { ww } else { 1.0 - ww };

                uuu * vvv * www * Vec3::dot(val, &weight_v)
            })
            .sum()
    }

    pub fn turbulence(&self, point: &Vec3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = point.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

const fn cast_to_range(x: i64) -> usize {
    // Only keep the last 8 bits (by bitwise AND)
    (x & 0xFF) as usize
}
