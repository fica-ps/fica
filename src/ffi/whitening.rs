use crate::ffi::*;
use crate::whitening::*;

// ************************** Interface *****************************************

#[no_mangle]
pub extern "C" fn create_matrix(values: *const f32, rows: u64, cols: u64) -> MatrixHandle {
    let values = unsafe {
        std::slice::from_raw_parts(values, (rows * cols) as usize)
    };
    matrix_from_slices(values, rows, cols)
}

#[no_mangle]
pub extern "C" fn ffi_print_matrix(hmatrix: MatrixHandle) {
    use arrayfire::print;
    use arrayfire::Array;

    let m: Array<f32> = hmatrix.into(); 
    print(&m);
}

#[no_mangle]
pub extern "C" fn ffi_normalized_svd(matrix: MatrixHandle) -> SVDHandle {
    let (u,s,v) = whitening::normalized_svd(&(matrix.into()));
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

// ***************************************************************************

fn matrix_from_slices(values: &[f32], rows: u64, cols: u64) -> MatrixHandle {
    use arrayfire::Dim4;
    use arrayfire::Array;

    let dim = Dim4::new(&[rows,cols,1,1]);
    let arr = Array::new(&values, dim);
    arr.get()
}