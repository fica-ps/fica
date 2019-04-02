use arrayfire::*;

pub type Matrix = Array<f32>;
pub type SVD = (Matrix, Matrix, Matrix);

// normalization constant = exp(1.0, -5).
static EPSILON: f32 = 0.00001_f32;

// *** data centering and SVD ***
pub fn normalized_svd(mat: &Matrix) -> SVD {
    svd({
        let centered_mat     = sub(mat, &mean(mat, 0 as i64), true);
        let cm_transpose_mul = matmul(&centered_mat, &centered_mat, MatProp::NONE, MatProp::TRANS);
        let inv_col_size     = 1.0 / centered_mat.dims().get()[1] as f32;
        
        &mul(&cm_transpose_mul, &inv_col_size, true)
    })
}

// *** data rotation ***
pub fn rotated_data_matrix(mat: &Matrix, sigma_svd_u: &Matrix) -> Matrix {
    matmul(mat, sigma_svd_u, MatProp::TRANS, MatProp::NONE)  
}

// *** dimensional reduction ***
pub fn reduced_dimension_repr(mat: &Matrix, sigma_svd_u: &Matrix, ncols: u64) -> Matrix {
    let new_dim = {
        let dims = mat.dims();
        let dims = dims.get();
        Dim4::new(&[dims[0], ncols, 1, 1])
    };
    matmul( &tile( &sigma_svd_u, new_dim ), mat, MatProp::TRANS, MatProp::NONE )  
}

/* *** PCA whitening ***
 * xPCAwhite(x) = diag(1./sqrt(diag(S) + epsilon)) * U' * x 
 * where x = mat, S = sigma_svd_s, U = sigma_svd_u
 */
pub fn pca_whitening(mat: &Matrix, sigma_svd_u: &Matrix, sigma_svd_s: &Matrix) -> Matrix {        
    let mut result:Matrix = diag_create(sigma_svd_s, 0);                    // r = diag(S)
    result = add(&result, &EPSILON,true);                                   // r = r + epsilon
    result = sqrt(&result);                                                 // r = sqrt(r)
    result = div(&1.0_f32, &result, true);                                  // r = 1.0 / r
    result = diag_create(&result, 0);                                       // r = diag(r)
    result = matmul(&result, sigma_svd_u, MatProp::NONE, MatProp::TRANS);   // r = r * U'
    
    matmul(&result, mat, MatProp::NONE, MatProp::NONE)                      // r * x
}

/* *** ZCA whitening ***
 * xZCAwhite(x) = U  * xPCAwhite(x) 
 * where x = mat, U = sigma_svd_u
 */
pub fn zca_whitening(mat: &Matrix, sigma_svd_u: &Matrix, sigma_svd_s: &Matrix) -> Matrix {
    matmul(sigma_svd_u, 
           &pca_whitening(mat, sigma_svd_u, sigma_svd_s), 
           MatProp::NONE, 
           MatProp::NONE)
}