pub mod utils;
pub mod whitening;
pub mod matrix;

use matrix::MatrixHandle;
use utils::*;

// generate ffi header:
// cbindgen -o .\include\fica.h -l C

#[no_mangle]
pub extern "C" fn fast_ica(
    whitened_matrix: &MatrixHandle,
    n_components: u64,
    max_iter: u64,
    conv_threshold: f32,
    alpha: f32,
    cfid: u32,
) -> MatrixHandle {

    use crate::contrast::ContrastFunctionId;
    use crate::fastica;

    let fid_enum: ContrastFunctionId = unsafe { std::mem::transmute(cfid as u8) };

    let wmat = from_handle(*whitened_matrix);
    
    let result = 
        fastica::fast_ica(
            &*wmat,
            n_components,
            max_iter,
            conv_threshold,
            alpha,
            fid_enum,
        );
    std::mem::forget(wmat);
    to_handle(result)
}