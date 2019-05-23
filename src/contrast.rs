use crate::data::Matrix;

use arrayfire::*;

pub enum ContrastFunctionId {
    LOGCOSH = 0,
    KURTOSIS = 1,
    EXP = 2,
}

pub type ContrastFunc = fn(&Matrix, f32) -> (Matrix, Matrix);

const CONTRAST_FUNCTIONS: [ContrastFunc; 3] = [logcosh, kurtosis, exponential];

pub fn get_contrast_function(fid: ContrastFunctionId) -> ContrastFunc {
    CONTRAST_FUNCTIONS[fid as usize]
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

fn kurtosis(mat: &Matrix, alpha: f32) -> (Matrix, Matrix) {
    (
        mul(&(1.0 / alpha), &pow(mat, &alpha, true), true),
        pow(mat, &(alpha - 1.0), true),
    )
}

fn exponential(mat: &Matrix, alpha: f32) -> (Matrix, Matrix) {
    let e = {
        let mut temp = pow(mat, &alpha, true);
        temp = sub(&0, &temp, true);

        let coef = -(1.0 / alpha);

        mul(&coef, &temp, true)
    };

    let g = matmul(mat, &e, MatProp::NONE, MatProp::NONE);

    // (1 - np.power(u, 2)) * exp
    let dg = {
        let mut temp = pow(mat, &alpha, true);
        temp = matmul(&temp, &e, MatProp::NONE, MatProp::NONE);
        sub(&1, &temp, true)
    };

    (g, dg)
}
