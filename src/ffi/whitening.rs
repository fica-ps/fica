
use super::data::*;
use crate::data::{Matrix as AFMatrix, SVD as AFSVD};
use crate::whitening;

#[no_mangle]
pub extern "C" fn normalized_svd(matrix: Matrix) -> SVD {
    let mat    = AFMatrix::from_handle(matrix);
    let result = whitening::normalized_svd(&mat);
    
    std::mem::forget(mat);
    
    result.to_handle()
}

#[no_mangle]
pub extern "C" fn rotated_data_matrix(matrix: Matrix, svd_u: Matrix) -> Matrix {
    let m       = AFMatrix::from_handle(matrix);
    let d_svd_u = AFMatrix::from_handle(svd_u);


    let result = whitening::rotated_data_matrix(&m, &d_svd_u);
    
    std::mem::forget(m);
    std::mem::forget(d_svd_u);

    result.to_handle()
}

#[no_mangle]
pub extern "C" fn reduced_dimension_repr(matrix: Matrix, svd_u: Matrix, ncols: u64) -> Matrix {
   
    let m       = AFMatrix::from_handle(matrix);
    let d_svd_u = AFMatrix::from_handle(svd_u);

    let result = whitening::reduced_dimension_repr(&m, &d_svd_u, ncols);
    
    std::mem::forget(m);
    std::mem::forget(d_svd_u);
    
    result.to_handle()
}

#[no_mangle]
pub extern "C" fn pca_whitening(matrix: Matrix, svd: SVD) -> Matrix {
    let m      = AFMatrix::from_handle(matrix);
    let svd    = AFSVD::from_handle(svd);
    let result = whitening::pca_whitening(&m, &svd.u, &svd.s);

    std::mem::forget(svd);
    std::mem::forget(m);
    
    result.to_handle()
}


#[no_mangle]
pub extern "C" fn zca_whitening(matrix: Matrix, svd: SVD) -> Matrix {
    let m      = AFMatrix::from_handle(matrix);
    let svd    = AFSVD::from_handle(svd);
    let result = whitening::zca_whitening(&m, &svd.u, &svd.s);

    std::mem::forget(svd);
    std::mem::forget(m);
    
    result.to_handle()
}

