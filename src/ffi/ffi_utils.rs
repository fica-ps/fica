use crate::data::Matrix;
use arrayfire::Backend;

const BACKENDS: [Backend; 3] = [Backend::OPENCL, Backend::CUDA, Backend::CPU];

#[repr(C)]
pub struct MatrixHandle(pub i64);

impl MatrixHandle {

    pub fn from(mat: Matrix) -> MatrixHandle {
        let id = mat.get();
        std::mem::forget(mat);
        MatrixHandle(id)
    }

    pub fn get_matrix(self) -> Matrix {
        self.0.into()
    }
}

#[repr(C)]
pub struct SVDHandle {
    pub u: MatrixHandle,
    pub s: MatrixHandle,
    pub v: MatrixHandle,
}

impl SVDHandle {
    pub fn get_components(self) -> (Matrix, Matrix, Matrix) {
        (
            self.u.get_matrix(),
            self.s.get_matrix(),
            self.v.get_matrix(),
        )
    }
}


#[no_mangle]
pub extern "C" fn free_handle(hmatrix: MatrixHandle) {
    hmatrix.get_matrix();
}

#[no_mangle]
pub extern "C" fn free_svd_handle(hsvd: SVDHandle) {
    hsvd.get_components();
}

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

    MatrixHandle::from(mat)
}

#[no_mangle]
pub extern "C" fn get_size(hmatrix: &MatrixHandle, cols: &mut u64, rows: &mut u64) {
    let m: &Matrix = &hmatrix.0.into();
    let (c, r) = {
        let dimobj = m.dims();
        let dimarr = dimobj.get();
        (dimarr[0], dimarr[1])
    };

    *cols = c;
    *rows = r;
}

#[no_mangle]
pub extern "C" fn copy_matrix(hmatrix: &MatrixHandle, to: *mut f32) {
    use std::slice;

    let m: &Matrix = &hmatrix.0.into();

    m.host(unsafe { slice::from_raw_parts_mut(to, m.elements()) });
}

#[no_mangle]
pub extern "C" fn get_matrix(hmatrix: MatrixHandle, to: *mut f32) {
    use std::slice;

    let m = hmatrix.get_matrix();


    m.host(unsafe { slice::from_raw_parts_mut(to, m.elements()) });
}
#[no_mangle]
pub extern "C" fn print_matrix(hmatrix: &MatrixHandle) {
    use arrayfire::print;

    let m: &Matrix = &hmatrix.0.into();;
    print(m);
}