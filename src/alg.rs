use std::ops;
use crate::ch;
use crate::ch::Comp;

#[derive(Debug, Clone)]
pub struct Poly {
    co: Vec<Comp>
}
impl Poly {
    pub fn new(co: Vec<Comp>) -> Poly { Poly { co } }
    fn l(&self) -> usize { self.co.len() }
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
    };
    x2
}
fn real_sqrt(x: f32) -> f32 {
    let (mut t1, mut t2): (f32, f32) = (2.0, 1.0);
    while (t2 - t1).abs() > 0.0001 {
        t1 = t2;
        t2 -= (t2*t2 - x) / (2.0*t2);
    }
    t2
}
pub(crate) fn comp_sqrt(x: Comp) -> Comp {
    let (mut t1, mut t2): (Comp, Comp) = (Comp::new(2.0,1.0), Comp::new(1.0,1.0));
    while (t2.r - t1.r).abs() > 0.0001 || (t2.i - t1.i).abs() > 0.0001 {
        t1 = t2;
        t2 -= (t2*t2 - x) / (2.0*t2);
    }
    t2
}
pub fn exp(x: Comp) -> Comp {
    let (mut total, mut power): (Comp, Comp) = (ch::CC0, ch::CC1);
    let extra: isize = x.r as isize;
    let x = Comp { r: x.r % 1.0, i: x.i % 6.28319 };
    for time in 1..12 {
        total += power;
        power *= x / time as f32;
    };
    if extra >= 0 { for _iter in 0..extra { total = total * 2.71828f32 }; } 
    else { for _iter in 0..-extra { total = total / 2.71828f32 }; }
    total
}
pub fn ixp(x: Comp) -> Comp { exp(ch::CCI * x) }
fn series_ln(x: Comp) -> Comp {
    let x = x - 1.0;
    let (mut total, mut neg, mut power): 
    (Comp, bool, Comp) = (ch::CC0, true, ch::CC1);
    for time in 1..16 {
        power *= x;
        neg = !neg;
        if neg { total -= power / time as f32; }
        else { total += power / time as f32; }
    };
    total
}
pub fn ln(x: Comp) -> Comp {
    
    let mut magnitude: Comp = Comp::new(real_sqrt(x.r*x.r + x.i*x.i), 0.0);
    let mut inp: Comp = x / magnitude;
    let mut neg: f32 = 1.0;
    let (mut addedr, mut addedi): (f32, f32) = (0.0, 0.0);

    if magnitude.r > 1.5 { magnitude = magnitude.inv();  neg = -1.0; }
    while magnitude.r < 0.2 { magnitude = magnitude * 2.71828; addedr += 1.0; }

    if x.i < -x.r {
        if x.i < x.r {
            inp *= ch::CCI;
        addedi = 1.5708;
        } else {
            inp *= -ch::CC1;
            addedi = -3.14159;
        }
    } else {
        if x.i > x.r {
            inp *= -ch::CCI;
            addedi = -1.5708;
        }
    }

    Comp {
        r: (series_ln(magnitude).r - addedr) * neg,
        i: series_ln(x).i - addedi,
    }
}
