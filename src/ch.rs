use std::ops;
use crate::alg::{exp,ln};

#[derive(Debug, Copy, Clone)]
pub struct Comp {
    pub r: f32,
    pub i: f32,
}
impl Comp {
    pub fn new(r: f32, i: f32) -> Self { Self { r, i } }
    pub fn conj(self) -> Self {
        Self {
            r: self.r,
            i: -self.i,
        }
    }
    pub fn square(self) -> Self {
        Self {
            r: self.r*self.r - self.i*self.i,
            i: 2.0*self.r*self.i,
        }
    }
    pub fn inv(self) -> Self {
        let coef: f32 = self.r*self.r + self.i*self.i;
        Self {
            r: self.r / coef,
            i: -self.i / coef,
        }
    }
    pub fn pow(self, other: Self) -> Self {
        exp(ln(self) * other)
    }
}
impl std::cmp::PartialEq for Comp {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.i == other.i
    }
    fn ne(&self, other: &Self) -> bool {
        self.r != other.r || self.i != other.i
    }
}
impl ops::Neg for Comp {
    type Output = Comp;
    fn neg(self) -> Self {
        Self {
            r: -self.r,
            i: -self.i,
        }
    }
}
impl std::str::FromStr for Comp {
    type Err = ();
    fn from_str(slice: &str) -> Result<Comp, Self::Err> {
        let last: usize = slice.len() - 1;
        if &slice[last..last+1] == "i" {
            match slice.rfind('+') {
                Some(v) => Ok( Comp {
                    r: slice[..v].parse::<f32>().unwrap(),
                    i: slice[v+1..last].parse::<f32>().unwrap()
                } ),
                None => match slice.rfind('-') {
                    Some(v) => Ok( Comp {
                        r: slice[..v].parse::<f32>().unwrap(),
                        i: -slice[v+1..last].parse::<f32>().unwrap()
                    } ),
                    None => Err(println!("basic number parsing error")),
                },
            }
        } else {
            Ok(Comp::new(slice.parse::<f32>().unwrap(), 0.0))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Quat {
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
}
impl Quat {
    pub fn new(r: f32, i: f32, j: f32, k: f32) -> Self { Self { r, i, j, k } }
    pub fn conj(self) -> Self {
        Self {
            r: self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
    pub fn inv(self) -> Self {
        let coef: f32 = self.r*self.r + self.i*self.i + self.j*self.j + self.k*self.k;
        Self {
            r: self.r / coef,
            i: -self.i / coef,
            j: -self.j / coef,
            k: -self.k / coef,
        }
    }
    pub fn square(self) -> Self {
        self * self
    }
}
impl ops::Neg for Quat {
    type Output = Quat;
    fn neg(self) -> Self {
        Self {
            r: -self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

impl ops::Add<Comp> for f32 {
    type Output = Comp;
    fn add(self, comp: Comp) -> Comp {
        Comp::new(self+comp.r, comp.i)
    }
}
impl ops::Sub<Comp> for f32 {
    type Output = Comp;
    fn sub(self, comp: Comp) -> Comp {
        Comp::new(self-comp.r, -comp.i)
    }
}
impl ops::Mul<Comp> for f32 {
    type Output = Comp;
    fn mul(self, comp: Comp) -> Comp {
        Comp::new(self*comp.r, self*comp.i)
    }
}
impl ops::Div<Comp> for f32 {
    type Output = Comp;
    fn div(self, comp: Comp) -> Comp {
        self * comp.inv()
    }
}
impl ops::Add<Quat> for f32 {
    type Output = Quat;
    fn add(self, quat: Quat) -> Quat {
        Quat::new(self+quat.r, quat.i, quat.j, quat.k)
    }
}
impl ops::Sub<Quat> for f32 {
    type Output = Quat;
    fn sub(self, quat: Quat) -> Quat {
        Quat::new(self-quat.r, -quat.i, -quat.j, -quat.k)
    }
}
impl ops::Mul<Quat> for f32 {
    type Output = Quat;
    fn mul(self, quat: Quat) -> Quat {
        Quat::new(self*quat.r, self*quat.i, self*quat.j, self*quat.k)
    }
}
impl ops::Div<Quat> for f32 {
    type Output = Quat;
    fn div(self, quat: Quat) -> Quat {
        self * quat.inv()
    }
}

impl ops::Add<f32> for Comp {
    type Output = Comp;
    fn add(self, other: f32) -> Comp {
        Comp::new(self.r+other, self.i)
    }
}
impl ops::Sub<f32> for Comp {
    type Output = Comp;
    fn sub(self, other: f32) -> Comp {
        Comp::new(self.r-other, self.i)
    }
}
impl ops::Mul<f32> for Comp {
    type Output = Comp;
    fn mul(self, other: f32) -> Comp {
        Comp::new(self.r*other, self.i*other)
    }
}
impl ops::Div<f32> for Comp {
    type Output = Comp;
    fn div(self, other: f32) -> Comp {
        Comp::new(self.r/other, self.i/other)
    }
}

impl ops::Add<Comp> for Comp {
    type Output = Comp;
    fn add(self, other: Comp) -> Comp {
        Comp::new(self.r+other.r, self.i+other.i)
    }
}
impl ops::Sub<Comp> for Comp {
    type Output = Comp;
    fn sub(self, other: Comp) -> Comp {
        Comp::new(self.r-other.r, self.i-other.i)
    }
}
impl ops::Mul<Comp> for Comp {
    type Output = Comp;
    fn mul(self, other: Comp) -> Comp {
            Comp::new(self.r*other.r - self.i*other.i,
        self.r*other.i + self.i*other.r)
    }
}
impl ops::Div<Comp> for Comp {
    type Output = Comp;
    fn div(self, other: Comp) -> Comp {
        self * other.inv()
    }
}
impl ops::Add<Quat> for Comp {
    type Output = Quat;
    fn add(self, quat: Quat) -> Quat {
        Quat::new(self.r+quat.r, self.i+quat.i, quat.j, quat.k)
    }
}
impl ops::Sub<Quat> for Comp {
    type Output = Quat;
    fn sub(self, quat: Quat) -> Quat {
        Quat::new(self.r-quat.r, self.i-quat.i, -quat.j, -quat.k)
    }
}
impl ops::Mul<Quat> for Comp {
    type Output = Quat;
    fn mul(self, quat: Quat) -> Quat {
        Quat {
            r: self.r*quat.r - self.i*quat.i,
            i: self.r*quat.i + self.i*quat.r,
            j: self.r*quat.j - self.i*quat.k,
            k: self.r*quat.k + self.i*quat.j,
        }
    }
}
impl ops::Div<Quat> for Comp {
    type Output = Quat;
    fn div(self, quat: Quat) -> Quat {
        self * quat.inv()
    }
}

impl ops::Add<f32> for Quat {
    type Output = Quat;
    fn add(self, other: f32) -> Quat {
        Quat::new(self.r+other, self.i, self.j, self.k)
    }
}
impl ops::Sub<f32> for Quat {
    type Output = Quat;
    fn sub(self, other: f32) -> Quat {
        Quat::new(self.r-other, self.i, self.j, self.k)
    }
}
impl ops::Mul<f32> for Quat {
    type Output = Quat;
    fn mul(self, other: f32) -> Quat {
        Quat::new(self.r*other, self.i*other, self.j*other, self.k*other)
    }
}
impl ops::Div<f32> for Quat {
    type Output = Quat;
    fn div(self, other: f32) -> Quat {
        Quat::new(self.r/other, self.i/other, self.j/other, self.k/other)
    }
}
impl ops::Add<Comp> for Quat {
    type Output = Quat;
    fn add(self, comp: Comp) -> Quat {
        Quat {
            r: self.r + comp.r,
            i: self.i + comp.i,
            j: self.j,
            k: self.k,
        }
    }
}
impl ops::Sub<Comp> for Quat {
    type Output = Quat;
    fn sub(self, comp: Comp) -> Quat {
        Quat {
            r: self.r - comp.r,
            i: self.i - comp.i,
            j: self.j,
            k: self.k,
        }
    }
}
impl ops::Mul<Comp> for Quat {
    type Output = Quat;
    fn mul(self, comp: Comp) -> Quat {
        Quat {
            r: self.r*comp.r - self.i*comp.i,
            i: self.i*comp.r + self.r*comp.i,
            j: self.j*comp.r + self.k*comp.i,
            k: self.k*comp.r - self.j*comp.i,
        }
    }
}
impl ops::Div<Comp> for Quat {
    type Output = Quat;
    fn div(self, comp: Comp) -> Quat {
        self * comp.inv()
    }
}
impl ops::Add<Quat> for Quat {
    type Output = Quat;
    fn add(self, other: Quat) -> Quat {
        Quat {
            r: self.r + other.r,
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}
impl ops::Sub<Quat> for Quat {
    type Output = Quat;
    fn sub(self, other: Quat) -> Quat {
        Quat {
            r: self.r - other.r,
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}
impl ops::Mul<Quat> for Quat {
    type Output = Quat;
    fn mul(self, other: Quat) -> Quat {
        Quat {
            r: self.r*other.r - self.i*other.i - self.j*other.j - self.k*other.k,
            i: self.r*other.i + self.i*other.r + self.j*other.k - self.k*other.j,
            j: self.r*other.j - self.i*other.k + self.j*other.r + self.k*other.i,
            k: self.r*other.k + self.i*other.j - self.j*other.i + self.k*other.r,
        }
    }
}
impl ops::Div<Quat> for Quat {
    type Output = Quat;
    fn div(self, other: Quat) -> Quat {
        self * other.inv()
    }
}

impl ops::AddAssign for Comp {
    fn add_assign(&mut self, other: Comp) {
        *self = Self {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}
impl ops::SubAssign for Comp {
    fn sub_assign(&mut self, other: Comp) {
        *self = Self {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }
}
impl ops::MulAssign for Comp {
    fn mul_assign(&mut self, other: Comp) {
        *self = Self {
            r: self.r*other.r - self.i*other.i,
            i: self.r*other.i + self.i*other.r,
        }
    }
}
impl ops::DivAssign for Comp {
    fn div_assign(&mut self, other: Comp) {
        *self = *self * other.inv()
    }
}

pub static CC0: Comp = Comp { r: 0.0, i: 0.0 };
pub static CC1: Comp = Comp { r: 1.0, i: 0.0 };
pub static CCI: Comp = Comp { r: 0.0, i: 1.0 };
