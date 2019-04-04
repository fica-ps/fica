use crate::ffi::*;
use crate::whitening::*;

// ************************** Interface *****************************************

#[no_mangle]
pub extern "C" fn create_matrix(values: *const f32, rows: u64, cols: u64) -> MatrixHandle {
    use crate::data::new_matrix;
    
    new_matrix(
        unsafe { std::slice::from_raw_parts(values, (rows * cols) as usize) }, 
        rows, 
        cols
    ).get()
}

#[no_mangle]
pub extern "C" fn ffi_print_matrix(hmatrix: MatrixHandle) {
    use arrayfire::print;
    use crate::data::Matrix;

    let m: Matrix = hmatrix.into(); 
    print(&m);
}

#[no_mangle]
pub extern "C" fn ffi_normalized_svd(hmatrix: MatrixHandle) -> SVDHandle {
    let (u,s,v) = normalized_svd(&hmatrix.into());
    SVDHandle { u: u.get(), s: s.get(), v: v.get() }
}

#[no_mangle]
pub extern "C" fn ffi_rotated_data_matrix(hmatrix: MatrixHandle, h_svd_u: MatrixHandle) -> MatrixHandle {
    rotated_data_matrix(&hmatrix.into(), &h_svd_u.into()).get()
}

#[no_mangle]
pub extern "C" fn ffi_reduced_dimension_repr(hmatrix: MatrixHandle, h_svd_u: MatrixHandle, ncols: u64) -> MatrixHandle {
    reduced_dimension_repr(&hmatrix.into(), &h_svd_u.into(), ncols).get()
}

#[no_mangle]
pub extern "C" fn ffi_pca_whitening(hmatrix: MatrixHandle, h_svd_u: MatrixHandle, h_svd_s: MatrixHandle) -> MatrixHandle {
    pca_whitening(&hmatrix.into(), &h_svd_u.into(), &h_svd_s.into()).get()
}

#[no_mangle]
pub extern "C" fn ffi_zca_whitening(hmatrix: MatrixHandle, h_svd_u: MatrixHandle, h_svd_s: MatrixHandle) -> MatrixHandle {
    zca_whitening(&hmatrix.into(), &h_svd_u.into(), &h_svd_s.into()).get()
}