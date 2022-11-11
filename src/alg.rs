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
    pub fn rootdiv(self, rt: f32) -> Poly {
        let mut c: f32 = self.co[0];
        let mut quotient: Vec<f32> = vec![c];
        for time in 1..self.co.len() {
            c = rt*c + self.co[time];
            quotient.push(c);
        }
        quotient.remove(self.l() - 1);
        Poly { co: quotient }
    }
    pub fn solve(self, y: f32) -> Vec<f32> {
        // welcome to cracked math
        let mut p: Poly = self - y;
        let mut rootvec: Vec<f32> = Vec::new();
        while p.co.len() > 1 {
            let solution = rnewton(&p,0.0);
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
            bigger[power] += addend;
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
impl ops::Add<f32> for Poly {
    type Output = Poly;
    fn add(self, ad: f32) -> Poly {
        let leng = &self.l();
        let mut p: Vec<f32> = self.co;
        p[leng - 1] += ad;
        Poly { co: p }
    }
}
impl ops::Sub<f32> for Poly {
    type Output = Poly;
    fn sub(self, ad: f32) -> Poly {
        let leng = &self.l();
        let mut p: Vec<f32> = self.co;
        p[leng - 1] -= ad;
        Poly { co: p }
    }
}
impl ops::Mul<f32> for Poly {
    type Output = Poly;
    fn mul(self, scale: f32) -> Poly {
        let mut p: Vec<f32> = self.co;
        for indx in 0..p.len() {
            p[indx] *= scale;
        };
        Poly { co: p }
    }
}

pub fn rnewton(p: &Poly, y: f32) -> f32 {
    let mut counter: usize = 0;
    let pp: &Poly = &p.dvt();
    let (mut x1, mut x2): (f32, f32) = (2.0, 1.0);
    while (x1 - x2) > 0.0001 || (x2 - x1) > 0.0001 {
        if counter > 100 {
            x2 += 1.0;
            counter = 0;
        };
        x1 = x2;
        x2 -= (p.val(x1) - y) / pp.val(x1);
        counter += 1;
    }
    x2
}