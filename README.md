# Ray Tracing In One Weekend

![Portrait image from the book](imgs/book_1_portrait_render.png)

This is a Ray Tracer based on the book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) but implemented in Rust.

It outputs a PNG image

To use it, you'll need to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. Then, just enter the main directory and run this command

```sh
cargo run
```

For maximum performance but a longer compile time, use:

```sh
RUSTFLAGS="-C target-cpu=native" cargo run --release
```
