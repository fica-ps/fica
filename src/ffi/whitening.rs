use ffi::*;
use ffi::util::*;

use crate::whitening::Matrix;

#[repr(C)]
pub struct SVDHandler {
    u:MatrixHandler,
    s:MatrixHandler,
    v:MatrixHandler
}

#[no_mangle]
pub extern "C" fn create_matrix(values: *const f32, rows: u64, cols: u64) -> MatrixHandler {
    let values = unsafe {
        std::slice::from_raw_parts(values, (rows * cols) as usize)
    };
    matrix_from_slices(values, rows, cols)
}

#[no_mangle]
pub extern "C" fn print_matrix(matrix: MatrixHandler) {
    print_mat_impl(&matrix);
}

#[no_mangle]
pub extern "C" fn normalized_svd(matrix: MatrixHandler) -> SVDHandler {
    let (u,s,v) = whitening::normalized_svd(matrix);
    to_svd_handler(u,s,v)
}