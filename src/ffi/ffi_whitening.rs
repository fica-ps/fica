use crate::whitening;
use super::ffi_utils::*;


#[no_mangle]
pub extern "C" fn normalized_svd(hmatrix: MatrixHandle) -> SVDHandle {
    let mat = handle2mat(hmatrix);
    let (u, s, v) = whitening::normalized_svd(&*mat);
    
    std::mem::forget(mat);
    SVDHandle {
        u: handle(u),
        s: handle(s),
        v: handle(v),
    }
}

#[no_mangle]
pub extern "C" fn rotated_data_matrix(
    hmatrix: MatrixHandle,
    h_svd_u: MatrixHandle,
) -> MatrixHandle {
    let m = handle2mat(hmatrix);
    let svd_u = handle2mat(h_svd_u);

    let r = whitening::rotated_data_matrix(&*m, &*svd_u);

    forget_many(&[m, svd_u]);
    handle(r)
}

#[no_mangle]
pub extern "C" fn reduced_dimension_repr(
    hmatrix: MatrixHandle,
    h_svd_u: MatrixHandle,
    ncols: u64,
) -> MatrixHandle {

    let m = handle2mat(hmatrix);
    let svd_u = handle2mat(h_svd_u);

    let r = whitening::reduced_dimension_repr(&*m, &*svd_u, ncols);
    forget_many(&[m,svd_u]);    
    handle(r)
}

#[no_mangle]
pub extern "C" fn pca_whitening(hmatrix: MatrixHandle, svd_h: &SVDHandle) -> MatrixHandle {
    let m = handle2mat(hmatrix);
    let (u,s) = (handle2mat(svd_h.u), handle2mat(svd_h.s));
    let r = whitening::pca_whitening(&*m, &*u, &*s);

    forget_many(&[m,u,s]);
    handle(r)
}


#[no_mangle]
pub extern "C" fn zca_whitening(hmatrix: MatrixHandle, svd_h: SVDHandle) -> MatrixHandle {
    let m = handle2mat(hmatrix);
    let (u,s) = (handle2mat(svd_h.u), handle2mat(svd_h.s));
    let r = whitening::zca_whitening(&*m, &*u, &*s);

    forget_many(&[m,u,s]);
    handle(r)
}

