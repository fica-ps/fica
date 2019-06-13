use arrayfire::Array;
use arrayfire::Dim4;

pub type Matrix = Array<f64>;

pub struct SVD {
    pub u: Matrix, 
    pub s: Matrix, 
    pub v: Matrix,
}

#[inline(always)]
pub fn dim(row_size: u64, col_size: u64) -> Dim4 {
    Dim4::new(&[row_size, col_size, 1, 1])
}

#[inline(always)]
pub fn empty_matrix(row_size: u64, col_size: u64) -> Matrix {
    Matrix::new_empty(dim(row_size, col_size))
}

#[inline(always)]
pub fn new_matrix(values: &[f64], row_size: u64, col_size: u64) -> Matrix {
    Matrix::new(values, dim(row_size, col_size))
}

#[inline(always)]
pub fn empty_row_vector(length: u64) -> Matrix {
    Matrix::new_empty(dim(length, 1))
}