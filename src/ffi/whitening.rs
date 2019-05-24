use crate::ffi::*;
use crate::whitening;

#[no_mangle]
pub extern "C" fn normalized_svd(hmatrix: MatrixHandle) -> SVDHandle {
    let (u, s, v) = whitening::normalized_svd(&hmatrix.into());
    SVDHandle {
        u: u.get(),
        s: s.get(),
        v: v.get(),
    }
}

#[no_mangle]
pub extern "C" fn rotated_data_matrix(
    hmatrix: MatrixHandle,
    h_svd_u: MatrixHandle,
) -> MatrixHandle {
    whitening::rotated_data_matrix(&hmatrix.into(), &h_svd_u.into()).get()
}

#[no_mangle]
pub extern "C" fn reduced_dimension_repr(
    hmatrix: MatrixHandle,
    h_svd_u: MatrixHandle,
    ncols: u64,
) -> MatrixHandle {
    whitening::reduced_dimension_repr(&hmatrix.into(), &h_svd_u.into(), ncols).get()
}

#[no_mangle]
pub extern "C" fn pca_whitening(hmatrix: MatrixHandle, svd_h: SVDHandle) -> MatrixHandle {

    whitening::pca_whitening(&hmatrix.into(), &svd_h.u.into(), &svd_h.s.into()).get()
}

#[no_mangle]
pub extern "C" fn zca_whitening(hmatrix: MatrixHandle, svd_h: SVDHandle) -> MatrixHandle {
    whitening::zca_whitening(&hmatrix.into(), &svd_h.u.into(), &svd_h.s.into()).get()
}

