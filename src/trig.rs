use crate::ch::{Comp, CCI};
use crate::alg::{exp, ixp, ln, comp_sqrt};

static PI: f32 = 3.1415926535;

pub fn sin(x: Comp) -> Comp {
    (ixp(x) - ixp(x).inv()) * Comp::new(0.0,-0.5)
}
pub fn cos(x: Comp) -> Comp {
    (ixp(x) + ixp(x).inv()) * 0.5
}
pub fn tan(x: Comp) -> Comp {
    (ixp(x) - ixp(x).inv()) / (ixp(x) + ixp(x).inv()) * -CCI
}
pub fn cot(x: Comp) -> Comp {
    (ixp(x) + ixp(x).inv()) / (ixp(x) - ixp(x).inv()) * CCI
}
pub fn sec(x: Comp) -> Comp {
    2.0 / (ixp(x) + ixp(x).inv())
}
pub fn csc(x: Comp) -> Comp {
    2.0 * CCI / (ixp(x) - ixp(x).inv())
}

pub fn sinh(x: Comp) -> Comp {
    (exp(x) - exp(x).inv()) * 0.5
}
pub fn cosh(x: Comp) -> Comp {
    (exp(x) + exp(x).inv()) * 0.5
}
pub fn tanh(x: Comp) -> Comp {
    (exp(x) - exp(x).inv()) / (exp(x) + exp(x).inv())
}
pub fn coth(x: Comp) -> Comp {
    (exp(x) + exp(x).inv()) / (exp(x) - exp(x).inv())
}
pub fn sech(x: Comp) -> Comp {
    2.0 / (exp(x) + exp(x).inv())
}
pub fn csch(x: Comp) -> Comp {
    2.0 / (exp(x) - exp(x).inv())
}

pub fn acos(x: Comp) -> Comp { -CCI * ln(x + comp_sqrt(x*x - 1.0)) }
pub fn asin(x: Comp) -> Comp { 0.5*PI - acos(x) }
pub fn asec(x: Comp) -> Comp { acos(x.inv()) }
pub fn acsc(x: Comp) -> Comp { 0.5*PI - acos(x.inv()) }
pub fn acot(x: Comp) -> Comp {Comp::new(0.0,0.5) * ln((x - CCI) / (x + CCI)) }
pub fn atan(x: Comp) -> Comp { 0.5*PI - acot(x) }