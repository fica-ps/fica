pub mod whitening;

pub type MatrixHandle = i64;

#[repr(C)]
pub struct SVDHandle {
    u:MatrixHandle,
    s:MatrixHandle,
    v:MatrixHandle
}

#[repr(C)]
pub enum ContrastFunctionId {
    LOGCOSH  = 0,
    KURTOSIS = 1,
    EXP      = 2,
}

// generate ffi header:
// cbindgen -o .\include\fica.h -l C
