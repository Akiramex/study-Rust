use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Mul},
};

use crate::Vector;
// [[1,2],[1,2],[1,2]] => [1,2,1,2,1,2]

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T> 
where 
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot_product errpr: a.col != b.row"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            data[i * b.col + j] += dot_product(row, col)?;
        }
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T> 
where 
    T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1{
                writeln!(f, ",")?;
            }
        }
        write!(f, r"}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T> 
where
    T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix({}, {}): {}", self.row, self.col, self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_print() -> Result<()> {
        let a = Matrix::new(vec![1;24], 4, 6);
        println!("{}", a);
        Ok(())
    }

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(&[1,2,3,4,5,6], 2, 3);
        let b = Matrix::new(&[1,2,3,4,5,6], 3, 2);
        let c = multiply(&a, &b)?;
        println!("{}", c);
        Ok(())
    }
}
