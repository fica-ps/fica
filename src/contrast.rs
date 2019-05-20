use crate::data::Matrix;
use crate::ffi::ContrastFunctionId;

use arrayfire::*;

pub type ContrastFunc =  Box<Fn(&Matrix) -> (Matrix, Matrix)>;

pub fn gen_contrast_function(fid: ContrastFunctionId, alpha: f32) -> ContrastFunc {
    
    // TODO implement kurtosis and exp
    match fid {
        ContrastFunctionId::LOGCOSH => Box::new(move |m| logcosh(m, alpha)),
        _ => unimplemented!()
    }
} 

fn logcosh(mat: &Matrix, alpha:f32) -> (Matrix, Matrix) {
    
    let factored_mat = mul(mat, &alpha, true);
    
    // 1/alpha * log(cosh(alph * u))
    let g = log(&cosh(&factored_mat));
    let g = mul(&(1.0 / alpha), &g, true);
    
    // np.tanh(alph * u)
    let dg = tanh(&factored_mat);
    
    (g, dg)  
}