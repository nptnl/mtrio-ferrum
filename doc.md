# **how the functions work and how to use them**

## **`ch::` complex and quaternion numbers**

Structs for complex numbers and quaternions - extending our set of numbers and acting as a sort of backbone from which to build a library of math.

Negation, addition, subtraction, multipication, and division are implemented with normal syntax using `std::ops`, and defined both ways between each pair of number types: `f32`, `Comp`, and `Quat`.

Create a new value with `Comp::new(r, i)` or `Quat::new(r, i, j, k)`. Each value should be of type `f32`, as these structs are designed around `f32`s specifically.

Additionally, use `.conj()` for conjugates, `.inv()` for the inverse (1/z), and `.square()` as shorthand to square.

## **`alg::` polynomial and exponential action**

A struct to encode polynomial functions as vectors. Becuase these contain vectors, make sure to make proper use of `&`borrowing when calling values of this type. You won't have to worry about that with `ch::Comp` and `ch::Quat`, because those structs consist purely of `f32` numbers. 

These polynomials can add, subtract, and multiply normally with each other and with `f32` and `Comp` types. In fact, the entire system is built on complex numbers, as the plane of complex numbers is where the Fundamental Theorem of Algebra will apply.

Use `.dvt()` to take the derivative of a polynomial and `.itg(c)` to integrate with any complex C-value. These two functions take borrowed `&Poly` inputs and simply use the power rule to generate a new polynomial.

Evalute polynomials using `.val(x)` and use `.rootdiv(rt)` to 'factor out' a known root of the polynomial using synthetic division.

To find one root, or solution, use Newton's method, an algorithm for quickly and accurately approximating a solution to `f(x) == y` using calculus. Call it with `newton(p,y)`, using the polynomial to be solved and the complex y-value it should be solved for.

Combining Newton's method of finding approximate roots with synthetic division to factor these roots out, `.solve(y)` generates a vector of all solutions to any polynomial.

`exp(x)` uses an optimized Taylor polynomial algorithm to compute `e^x`, and `ixp(x)` is shorthand for `e^ix`. `ln(x)` performs the natural logarithm for any `Comp` input, also using an optimized Taylor polynomial. All of these functions have proper range-fixing, and will still remain accurate for values outside the Taylor series convergence. For ease of use, use `log(n,x)` as shorthand for logarithms of any base.

## **`trig::` easy trigonometric functions**

This module simply contains all trig functions, nothing else.

`sin(x), cos(x), tan(x), cot(x), csc(x), sec(x)` use `ixp(x)` for standard trig values on any complex number.

`sinh(x), cosh(x), tanh(x), coth(x), csch(x), sech(x)` use `exp(x)` for hyperbolic trig values on any complex number.

`asin(x), acos(x), atan(x), acot(x), acsc(x), asec(x)` use `ln(x)` for inverse trig values for any complex numbers.