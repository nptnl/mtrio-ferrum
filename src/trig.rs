use crate::ch::CCI;
use crate::ch::Comp;
use crate::alg::exp;
use crate::alg::ixp;

pub fn sin(x: Comp) -> Comp {
    (ixp(x) - ixp(x).inv()) * -0.5 * CCI
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