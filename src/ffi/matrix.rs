use std::os::raw::c_void;
use super::utils::*;


pub type MatrixHandle = *mut c_void;

#[repr(C)]
pub struct SVDHandle {
    pub u: MatrixHandle,
    pub s: MatrixHandle,
    pub v: MatrixHandle,
}

#[no_mangle]
pub extern "C" fn create_matrix(values: *mut f32, cols: u64, rows: u64) -> MatrixHandle {
    use crate::data::new_matrix;
    use std::slice;

    let mat = 
        new_matrix(
            unsafe { slice::from_raw_parts(values, (rows * cols) as usize) },
            rows,
            cols,
        );

    to_handle(mat)
}

#[no_mangle]
pub extern "C" fn get_size(hmatrix: &MatrixHandle, cols: &mut u64, rows: &mut u64) {
    let m = from_handle(*hmatrix);
    let (c, r) = {
        let dimobj = m.dims();
        let dimarr = dimobj.get();
        (dimarr[0], dimarr[1])
    };   

    *cols = c;
    *rows = r;
} 

#[no_mangle]
pub extern "C" fn get_matrix(hmatrix: &MatrixHandle, to: *mut f32) {
    use std::slice;

    let m = from_handle(*hmatrix);
    
    m.host(
        unsafe { slice::from_raw_parts_mut(to, m.elements()) }
    );
}


#[no_mangle]
pub extern "C" fn print_matrix(hmatrix: &MatrixHandle) {
    use arrayfire::print;

    let m = from_handle(*hmatrix);
    print(&m);
}