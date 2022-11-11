mod ch;
mod alg;

fn main() {
    static BASED: bool = true;
    let f: alg::Poly = alg::Poly::new(vec![1.0,0.0,-4.0]);
    println!("{:#?}", f * 2.0);
}