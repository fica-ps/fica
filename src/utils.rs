use super::data::Matrix;
use arrayfire::*;

pub fn normalize(matrix: &Matrix) -> Matrix {
    
    let matrix_dims = matrix.dims();
    
    let ini: Matrix = Matrix::new_empty(matrix_dims);
    
    let reducer = |mat, idx| { 
            let col_i     = col(matrix, idx);
            let col_norm  = norm(&col_i, NormType::VECTOR_1, 0.0, 0.0);
            let new_col   = div(&col_i, &col_norm, true);
            set_col(&mat, &new_col, idx) 
    };

    (0..matrix_dims.get()[0]).fold(ini, reducer)
}