use crate::data;
use arrayfire::Backend;
use std::ffi::c_void;

const BACKENDS: [Backend; 3] = [Backend::OPENCL, Backend::CUDA, Backend::CPU];

pub type Matrix = *mut c_void;

pub fn handle(mat: data::Matrix) -> Matrix {
    handle_boxed(Box::new(mat))
}

pub fn handle_boxed(bmat: Box<data::Matrix>) -> Matrix {
    Box::into_raw(bmat) as Matrix
}

pub fn handle2mat(hmatrix: Matrix) -> Box<data::Matrix> {
    unsafe { Box::from_raw(hmatrix as *mut data::Matrix) }
}

pub fn forget_many(mhandles: &[Box<data::Matrix>]) {
    for h in mhandles {
        std::mem::forget(h);
    }
}


#[no_mangle]
pub extern "C" fn free_handle(hmatrix: Matrix) {
    let x: Box<data::Matrix> = handle2mat(hmatrix);
    std::mem::drop(x);
}

#[no_mangle]
pub extern "C" fn set_backend(backend_id: usize) {
    arrayfire::set_backend(BACKENDS[backend_id]);
}


#[no_mangle]
pub extern "C" fn create_matrix(values: *mut f64, cols: u64, rows: u64) -> Matrix {
    use crate::data::new_matrix;
    use std::slice;

    let mat = new_matrix(
        unsafe { slice::from_raw_parts(values, (rows * cols) as usize) },
        rows,
        cols,
    );


    handle(mat)
}

#[no_mangle]
pub extern "C" fn get_size(hmatrix: Matrix, cols: &mut u64, rows: &mut u64) {
    let m: Box<data::Matrix> = handle2mat(hmatrix);
    let (c, r) = {
        let dimobj = m.dims();
        let dimarr = dimobj.get();
        (dimarr[0], dimarr[1])
    };

    *cols = c;
    *rows = r;
    std::mem::forget(m);
}

#[no_mangle]
pub extern "C" fn copy_matrix(hmatrix: Matrix, to: *mut f64, size: usize) {
    use std::mem;
    use std::slice;

    let m: Box<data::Matrix> = handle2mat(hmatrix);
    let slc = unsafe { slice::from_raw_parts_mut(to, size) };

    m.host(slc);
    mem::forget(m);
}

#[no_mangle]
pub extern "C" fn move_matrix(hmatrix: Matrix, to: *mut f64, size: usize) {
    use std::slice;

    let m: Box<data::Matrix> = handle2mat(hmatrix);

    m.host(unsafe { slice::from_raw_parts_mut(to, size) });
}

#[no_mangle]
pub extern "C" fn print_matrix(hmatrix: Matrix) {
    use arrayfire::print;

    let m: Box<data::Matrix> = handle2mat(hmatrix);

    print(&m);

    std::mem::forget(m);
}

#[repr(C)]
pub struct SVDHandle {
    pub u: Matrix,
    pub s: Matrix,
    pub v: Matrix,
}

impl SVDHandle {
    pub fn get_components(self) -> (Box<data::Matrix>, Box<data::Matrix>, Box<data::Matrix>) {
        (handle2mat(self.u), handle2mat(self.s), handle2mat(self.v))
    }
}

#[no_mangle]
pub extern "C" fn free_svd_handle(hsvd: SVDHandle) {
    for h in &[hsvd.s, hsvd.u, hsvd.v] {
        free_handle(*h)
    }
}