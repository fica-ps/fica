use crate::data::Matrix;
use crate::ffi::ContrastFunctionId;

use arrayfire::*;

pub type ContrastFunc = fn(&Matrix, f32) -> (Matrix, Matrix);

const functions: [ContrastFunc; 3] = [logcosh, logcosh, logcosh];

pub fn get_contrast_function(fid: ContrastFunctionId) -> ContrastFunc {
    functions[fid as usize]
}

fn logcosh(mat: &Matrix, alpha: f32) -> (Matrix, Matrix) {
    let factored_mat = mul(mat, &alpha, true);

    // 1/alpha * log(cosh(alph * u))
    let g = log(&cosh(&factored_mat));
    let g = mul(&(1.0 / alpha), &g, true);

    // np.tanh(alph * u)
    let dg = tanh(&factored_mat);

    (g, dg)
}
