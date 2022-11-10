use std::ops;

#[derive(Debug, Clone)]
pub struct Poly {
    co: Vec<f32>
}
impl Poly {
    pub fn new(coefv: Vec<f32>) -> Poly {
        Poly { co: coefv }
    }
    pub fn val(self, x: f32) -> f32 {
        let mut power: f32 = 1.0;
        let mut total: f32 = 0.0;
        let leng = self.co.len();
        for _iter in 0..leng {
            total += power * self.co[leng-_iter-1];
            power *= x;
        }
        total
    }
}