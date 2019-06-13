use crate::data::{Matrix as AFMatrix, SVD as AFSVD};
use std::marker::Sized;

#[repr(C)]
pub struct Matrix(u64);

#[repr(C)]
pub struct SVD {
    pub u: Matrix,
    pub s: Matrix,
    pub v: Matrix,
}

impl SVD {
    pub fn get_components(self) -> (AFMatrix, AFMatrix, AFMatrix) {
        let svd = AFSVD::from_handle(self);
        (svd.u, svd.s, svd.v)
    }
}

pub trait FFI<T> {
    fn to_handle(self) -> T;
    fn from_handle(t: T) -> Self;
}


impl FFI<Matrix> for AFMatrix {
    fn to_handle(self) -> Matrix  {
        unsafe { std::mem::transmute::<AFMatrix, Matrix>(self) }
    }

    fn from_handle(t: Matrix) -> AFMatrix {
        unsafe { std::mem::transmute::<Matrix, AFMatrix>(t) }
    }
}

impl FFI<SVD> for AFSVD {

    fn to_handle(self) -> SVD {
        SVD {
            u: self.u.to_handle(),
            s: self.s.to_handle(),
            v: self.v.to_handle(),
        }
    }

    fn from_handle(handle: SVD) -> AFSVD {
        AFSVD {
            u: AFMatrix::from_handle(handle.u),
            s: AFMatrix::from_handle(handle.s),
            v: AFMatrix::from_handle(handle.v),
        }
    }

}