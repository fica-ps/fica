use crate::data::{Matrix as AFMatrix, SVD as AFSVD};
use super::data::*;

use arrayfire::Backend;

const BACKENDS: [Backend; 3] = [Backend::OPENCL, Backend::CUDA, Backend::CPU];

#[no_mangle]
pub extern "C" fn set_backend(backend_id: usize) {
    arrayfire::set_backend(BACKENDS[backend_id]);
}

#[no_mangle]
pub extern "C" fn free_matrix(matrix: Matrix) {
    let x = AFMatrix::from_handle(matrix);
    std::mem::drop(x);
}

#[no_mangle]
pub extern "C" fn create_matrix(values: *mut f64, cols: u64, rows: u64) -> Matrix {
    use crate::data::new_matrix;
    use std::slice;

    let mat = new_matrix(
        unsafe { slice::from_raw_parts(values, (rows * cols) as usize) },
        rows,
        cols,
    );

    mat.to_handle()
}

#[no_mangle]
pub extern "C" fn matrix_dims(hmatrix: Matrix, cols: &mut u64, rows: &mut u64) {
    let m = AFMatrix::from_handle(hmatrix);
    let (c, r) = {
        let dimobj = m.dims();
        let dimarr = dimobj.get();
        (dimarr[0], dimarr[1])
    };

    *cols = c;
    *rows = r;
    std::mem::forget(m);
}

#[no_mangle]
pub extern "C" fn copy_matrix(matrix: Matrix, to: *mut f64, size: usize) {
    let mat = AFMatrix::from_handle(matrix);
    copy_values_to_slice(&mat, to , size);
    std::mem::forget(mat);
}

#[no_mangle]
pub extern "C" fn move_matrix(matrix: Matrix, to: *mut f64, size: usize) {
    let mat = AFMatrix::from_handle(matrix);
    copy_values_to_slice(&mat, to , size);
}

#[no_mangle]
pub extern "C" fn print_matrix(matrix: Matrix) {
    use arrayfire::print;

    let mat = AFMatrix::from_handle(matrix);

    print(&mat);

    std::mem::forget(mat);
}

#[no_mangle]
pub extern "C" fn free_svd(svd: SVD) {
    AFSVD::from_handle(svd);
}

fn copy_values_to_slice(mat: &AFMatrix, to: *mut f64, size: usize) {
    let slc = unsafe { std::slice::from_raw_parts_mut(to, size) };
    mat.host(slc);
}