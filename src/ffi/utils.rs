use super::MatrixHandle;

#[no_mangle]
pub extern "C" fn create_matrix(values: *mut f32, rows: u64, cols: u64) -> MatrixHandle {
    use crate::data::new_matrix;
    use std::slice;

    new_matrix(
        unsafe { slice::from_raw_parts(values, (rows * cols) as usize) },
        rows,
        cols,
    )
    .get()
}

#[no_mangle]
pub extern "C" fn get_matrix(hmatrix: MatrixHandle, to: *mut f32, size: usize) {
    use crate::data::Matrix;
    use std::slice;

    let m: Matrix = hmatrix.into();
    let buffer: &mut [f32] = unsafe { slice::from_raw_parts_mut(to, size) };
    m.host(buffer);
}

#[no_mangle]
pub extern "C" fn print_matrix(hmatrix: MatrixHandle) {

    use crate::data::Matrix;
    use arrayfire::print;
    let m: Matrix = hmatrix.into();
    print(&m);
}