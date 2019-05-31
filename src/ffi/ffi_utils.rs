use crate::data::Matrix;
use arrayfire::Backend;
use std::ffi::c_void;

const BACKENDS: [Backend; 3] = [Backend::OPENCL, Backend::CUDA, Backend::CPU];

pub type MatrixHandle = *mut c_void;

pub fn handle(mat: Matrix) -> MatrixHandle {
    handle_boxed(Box::new(mat))
}

pub fn handle_boxed(bmat: Box<Matrix>) -> MatrixHandle {
    Box::into_raw(bmat) as MatrixHandle
}

pub fn handle2mat(hmatrix: MatrixHandle) -> Box<Matrix> {
    unsafe { Box::from_raw(hmatrix as *mut Matrix) }
}

pub fn forget_many(mhandles: &[Box<Matrix>]) {
    for h in mhandles {
        std::mem::forget(h);
    }
}


#[no_mangle]
pub extern "C" fn free_handle(hmatrix: MatrixHandle) {
    let x: Box<Matrix> = handle2mat(hmatrix);
    std::mem::drop(x);
}

#[no_mangle]
pub extern "C" fn set_backend(backend_id: i32) {
    arrayfire::set_backend(BACKENDS[backend_id as usize]);
}


#[no_mangle]
pub extern "C" fn create_matrix(values: *mut f32, cols: u64, rows: u64) -> MatrixHandle {
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
pub extern "C" fn get_size(hmatrix: MatrixHandle, cols: &mut u64, rows: &mut u64) {
    let m: Box<Matrix> = handle2mat(hmatrix);
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
pub extern "C" fn copy_matrix(hmatrix: MatrixHandle, to: *mut f32) {
    use std::slice;

    let m: Box<Matrix> = handle2mat(hmatrix);;
    m.host(unsafe { slice::from_raw_parts_mut(to, m.elements()) });
    std::mem::forget(m);
}

#[no_mangle]
pub extern "C" fn move_matrix(hmatrix: MatrixHandle, to: *mut f32) {
    use std::slice;

    let m: Box<Matrix> = handle2mat(hmatrix);


    m.host(unsafe { slice::from_raw_parts_mut(to, m.elements()) });

    free_handle(hmatrix);
}

#[no_mangle]
pub extern "C" fn print_matrix(hmatrix: MatrixHandle) {
    use arrayfire::print;

    let m: Box<Matrix> = handle2mat(hmatrix);

    print(&m);

    std::mem::forget(m);
}

#[repr(C)]
pub struct SVDHandle {
    pub u: MatrixHandle,
    pub s: MatrixHandle,
    pub v: MatrixHandle,
}

impl SVDHandle {

    pub fn get_components(self) -> (Box<Matrix>, Box<Matrix>, Box<Matrix>) {
        (handle2mat(self.u), handle2mat(self.s), handle2mat(self.v))
    }
}

#[no_mangle]
pub extern "C" fn free_svd_handle(hsvd: SVDHandle) {
    for h in &[hsvd.s, hsvd.u, hsvd.v] {
        free_handle(*h)
    }
}