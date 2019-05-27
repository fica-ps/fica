use super::matrix::*;
use crate::data::Matrix;
use arrayfire::Backend;

const BACKENDS:[Backend;3] = [Backend::OPENCL, Backend::CUDA, Backend::CPU];

#[no_mangle]
pub extern "C" fn free_handle(hmatrix: MatrixHandle) {
    from_handle(hmatrix);
}

#[no_mangle]
pub extern "C" fn free_svd_handle(hsvd: SVDHandle) {
     free_handle(hsvd.u);
     free_handle(hsvd.s);
     free_handle(hsvd.v);
}

pub extern "C" fn set_backend(backend_id: i32) {
     arrayfire::set_backend(BACKENDS[backend_id as usize]);
}

pub fn from_handle(hmatrix: MatrixHandle) -> Box<Matrix> {
    unsafe { Box::from_raw(hmatrix as *mut Matrix) }
}

pub fn to_handle_boxed(matbox:Box<Matrix>) -> MatrixHandle {
    Box::into_raw(matbox) as MatrixHandle
}

pub fn to_handle(mat: Matrix) -> MatrixHandle {
    to_handle_boxed(Box::new(mat))
}

pub fn svd_components(svd: &SVDHandle) -> (Box<Matrix>, Box<Matrix>, Box<Matrix>) {
    (from_handle(svd.u),
     from_handle(svd.s),
     from_handle(svd.v))
}

