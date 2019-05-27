use crate::whitening;
use super::utils::*;
use super::matrix::*;


#[no_mangle]
pub extern "C" fn normalized_svd(hmatrix: &MatrixHandle) -> SVDHandle {
    let mat = *from_handle(*hmatrix);
    let (u, s, v) = whitening::normalized_svd(&mat);
    SVDHandle {
        u: to_handle(u),
        s: to_handle(s),
        v: to_handle(v),
    }
}

#[no_mangle]
pub extern "C" fn rotated_data_matrix(
    hmatrix: &MatrixHandle,
    h_svd_u: &MatrixHandle,
) -> MatrixHandle {
    let m = *from_handle(*hmatrix);
    let svd_u = *from_handle(*h_svd_u);

    let r = whitening::rotated_data_matrix(&m, &svd_u);
    to_handle(r)
}

#[no_mangle]
pub extern "C" fn reduced_dimension_repr(
    hmatrix: &MatrixHandle,
    h_svd_u: &MatrixHandle,
    ncols: u64,
) -> MatrixHandle {

    let m = *from_handle(*hmatrix);
    let svd_u = *from_handle(*h_svd_u);

    let r = whitening::reduced_dimension_repr(&m, &svd_u, ncols);
    to_handle(r)
}

#[no_mangle]
pub extern "C" fn pca_whitening(hmatrix: &MatrixHandle, svd_h: &SVDHandle) -> MatrixHandle {
    let m = *from_handle(*hmatrix);
    let (u,s,_) = svd_components(svd_h);
    let r = whitening::pca_whitening(&m, &u, &s);
    to_handle(r)
}

#[no_mangle]
pub extern "C" fn zca_whitening(hmatrix: &MatrixHandle, svd_h: &SVDHandle) -> MatrixHandle {
    let m = *from_handle(*hmatrix);
    let (u,s,_) = svd_components(svd_h);
    let r = whitening::zca_whitening(&m, &u, &s);
    to_handle(r)
}

