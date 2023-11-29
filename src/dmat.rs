extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::fmt::Display;

use core::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct DMat {
    vals: Box<[f64]>,
    rows: usize,
    cols: usize,
}

impl PartialEq for DMat {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            false
        } else {
            self.vals == other.vals
        }
    }
}

impl Index<usize> for DMat {
    type Output = [f64];
    fn index(&self, index: usize) -> &Self::Output {
        let starting_idx = self.cols * index;
        &self.vals[starting_idx..(starting_idx + self.cols)]
    }
}

pub struct RowIterator<'a> {
    mat: &'a DMat,
    row: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a [f64];
    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.mat.rows {
            let value = Some(&self.mat[self.row]);
            self.row += 1;
            value
        } else {
            None
        }
    }
}

impl DMat {
    #[must_use]
    pub fn row_iter(&self) -> RowIterator {
        RowIterator { mat: self, row: 0 }
    }
}

impl Display for DMat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let max_column_lengths: Vec<usize> = self
            .transpose()
            .row_iter()
            .map(|column| column.iter().map(|x| x.to_string().len()).max().unwrap())
            .collect();

        for (row_index, row) in self.row_iter().enumerate() {
            let row_string = row
                .iter()
                .enumerate()
                .map(|(column_index, n)| {
                    format!(
                        "{:^len$}",
                        n.to_string(),
                        len = max_column_lengths[column_index]
                    )
                })
                .collect::<Vec<String>>()
                .join(" ");
            let (start_char, end_char) = match row_index {
                0 if self.rows == 1 => ("[", "]"),
                0 => ("┌", "┐\n"),
                int if int == self.rows - 1 => ("└", "┘"),
                _ => ("│", "│\n"),
            };
            write!(f, "{start_char} {row_string} {end_char}")?;
        }
        Ok(())
    }
}

impl Add<&Self> for DMat {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self::Output {
        assert!(
            (self.rows, self.cols) == (rhs.rows, rhs.cols),
            "Attempted to add two matrices of different sizes"
        );
        self.mutate(|val, row, col| val + rhs[row][col]);
        self
    }
}

impl Add<Self> for DMat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Add::<&Self>::add(self, &rhs)
    }
}

impl Add for &DMat {
    type Output = DMat;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(
            (self.rows, self.cols) == (rhs.rows, rhs.cols),
            "Attempted to add two matrices of different sizes"
        );

        Self::Output::generate(self.rows, self.cols, |row, col| {
            self[row][col] + rhs[row][col]
        })
    }
}

impl AddAssign for DMat {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            vals: core::mem::take(&mut self.vals),
            rows: self.rows,
            cols: self.cols,
        } + rhs;
    }
}

impl Sub<&Self> for DMat {
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        assert!(
            (self.rows, self.cols) == (rhs.rows, rhs.cols),
            "Attempted to subtract two matrices of different sizes"
        );
        self.mutate(|val, row, col| val - rhs[row][col]);
        self
    }
}

impl Sub<Self> for DMat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Sub::<&Self>::sub(self, &rhs)
    }
}

impl Sub for &DMat {
    type Output = DMat;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            (self.rows, self.cols) == (rhs.rows, rhs.cols),
            "Attempted to subtract two matrices of different sizes"
        );

        Self::Output::generate(self.rows, self.cols, |row, col| {
            self[row][col] - rhs[row][col]
        })
    }
}

impl SubAssign for DMat {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            vals: core::mem::take(&mut self.vals),
            rows: self.rows,
            cols: self.cols,
        } - rhs;
    }
}

impl<T: Into<f64>> Mul<T> for DMat {
    type Output = DMat;
    fn mul(mut self, scalar: T) -> Self::Output {
        let scalar: f64 = scalar.into();
        self.mutate(|val, _, _| val * scalar);
        self
    }
}

impl<T: Into<f64>> Mul<T> for &DMat {
    type Output = DMat;
    fn mul(self, scalar: T) -> Self::Output {
        let scalar: f64 = scalar.into();
        self.map(|val| val * scalar)
    }
}

impl<T: Into<f64>> MulAssign<T> for DMat {
    fn mul_assign(&mut self, scalar: T) {
        *self = Self {
            vals: core::mem::take(&mut self.vals),
            rows: self.rows,
            cols: self.cols,
        } * scalar;
    }
}

impl Mul<Self> for DMat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        <&DMat as Mul>::mul(&self, &rhs)
    }
}

impl Mul<&Self> for DMat {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        <&DMat as Mul>::mul(&self, rhs)
    }
}

impl Mul for &DMat {
    type Output = DMat;
    fn mul(self, rhs: Self) -> Self::Output {
        assert!(
            self.cols == rhs.rows,
            "Attempted to multiply two non-commutative matrices"
        );

        let rhs_t = rhs.transpose();

        Self::Output::generate(self.rows, rhs.cols, |row, col| {
            let row = &self[row];
            let col = &rhs_t[col];

            row.iter()
                .enumerate()
                .fold(0.0, |acc, (index, n)| acc + n * col[index])
        })
    }
}

impl Neg for DMat {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl Neg for &DMat {
    type Output = DMat;
    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl<T: Into<f64>> Div<T> for DMat {
    type Output = DMat;
    fn div(mut self, scalar: T) -> Self::Output {
        let scalar: f64 = scalar.into();
        self.mutate(|val, _, _| val / scalar);
        self
    }
}

impl<T: Into<f64>> Div<T> for &DMat {
    type Output = DMat;
    fn div(self, scalar: T) -> Self::Output {
        let scalar: f64 = scalar.into();
        self.map(|val| val / scalar)
    }
}

impl<T: Into<f64>> DivAssign<T> for DMat {
    fn div_assign(&mut self, scalar: T) {
        *self = Self {
            vals: core::mem::take(&mut self.vals),
            rows: self.rows,
            cols: self.cols,
        } / scalar;
    }
}

impl<const R: usize, const C: usize, T: Into<f64>> From<[[T; C]; R]> for DMat {
    fn from(value: [[T; C]; R]) -> Self {
        let vec = value
            .into_iter()
            .flat_map(|row| row.map(|n| n.into()))
            .collect::<Vec<f64>>();

        Self {
            vals: vec.into_boxed_slice(),
            rows: R,
            cols: C,
        }
    }
}

impl DMat {
    #[must_use]
    pub fn zero(rows: usize, cols: usize) -> Self {
        Self {
            vals: vec![0.0; rows * cols].into_boxed_slice(),
            rows,
            cols,
        }
    }

    #[must_use]
    pub fn generate<F: Fn(usize, usize) -> f64>(rows: usize, cols: usize, f: F) -> Self {
        let vec: Vec<f64> = (0..rows * cols)
            .map(|index| f(index / cols, index % cols))
            .collect();

        Self {
            vals: vec.into_boxed_slice(),
            rows,
            cols,
        }
    }

    pub fn mutate<F: Fn(f64, usize, usize) -> f64>(&mut self, f: F) {
        for (index, val) in self.vals.iter_mut().enumerate() {
            *val = f(*val, index / self.cols, index % self.cols);
        }
    }

    #[must_use]
    pub fn identity(n: usize) -> Self {
        Self::generate(n, n, |row, col| if row == col { 1.0 } else { 0.0 })
    }

    #[must_use]
    pub fn map<F: Fn(f64) -> f64>(&self, f: F) -> Self {
        Self::generate(self.rows, self.cols, |row, col| f(self[row][col]))
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        Self::generate(self.cols, self.rows, |row, col| self[col][row])
    }

    #[must_use]
    pub fn is_diagonal(&self) -> bool {
        //non-square matrices cannot be diagonal
        if self.rows != self.cols {
            return false;
        }

        for row_index in 0..self.rows {
            for col_index in 0..self.cols {
                if (row_index != col_index) && (self[row_index][col_index] != 0.0) {
                    return false;
                }
            }
        }

        true
    }

    #[must_use]
    pub fn is_scalar_identity_multiple(&self) -> bool {
        //non-square matrices cannot be scalar identity multiples
        if self.rows != self.cols {
            return false;
        }
        *self == Self::identity(self.rows) * self[0][0]
    }

    #[must_use]
    pub fn is_orthogonal(&self) -> bool {
        //non-square matrices cannot be orthogonal
        if self.rows != self.cols {
            return false;
        }

        self.clone() * self.transpose() == Self::identity(self.rows)
    }

    #[must_use]
    pub fn is_symmetric(&self) -> bool {
        //non-square matrices cannot be symmetric
        if self.rows != self.cols {
            return false;
        }
        *self == self.transpose()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dmat_macro {
    ( $( $($e: expr),* );* ) => {
        DMat::from([ $([ $($e),* ]),* ])
    };
}

#[doc(inline)]
pub use __dmat_macro as dmat;
