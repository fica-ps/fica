pub mod ffi_utils;
pub mod ffi_whitening;
pub mod ffi_fastica;

// generate ffi header:
// cbindgen -o .\include\fica.h -l C