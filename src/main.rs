#![allow(dead_code)]
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Mat<const R: usize, const C: usize>([[f64; C]; R]);

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_column_lengths = self.transpose().map(|column| {
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

impl<const R: usize, const C: usize> Mat<R, C> {
    fn zero() -> Self {
        Mat([[0.0; C]; R])
    }

    fn generate<F: Fn(usize, usize) -> f64>(f: F) -> Self {
        let mut mat = Self::zero();

        for (row_index, row) in mat.iter_mut().enumerate() {
            for (column_index, n) in row.iter_mut().enumerate() {
                *n = f(row_index, column_index);
            }
        }

        mat
    }

    fn transpose(&self) -> Mat<C, R> {
        Mat::<C, R>::generate(|row, column| self[column][row])
    }
}

//functions exclusive to square matrices
impl<const N: usize> Mat<N, N> {
    fn identity() -> Self {
        Self::generate(|row, column| if row == column { 1.0 } else { 0.0 })
    }
}

fn main() {
    let m1: Mat<0, 0> = Mat::identity();
    dbg!(m1);
}
