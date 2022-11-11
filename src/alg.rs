use std::ops;

#[derive(Debug, Clone)]
pub struct Poly {
    co: Vec<f32>
}
impl Poly {
    pub fn new(co: Vec<f32>) -> Poly { Poly { co } }
    pub fn l(&self) -> usize { self.co.len() }
    pub fn val(&self, x: f32) -> f32 {
        let mut power: f32 = 1.0;
        let mut total: f32 = 0.0;
        let leng: usize = self.co.len();
        for _iter in 0..leng {
            total += power * self.co[leng-_iter-1];
            power *= x;
        }
        total
    }
    pub fn dvt(&self) -> Poly {
        let mut p = self.co.to_vec();
        for indx in 0..p.len() {
            p[indx] = p[indx] * (p.len() - indx - 1) as f32;
        }
        p.remove(p.len() - 1);
        Poly { co: p }
    }
    pub fn itg(&self, c: f32) -> Poly {
        let mut p = self.co.to_vec();
        for indx in 0..p.len() {
            p[indx] = p[indx] / (p.len() - indx) as f32;
        }
        p.push(c);
        Poly { co: p }
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
impl ops::Mul<Poly> for Poly {
    type Output = Poly;
    fn mul(self, other: Poly) -> Poly {
        let (p1, p2): (Vec<f32>, Vec<f32>) = (self.co, other.co);
        let leng: usize = p1.len() + p2.len() - 1;
        let mut product: Vec<f32> = vec![0.0; p1.len()+p2.len()-1];
        for t1 in 0..p1.len() {
            for t2 in 0..p2.len() {
                product[leng-t1-t2-1] += p1[p1.len()-t1-1] * p2[p2.len()-t2-1]
            }
        }
        Poly { co: product }
    }
}