
use super::ffi_utils::*;
use crate::whitening;

#[no_mangle]
pub extern "C" fn normalized_svd(matrix: Matrix) -> SVD {
    let mat = handle2mat(matrix);
    let (u, s, v) = whitening::normalized_svd(&*mat);

    std::mem::forget(mat);
    SVD {
        u: handle(u),
        s: handle(s),
        v: handle(v),
    }
}

#[no_mangle]
pub extern "C" fn rotated_data_matrix(matrix: Matrix, svd_u: Matrix) -> Matrix {
    let m = handle2mat(matrix);
    let d_svd_u = handle2mat(svd_u);

    let r = whitening::rotated_data_matrix(&*m, &*d_svd_u);

    forget_many(&[m, d_svd_u]);
    handle(r)
}

#[no_mangle]
pub extern "C" fn reduced_dimension_repr(matrix: Matrix, svd_u: Matrix, ncols: u64) -> Matrix {
    let m = handle2mat(matrix);
    let d_svd_u = handle2mat(svd_u);

    let r = whitening::reduced_dimension_repr(&*m, &*d_svd_u, ncols);
    forget_many(&[m, d_svd_u]);
    handle(r)
}

#[no_mangle]
pub extern "C" fn pca_whitening(matrix: Matrix, svd: &SVD) -> Matrix {
    let m = handle2mat(matrix);
    let (u, s) = (handle2mat(svd.u), handle2mat(svd.s));
    let r = whitening::pca_whitening(&*m, &*u, &*s);

    forget_many(&[m, u, s]);
    handle(r)
}


#[no_mangle]
pub extern "C" fn zca_whitening(matrix: Matrix, svd: SVD) -> Matrix {
    let m = handle2mat(matrix);
    let (u, s) = (handle2mat(svd.u), handle2mat(svd.s));
    let r = whitening::zca_whitening(&*m, &*u, &*s);

    forget_many(&[m, u, s]);
    handle(r)
}

