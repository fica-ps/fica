pub mod fastica;
pub mod utils;
pub mod whitening;
pub mod data;

// generate ffi header:
// windows cmd: cbindgen -l C -o include/c/fica.h && cbindgen -l C++ -o include/cpp/fica.h
// windows powershell: cbindgen -l C -o include/c/fica.h; cbindgen -l C++ -o include/cpp/fica.h