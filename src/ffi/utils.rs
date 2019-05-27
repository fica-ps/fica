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
    svd2mat(&hsvd);
}

pub extern "C" fn set_backend(backendId: i32) {
     arrayfire::set_backend(BACKENDS[backendId as usize]);
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

pub fn svd2mat(svd: &SVDHandle) -> (Box<Matrix>, Box<Matrix>, Box<Matrix>) {
    (from_handle(svd.u),
     from_handle(svd.s),
     from_handle(svd.v))
}

