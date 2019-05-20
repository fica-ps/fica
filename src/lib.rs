pub mod whitening;
pub mod ffi;
pub mod fastica;
pub mod contrast;

pub mod data {
    use arrayfire::Array;
    use arrayfire::Dim4;

    pub type Matrix = Array<f32>;
    pub type SVD = (Matrix, Matrix, Matrix);
    
    #[inline(always)]
    pub fn dim(row_size: u64, col_size: u64) -> Dim4 {
        Dim4::new(&[row_size, col_size, 1, 1])
    }

    #[inline(always)]
    pub fn empty_matrix(row_size:u64, col_size: u64) -> Matrix {
        Matrix::new_empty(dim(row_size, col_size))
    }
    
    #[inline(always)]
    pub fn new_matrix(values: &[f32], row_size:u64, col_size: u64) -> Matrix {
        Matrix::new(values, dim(row_size, col_size))
    }

    #[inline(always)]
    pub fn empty_row_vector(length: u64) -> Matrix {
        Matrix::new_empty(dim(length, 1))
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn name() {
        use arrayfire::{Dim4, print, randu, max};
        let dims = Dim4::new(&[1, 1, 1, 1]);
        let a = randu::<f32>(dims);
        print(&a);
        let b = max(&a, 0);
        print(&b);
    }
    
}