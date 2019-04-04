pub mod whitening;

pub type MatrixHandle = i64;

#[repr(C)]
pub struct SVDHandle {
    u:MatrixHandle,
    s:MatrixHandle,
    v:MatrixHandle
}

// generate ffi with "cbindgen -o .\include\fica.h -l C"
