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

impl<const R: usize, const C: usize> Display for Mat<R,C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_column_lengths = self.transpose().map(|column| {
            column.map(|x| x.to_string().len()).iter().copied().max().unwrap()
        });

        for (row_index, row) in self.iter().enumerate() {
            let row_string = row.iter().enumerate()
            .map(|(column_index, column)| {
                format!("{:^len$}", column.to_string(), len = max_column_lengths[column_index])
            }).collect::<Vec<String>>().join(" ");
            match row_index {
                0 if R == 1 => write!(f, "[ {} ]\n", row_string)?,
                0 => write!(f, "┌ {} ┐\n", row_string)?,
                int if int == R-1 => write!(f, "└ {} ┘\n", row_string)?,
                _ => write!(f, "│ {} │\n", row_string)?
            }
        }

        Ok(())
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
    let mut m1 = Mat::<3,3>::identity();
    m1.0[1][1] = 123.234;
    println!("{}", m1);
}
