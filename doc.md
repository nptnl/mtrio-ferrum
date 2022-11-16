# **how the functions work and how to use them**

## **`ch::` complex and quaternion numbers**

Structs for complex numbers and quaternions - extending our set of numbers and acting as a sort of backbone from which to build a library of math.

Negation, addition, subtraction, multipication, and division are implemented with normal syntax using `std::ops`, and defined both ways between each pair of number types: `f32`, `Comp`, and `Quat`.

Create a new value with `Comp::new(r, i)` or `Quat::new(r, i, j, k)`. Each value should be of type `f32`, as these structs are designed around `f32`s specifically.

Additionally, use `.conj()` for conjugates, `.inv()` for the inverse (1/z), and `.square()` as shorthand to square.

## **`alg::` polynomial and exponential action**

A struct to encode polynomial functions as vectors. Becuase these contain vectors, make sure to make proper use of `&`borrowing when calling values of this type. You won't have to worry about that with `ch::Comp` and `ch::Quat`, because those structs consist purely of `f32` numbers.

