
pub mod utils;
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

pub extern "C" fn fast_ica(
    whitened_matrix: MatrixHandle,
    n_components: u64,
    max_iter: u64,
    conv_threshold: f32,
    alpha: f32,
    cfid: u32,
) -> MatrixHandle {

    use crate::fastica;
    use crate::contrast::ContrastFunctionId;
    
    let fid_enum: ContrastFunctionId = unsafe {std::mem::transmute(cfid as u8)};

    fastica::fast_ica(
        &whitened_matrix.into(), 
        n_components, max_iter, 
        conv_threshold, alpha, 
        fid_enum 
    ).get()

}
