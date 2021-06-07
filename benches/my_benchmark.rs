use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing::textures::perlin::Perlin;
use raytracing::Vec3;

fn bench_perlin_noise(c: &mut Criterion) {
    let perlin = Perlin::new();
    let vec1 = black_box(Vec3::new(10.0, 10.0, 10.0));

    c.bench_function("perlin noise", |b| b.iter(|| perlin.noise(&vec1)));
}

fn bench_tri_interp(crit: &mut Criterion) {
    let perlin1 = Perlin::new();
    let perlin2 = Perlin::new();
    let perlin3 = Perlin::new();
    let perlin4 = Perlin::new();
    let perlin5 = Perlin::new();

    let (c1, u1, v1, w1) = black_box(perlin1.generate_c_u_v_w(&Vec3::new(10.0, 10.0, 10.0)));
    let (c2, u2, v2, w2) = black_box(perlin2.generate_c_u_v_w(&Vec3::new(-1000.0, 500.0, -233.0)));
    let (c3, u3, v3, w3) = black_box(perlin3.generate_c_u_v_w(&Vec3::random_unit_vector()));
    let (c4, u4, v4, w4) = black_box(perlin4.generate_c_u_v_w(&(Vec3::random_unit_vector() * 5.0)));
    let (c5, u5, v5, w5) = black_box(perlin5.generate_c_u_v_w(&(Vec3::random() * 300.0)));

    crit.bench_function("tri interp", |b| {
        b.iter(|| {
            (
                Perlin::trilinear_interpolation(c1, u1, v1, w1),
                Perlin::trilinear_interpolation(c2, u2, v2, w2),
                Perlin::trilinear_interpolation(c3, u3, v3, w3),
                Perlin::trilinear_interpolation(c4, u4, v4, w4),
                Perlin::trilinear_interpolation(c5, u5, v5, w5),
            )
        })
    });
}

criterion_group!(benches, /*bench_perlin_noise,*/ bench_tri_interp);
criterion_main!(benches);
