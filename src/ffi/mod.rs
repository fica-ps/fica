pub mod ffi_fastica;

pub mod ffi_utils;
pub mod ffi_whitening;

// generate ffi header:
// windows cmd: cbindgen -l C -o include/c/fica.h && cbindgen -l C++ -o include/cpp/fica.h
// windows powershell: cbindgen -l C -o include/c/fica.h; cbindgen -l C++ -o include/cpp/fica.h