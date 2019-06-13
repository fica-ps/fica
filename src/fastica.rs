use super::contrast::*;
use super::data::*;
use super::utils::normalize;
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
            
            let mut temp = Matrix::new_empty(wp.dims());
            
            for i in 0..comp_i {
                let rwi = col(&ret_weights, i);
                let k = {
                    let temp = mul(&wp, &rwi , true);
                    sum(&temp, 0)
                };

                temp = add(&temp, &k, true);
                temp = matmul(&temp, &rwi, MatProp::NONE, MatProp::NONE);
            }

            wp = sub(&wp, &temp, false);
        }

        wp = normalize(&wp);
        
        let converged = false;
        while !converged {
            let wx = matmul(&wp, &matrix, MatProp::TRANS, MatProp::NONE);
            
        }      
    }

    unimplemented!()
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

fn gram_schmit_decorrelation(nw: &Matrix, wcol: &Matrix) -> Matrix {
    /*
        x = dotProduct(w_new, W[col_i]')
        x = dotProduct(x, W[col_i])
        w_new - x
    */

    let mut rw = dot(nw, wcol, MatProp::NONE, MatProp::TRANS);
    rw = dot(&rw, wcol, MatProp::NONE, MatProp::NONE);
    sub(nw, &rw, false)
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

