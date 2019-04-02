use crate::ffi::*;
use crate::ffi::whitening::SVDHandler;
use whitening::Matrix;
use arrayfire::*;



pub fn matrix_from_slices(values: &[f32], rows: u64, cols: u64) -> MatrixHandler {
    let dim = Dim4::new(&[rows,cols,1,1]);
    let arr = Array::new(&values, dim);
    arr.get()
}

pub fn to_svd_handler(u:Matrix, s:Matrix, v:Matrix) -> SVDHandler {
        let (u,s,v) = (u.get(), s.get(), v.get());
        SVDHandler { u, s, v }
}

pub fn print_mat_impl(mat: &MatrixHandler) {
    let mat: Matrix = (*mat).into();
    print(&mat)
}