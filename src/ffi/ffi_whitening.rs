use crate::whitening;
use super::ffi_utils::*;


#[no_mangle]
pub extern "C" fn normalized_svd(hmatrix: &MatrixHandle) -> SVDHandle {
    let mat = &hmatrix.0.into();
    let (u, s, v) = whitening::normalized_svd(mat);
    SVDHandle {
        u: MatrixHandle::from(u),
        s: MatrixHandle::from(s),
        v: MatrixHandle::from(v),
    }
}

#[no_mangle]
pub extern "C" fn rotated_data_matrix(
    hmatrix: &MatrixHandle,
    h_svd_u: &MatrixHandle,
) -> MatrixHandle {
    let m = &hmatrix.0.into();
    let svd_u = &h_svd_u.0.into();

    let r = whitening::rotated_data_matrix(m, svd_u);
    MatrixHandle::from(r)
}

#[no_mangle]
pub extern "C" fn reduced_dimension_repr(
    hmatrix: &MatrixHandle,
    h_svd_u: &MatrixHandle,
    ncols: u64,
) -> MatrixHandle {

    let m = &hmatrix.0.into();
    let svd_u = &h_svd_u.0.into();

    let r = whitening::reduced_dimension_repr(m, svd_u, ncols);
    
    MatrixHandle::from(r)
}

#[no_mangle]
pub extern "C" fn pca_whitening(hmatrix: &MatrixHandle, svd_h: &SVDHandle) -> MatrixHandle {
    let m = &hmatrix.0.into();
    let (u,s) = (&svd_h.u.0.into(), &svd_h.s.0.into());
    let r = whitening::pca_whitening(m, u, s);

    MatrixHandle::from(r)
}

#[no_mangle]
pub extern "C" fn zca_whitening(hmatrix: &MatrixHandle, svd_h: &SVDHandle) -> MatrixHandle {
    let m = &hmatrix.0.into();
    let (u,s) = (&svd_h.u.0.into(), &svd_h.s.0.into());
    let r = whitening::zca_whitening(m, u, s);
    
    MatrixHandle::from(r)
}

