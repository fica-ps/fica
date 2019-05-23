pub mod whitening;

// generate ffi header:
// cbindgen -o .\include\fica.h -l C

pub type MatrixHandle = i64;

#[repr(C)]
pub struct SVDHandle {
    u: MatrixHandle,
    s: MatrixHandle,
    v: MatrixHandle,
}
