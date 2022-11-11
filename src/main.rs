mod ch;
mod alg;
use alg::Poly;

fn main() {
    static BASED: bool = true;
    let f: Poly = Poly::new(vec![1.0,2.0,3.0,4.0,5.0]);
    println!("{:#?} {:#?} {:#?}", f, &f.dvt(), &f.itg(0.0))
}