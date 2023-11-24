extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Display;
use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

//no_std f64 abs
fn f64_abs(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & (i64::MAX as u64))
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat<const R: usize, const C: usize>([[f64; C]; R]);

impl<const R: usize, const C: usize> Deref for Mat<R, C> {
    type Target = [[f64; C]; R];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const R: usize, const C: usize> DerefMut for Mat<R, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const R: usize, const C: usize> Display for Mat<R, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let max_column_lengths = (*self.transpose()).map(|column| {
            column
                .map(|x| x.to_string().len())
                .iter()
                .copied()
                .max()
                .unwrap()
        });

        for (row_index, row) in self.iter().enumerate() {
            let row_string = row
                .iter()
                .enumerate()
                .map(|(column_index, column)| {
                    format!(
                        "{:^len$}",
                        column.to_string(),
                        len = max_column_lengths[column_index]
                    )
                })
                .collect::<Vec<String>>()
                .join(" ");
            let (start_char, end_char) = match row_index {
                0 if R == 1 => ("[", "]"),
                0 => ("┌", "┐\n"),
                int if int == R - 1 => ("└", "┘"),
                _ => ("│", "│\n"),
            };
            write!(f, "{} {} {}", start_char, row_string, end_char)?;
        }

        Ok(())
    }
}

impl<const R: usize, const C: usize, T: Into<f64>> From<[[T; C]; R]> for Mat<R, C> {
    fn from(value: [[T; C]; R]) -> Self {
        Self(value.map(|row| row.map(|n| n.into())))
    }
}

impl<const R: usize, const C: usize> Add for Mat<R, C> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Mat::generate(|row, col| self[row][col] + rhs[row][col])
    }
}

impl<const R: usize, const C: usize> AddAssign for Mat<R, C> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const R: usize, const C: usize> Sub for Mat<R, C> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Mat::generate(|row, col| self[row][col] - rhs[row][col])
    }
}

impl<const R: usize, const C: usize> SubAssign for Mat<R, C> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const R: usize, const C: usize, T: Into<f64>> Mul<T> for Mat<R, C> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        let scalar: f64 = scalar.into();
        Self((*self).map(|row| row.map(|n| n * scalar)))
    }
}

impl<const R: usize, const C: usize, T: Into<f64>> MulAssign<T> for Mat<R, C> {
    fn mul_assign(&mut self, scalar: T) {
        *self = *self * scalar
    }
}

impl<const R: usize, const C: usize, const C2: usize> Mul<Mat<C, C2>> for Mat<R, C> {
    type Output = Mat<R, C2>;

    fn mul(self, rhs: Mat<C, C2>) -> Self::Output {
        Self::Output::generate(|r, c| {
            let row = self.row(r);
            let col = rhs.col(c);
            let dot_product = row
                .iter()
                .enumerate()
                .fold(0.0, |acc, (index, n)| acc + n * col[index]);

            dot_product
        })
    }
}

impl<const R: usize, const C: usize, T: Into<f64>> Div<T> for Mat<R, C> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        let scalar: f64 = scalar.into();
        Self((*self).map(|row| row.map(|n| n / scalar)))
    }
}

impl<const R: usize, const C: usize, T: Into<f64>> DivAssign<T> for Mat<R, C> {
    fn div_assign(&mut self, scalar: T) {
        *self = *self / scalar
    }
}

impl<const R: usize, const C: usize> Mat<R, C> {
    pub fn zero() -> Self {
        Self([[0.0; C]; R])
    }

    pub fn generate<F: Fn(usize, usize) -> f64>(f: F) -> Self {
        let mut mat = Self::zero();

        for (row_index, row) in mat.iter_mut().enumerate() {
            for (column_index, n) in row.iter_mut().enumerate() {
                *n = f(row_index, column_index);
            }
        }

        mat
    }

    pub fn map<F: Fn(f64) -> f64>(&self, f: F) -> Self {
        Self::generate(|row, col| f(self[row][col]))
    }

    pub fn transpose(&self) -> Mat<C, R> {
        Mat::<C, R>::generate(|row, column| self[column][row])
    }

    pub fn row(&self, row: usize) -> [f64; C] {
        self[row]
    }

    pub fn col(&self, col: usize) -> [f64; R] {
        (**self).map(|row| row[col])
    }
}

//functions/operations exclusive to square matrices

impl<const N: usize> Mat<N, N> {
    pub fn identity() -> Self {
        Self::generate(|row, column| if row == column { 1.0 } else { 0.0 })
    }

    pub fn is_diagonal(&self) -> bool {
        for row_index in 0..N {
            for col_index in 0..N {
                if (row_index != col_index) && (self[row_index][col_index] != 0.0) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_scalar_identity_multiple(&self) -> bool {
        *self == Self::identity() * self[0][0]
    }

    pub fn is_orthogonal(&self) -> bool {
        *self * self.transpose() == Self::identity()
    }

    pub fn is_symmetric(&self) -> bool {
        *self == self.transpose()
    }

    pub fn determinant(&self) -> f64 {
        //Algorithm to find matrix determinant using gaussian row reduction,
        //since a recursive method using submatrices cannot be implemented
        //in stable rust due to const generic expressions being unstable

        //perform gaussian elimination and store the determinant transformation coefficient
        let mut reduced = self.clone();
        let mut transformation_coefficient = 1.0;

        for k in 0..N {
            //find k-th pivot
            let pivot = reduced
                .col(k)
                .iter()
                .enumerate()
                .fold(k, |acc, (index, x)| {
                    if f64_abs(*x) > f64_abs(reduced[acc][k]) {
                        index
                    } else {
                        k
                    }
                });

            if reduced[pivot][k] == 0.0 {
                //matrix is singular
                return 0.0;
            };

            //swap rows, flip transformation coefficient
            if k != pivot {
                reduced.swap(k, pivot);
                transformation_coefficient *= -1.0;
            }

            //for all rows below pivot
            for i in k + 1..N {
                let c = -reduced[i][k] / reduced[k][k];
                //for all remaining elements in current row
                for j in k + 1..N {
                    reduced[i][j] += reduced[k][j] * c;
                }
                //fill lower triangle with 0s
                reduced[i][k] = 0.0;
            }
        }

        //return product of elements in diagonal multiplied by the transformation coefficient
        let diagonal_product = reduced
            .iter()
            .enumerate()
            .fold(1.0, |acc, (index, row)| acc * row[index]);

        diagonal_product / transformation_coefficient
    }

    pub fn inverse(&self) -> Option<Self> {
        //Algorithm to find matrix inverse, using gauss-jordan elimination
        // on the matrix augmented with its corresponding identity matrix
        //(represented here as an array of two matrices)
        let mut augmented = [self.clone(), Self::identity()];

        //perform gauss-jordan elimination
        let mut pivot = 0;
        'outer: for row in 0..N {
            if pivot >= N * 2 {
                break;
            }
            let mut i = row;

            //Find first non-zero element in lower triangle,
            //increasing pivot column if the row has no zero elements
            //and breaking the loop if none can be found

            while augmented[pivot / N][i][pivot % N] == 0.0 {
                i = i + 1;
                if i == N {
                    i = row;
                    pivot = pivot + 1;
                    if pivot == N * 2 {
                        break 'outer;
                    }
                }
            }

            //swap columns in the first half of the matrix
            if row != i {
                for col in 0..N {
                    let temp = augmented[0][row][col];
                    augmented[0][row][col] = augmented[0][i][col];
                    augmented[0][i][col] = temp;
                }
            }

            //divide current row by pivot element
            let divisor = augmented[pivot / N][row][pivot % N];
            if divisor != 0.0 {
                for col in 0..N * 2 {
                    augmented[col / N][row][col % N] /= divisor;
                }
            }

            //subtract every other row by the current one multiplied by each row's pivot element
            for j in 0..N {
                if j != row {
                    let hold = augmented[pivot / N][j][pivot % N];
                    for col in 0..N * 2 {
                        augmented[col / N][j][col % N] -= hold * augmented[col / N][row][col % N];
                    }
                }
            }

            pivot += 1;
        }

        //If the first 3 columns equal the identity matrix, the second 3 columns are the inverse matrix
        if augmented[0] == Self::identity() {
            Some(augmented[1])
        } else {
            None
        }
    }
}

impl<const N: usize> MulAssign for Mat<N, N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __mat_macro {
    ( $( $($e: expr),* );* ) => {
        Mat::from([ $([ $($e),* ]),* ])
    };
}

#[doc(inline)]
pub use __mat_macro as mat;
