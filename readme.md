# mat-rs

A `no_std` implementation of mathematical matrix types in rust

## ⚠️ WIP

This library is functional, but not done. There are a couple of things on the to-do list:

- Implement the `to_inverse` (if even possible) and `inverse` methods for DMat
- Add doc comments for types and operations
- Add one-way interoperability between DMat and Mat (possibly through a common Matrix trait)

## Features

This library crate implements two types of matrices:

- `Mat<R, C>`, statically sized matrix types using const generics
- `DMat`, a dynamically sized matrix type

Both of the matrix variants store `f64` values internally.

## Usage

Matrices can be initialised using the provided `mat![]` and `dmat![]` macros, or using some of the types' provided functions:

```rs
use mat_rs::mat::{mat, Mat};

let a = mat![
    1, 2, 3;
    4, 5, 6;
    7, 8, 9
]; //the type will be automatically inferred as Mat<3, 3>

let b = Mat::identity(3); //3x3 identity matrix

assert_eq(a*b, a);
println!("{}", a.determinant()); //0
```

## Operations

For the statically sized `Mat`s, operations are defined only on valid corresponding types.
For example, `Mat<R, C>::add` will be defined only on matrices of the same size.

This also applies to commutative matrices for multiplication, and operations that are only valid on square matrices like determinants and inverses.

Dynamic matrices will currently panic on invalid operations, so it's up to the implementer to verify sizes if necessary before performing operations.