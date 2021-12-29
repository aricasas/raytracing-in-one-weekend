# Ray Tracing In One Weekend

![Portrait image from the book](imgs/checkered_floor.png)

This is a Ray Tracer based on the book series [Ray Tracing in One Weekend](https://raytracing.github.io/) but implemented in Rust.

It outputs a PNG image

To use it, you'll need to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. Then, just enter the main directory and run this command

```sh
cargo run
```

For maximum performance but a longer compile time, use:

```sh
RUSTFLAGS="-C target-cpu=native" cargo run --release
```

Here's a gallery of all the scenes I've programmed in 1024 width images with 300 samples per pixel. If you want to see some of these in higher quality, you can check out [this post](https://www.aricasas.com/programming/render-showcase/) in my webpage.

![](imgs/scenes/1.png)
![](imgs/scenes/2.png)
![](imgs/scenes/3.png)
![](imgs/scenes/4.png)
![](imgs/scenes/5.png)
![](imgs/scenes/6.png)
![](imgs/scenes/7.png)
![](imgs/scenes/8.png)
![](imgs/scenes/9.png)
![](imgs/scenes/10.png)
![](imgs/scenes/11.png)
![](imgs/scenes/12.png)
![](imgs/scenes/13.png)
![](imgs/scenes/14.png)
