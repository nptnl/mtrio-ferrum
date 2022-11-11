use std::ops;

#[derive(Debug, Clone)]
pub struct Poly {
    co: Vec<f32>
}
impl Poly {
    pub fn new(co: Vec<f32>) -> Poly { Poly { co } }
    pub fn l(&self) -> usize { self.co.len() }
    pub fn val(self, x: f32) -> f32 {
        let mut power: f32 = 1.0;
        let mut total: f32 = 0.0;
        let leng: usize = self.co.len();
        for _iter in 0..leng {
            total += power * self.co[leng-_iter-1];
            power *= x;
        }
        total
    }
}
impl ops::Neg for Poly {
    type Output = Poly;
    fn neg(self) -> Poly {
        let mut p = self.co;
        for indx in 0..p.len() {
            p[indx] = -p[indx];
        };
        Poly { co: p }
    }
}
impl ops::Add<Poly> for Poly {
    type Output = Poly;
    fn add(self, other: Poly) -> Poly {
        let (mut bigger, smaller) = if self.l() > other.l() {
            (self.co, other.co)
        } else {
            (other.co, self.co)
        };
        for (power, addend) in smaller.iter().enumerate() {
            let power = smaller.len() - power - 1;
            bigger[power] += addend;
        }
        Poly { co: bigger }
    }
}
impl ops::Sub<Poly> for Poly {
    type Output = Poly;
    fn sub(self, other: Poly) -> Poly { self + -other }
} // oldest trick in the book