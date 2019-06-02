use super::ffi_utils::*;

#[no_mangle]
pub extern "C" fn fast_ica(
    whitened_matrix: Matrix,
    n_components: u64,
    max_iter: u64,
    conv_threshold: f64,
    alpha: f64,
    cfid: u32,
) -> Matrix {

    use crate::contrast::ContrastFunctionId;
    use crate::fastica;

    let fid_enum: ContrastFunctionId = unsafe { std::mem::transmute(cfid as u8) };

    let wmat = handle2mat(whitened_matrix);

    let result = fastica::fast_ica(
        &*wmat,
        n_components,
        max_iter,
        conv_threshold,
        alpha,
        fid_enum,
    );

    handle(result)
}