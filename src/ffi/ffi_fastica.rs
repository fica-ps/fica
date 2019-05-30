use super::ffi_utils::*;

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

    let wmat = &whitened_matrix.0.into();
    
    let result = 
        fastica::fast_ica(
            wmat,
            n_components,
            max_iter,
            conv_threshold,
            alpha,
            fid_enum,
        );

    MatrixHandle::from(result)
}