use std::fmt::Display;
use std::ops::Deref;
#[allow(dead_code)]

#[derive(Debug)]
struct Mat<const R: usize, const C: usize>([[f64; C]; R]);



impl<const R: usize, const C: usize> Deref for Mat<R,C> {
    type Target = [[f64; C]; R];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const R: usize, const C: usize> Mat<R,C> {
    fn zero() -> Self {
        Mat([[0.0; C]; R])
    }

    fn generate<F: Fn(usize, usize) -> f64>(f: F) -> Self {
        let mut arr = *Self::zero();

        for (row_index, row) in arr.iter_mut().enumerate() {
            for (column_index, n) in row.iter_mut().enumerate() {
                *n = f(row_index, column_index);
            }
        }

        Mat(arr)
    }


    fn identity() -> Self { 
        Self::generate(|row, column| {
            if row == column { 1.0 } else { 0.0 }
        })
    }

    fn transpose(&self) -> Mat<C, R> {
        Mat::<C, R>::generate(|row, column| {
            self[column][row]
        })
    }
}



fn main() {
    let m1 = Mat::<3,3>::identity();

    dbg!(m1);
}
