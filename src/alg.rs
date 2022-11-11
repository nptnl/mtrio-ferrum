use std::ops;
use crate::ch;
use crate::ch::Comp;

#[derive(Debug, Clone)]
pub struct Poly {
    co: Vec<Comp>
}
impl Poly {
    pub fn new(co: Vec<Comp>) -> Poly { Poly { co } }
    pub fn l(&self) -> usize { self.co.len() }
    pub fn val(&self, x: Comp) -> Comp {
        let mut power: Comp = ch::CC1;
        let mut total: Comp = ch::CC0;
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
    pub fn itg(&self, c: Comp) -> Poly {
        let mut p = self.co.to_vec();
        for indx in 0..p.len() {
            p[indx] = p[indx] / (p.len() - indx) as f32;
        }
        p.push(c);
        Poly { co: p }
    }
    pub fn rootdiv(self, rt: Comp) -> Poly {
        let mut c: Comp = self.co[0];
        let mut quotient: Vec<Comp> = vec![c];
        for time in 1..self.co.len() {
            c = rt*c + self.co[time];
            quotient.push(c);
        }
        quotient.remove(self.l() - 1);
        Poly { co: quotient }
    }
    pub fn solve(self, y: Comp) -> Vec<Comp> {
        // welcome to cracked math
        let mut p: Poly = self - y;
        let mut rootvec: Vec<Comp> = Vec::new();
        while p.co.len() > 1 {
            let solution = newton(&p,ch::CC0);
            rootvec.push(solution);
            p = p.rootdiv(solution);
        };
        rootvec

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
            bigger[power] += *addend;
        }
        Poly { co: bigger }
    }
}
impl ops::Sub<Poly> for Poly {
    type Output = Poly;
    fn sub(self, other: Poly) -> Poly { self + -other }
    // oldest trick in the book
}
impl ops::Mul<Poly> for Poly {
    type Output = Poly;
    fn mul(self, other: Poly) -> Poly {
        let (p1, p2): (Vec<Comp>, Vec<Comp>) = (self.co, other.co);
        let leng: usize = p1.len() + p2.len() - 1;
        let mut product: Vec<Comp> = vec![ch::CC0; p1.len()+p2.len()-1];
        for t1 in 0..p1.len() {
            for t2 in 0..p2.len() {
                product[leng-t1-t2-1] += p1[p1.len()-t1-1] * p2[p2.len()-t2-1]
            }
        }
        Poly { co: product }
    }
}
impl ops::Add<Comp> for Poly {
    type Output = Poly;
    fn add(self, ad: Comp) -> Poly {
        let leng = &self.l();
        let mut p: Vec<Comp> = self.co;
        p[leng - 1] += ad;
        Poly { co: p }
    }
}
impl ops::Sub<Comp> for Poly {
    type Output = Poly;
    fn sub(self, ad: Comp) -> Poly {
        let leng = &self.l();
        let mut p: Vec<Comp> = self.co;
        p[leng - 1] -= ad;
        Poly { co: p }
    }
}
impl ops::Mul<Comp> for Poly {
    type Output = Poly;
    fn mul(self, scale: Comp) -> Poly {
        let mut p: Vec<Comp> = self.co;
        for indx in 0..p.len() {
            p[indx] *= scale;
        };
        Poly { co: p }
    }
}

pub fn newton(p: &Poly, y: Comp) -> Comp {
    let mut counter: usize = 0;
    let pp: &Poly = &p.dvt();
    let (mut x1, mut x2): (Comp, Comp) = (Comp::new(2.0,1.0), Comp::new(1.0,2.0));
    while (x1.r - x2.r).abs() > 0.0001 || (x2.i - x1.i).abs() > 0.0001 {
        if counter > 100 {
            x2 = x2 + 1.0f32;
            counter = 0;
        };
        x1 = x2;
        x2 -= (p.val(x1) - y) / pp.val(x1);
        counter += 1;
    }
    x2
}