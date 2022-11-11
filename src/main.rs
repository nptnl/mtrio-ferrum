#[allow(dead_code)]
mod ch;

#[allow(dead_code)]
mod alg;

#[allow(dead_code)]
fn main() {
    static BASED: bool = true;
    let f: alg::Poly = alg::Poly::new(vec![1.0,-2.0]);
    let g: alg::Poly = alg::Poly::new(vec![1.0,-3.0]);
    println!("{:#?}", (f * g).solve(0.0))
}