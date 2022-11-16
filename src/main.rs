use crate::fractal::mandelbrot;

#[allow(dead_code)]
mod ch;
#[allow(dead_code)]
mod alg;
#[allow(dead_code)]
mod trig;
#[allow(dead_code)]
mod fractal;

#[allow(dead_code)]
fn main() {
    static BASED: bool = true;
    println!("based math has arrived");
    mandelbrot(8192, 64);
}
