#[allow(dead_code)]
mod ch;

#[allow(dead_code)]
mod alg;

#[allow(dead_code)]

fn main() {
    static BASED: bool = true;
    let f: alg::Poly = alg::Poly::new(
        vec![ch::CC1, ch::CC0, ch::Comp::new(4.0,0.0)]
    );
    println!("{:#?}", f.solve(ch::CC0));
}