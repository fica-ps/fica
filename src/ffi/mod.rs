pub mod ffi_fastica;

pub mod ffi_utils;
pub mod ffi_whitening;

// generate ffi header:
// cbindgen -o .\include\fica.h -l C
// cbindgen -o .\include\fica.h -l C++