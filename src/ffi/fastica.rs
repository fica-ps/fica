use super::data::*;
use crate::data::Matrix as AFMatrix;
use crate::contrast::ContrastFunctionId;
use crate::fastica;

#[no_mangle]
pub extern "C" fn fast_ica(
    whitened_matrix: Matrix,
    n_components: u64,
    max_iter: u64,
    conv_threshold: f64,
    alpha: f64,
    cfid: u32,
) -> Matrix {

    let fid_enum: ContrastFunctionId = unsafe { std::mem::transmute(cfid as u8) };

    let wmat = AFMatrix::from_handle(whitened_matrix);
    
    let result = fastica::fast_ica(
        &wmat,
        n_components,
        max_iter,
        conv_threshold,
        alpha,
        fid_enum,
    );
    
    result.to_handle()
}