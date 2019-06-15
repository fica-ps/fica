use super::contrast::*;
use super::data::*;
use arrayfire::*;

/**
 * *** FastICA ***
 *
 *  h_white_matrix:
 *      whitened matrix reference - preprocessing technique is chosen by the user
 *      this crate provides whitening via pca or zca.
 *  
 *  max_iter:
 *      maximum number of iterations
 *
 *  n_components:
 *      number of components desired
 *
 *  conv_threshold:
 *      maximum threshold to accept convergence
 *
 * alpha:
 *      argument for g and g' functions
 *
 * ini_weights:
 *      initial weight matrix
 *   
 */
pub fn fast_ica(
    matrix: &Matrix,
    n_components: u64,
    max_iter: u64,
    conv_threshold: f64,
    alpha: f64,
    cfid: ContrastFunctionId,
) -> Matrix {

    let cfunc = get_contrast_function(cfid);

    let mut ret_weights = empty_matrix(n_components, n_components);

    for comp_i in 0..n_components {
        let mut wp = col(&matrix, comp_i);
        
        // decorrelation
        if comp_i > 1 {
            wp = decorrelate(&wp, &ret_weights, comp_i);
        }
        wp = normalize(&wp);
        
        let converged = false;

        while !converged {
            let wx = matmul(&wp, &matrix, MatProp::TRANS, MatProp::NONE);

        }      
    }

    unimplemented!()
}

    
/*
    
    t = zeros(size(w1))
    for u = 1:(i-1)
        k = sum(w1 .* retW[u,:,])
        t = t + k * retW[u,:,]
    w1 - t
*/

fn decorrelate(col_mat: &Matrix, ret_w: &Matrix, iter: u64) -> Matrix {
    let mut t = Matrix::new_empty(col_mat.dims());
    
    for col_i in 0..iter {
        let rw_col_i = col(ret_w, col_i);
        
        let k = { 
            let temp = mul(col_mat, &rw_col_i, true); 
            sum(&temp,0) 
        };
        
        t = {
            let temp = matmul(&k, col_mat, MatProp::NONE, MatProp::NONE);
            add(&t, &temp, true)
        };

    }

    sub(col_mat, &t, false)
}

fn normalize(matrix: &Matrix) -> Matrix {
    
    let matrix_dims = matrix.dims();
    
    let ini: Matrix = Matrix::new_empty(matrix_dims);
    
    let reducer = |mat, idx| { 
            let col_i     = col(matrix, idx);
            let col_norm  = norm(&col_i, NormType::VECTOR_1, 0.0, 0.0);
            let new_col   = div(&col_i, &col_norm, true);
            set_col(&mat, &new_col, idx) 
    };

    (0..matrix_dims.get()[0]).fold(ini, reducer)
}

fn distance(w: &Matrix, nw: &Matrix) -> f64 {
    /*
       dist = w1 .* wp;
       dist = sum(dist)
       dist = abs. (dist) .- 1.0
       dist = abs. (dist)
       return maximum(dist)
    */

    let dist = {
        let w_prod = mul(w, nw, true);

        let vec_size = w.dims()[0] as usize;

        let mut buffer = empty_row_vector(w.dims()[0]);

        for i in 0..vec_size {
            buffer = {
                let col_sum = sum(&w_prod, i as i32);
                set_col(&buffer, &col_sum, i as u64)
            };
        }

        let temp = abs(&buffer);
        let temp = sub(&temp, &1.0, true);
        abs(&temp)
    };

    let (max_val, _, _) = imax_all(&dist);
    max_val 
}

fn update_weights(
    weights: &Matrix,
    whitened_mat: &Matrix,
    cf: &ContrastFunc,
    alpha: f64,
) -> Matrix {
    let (g, dg) = {
        let temp = dot(weights, whitened_mat, MatProp::TRANS, MatProp::NONE);
        cf(&temp, alpha)
    };
    let new_weights = {
        let temp = mul(whitened_mat, &g, false);
        temp
        // TODO finish weight updating
    };
    new_weights
}

