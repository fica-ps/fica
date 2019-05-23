use super::data::*;
use crate::contrast::*;

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
    whitened_matrix: &Matrix,
    n_components: u64,
    max_iter: u64,
    conv_threshold: f32,
    alpha: f32,
    cfid: ContrastFunctionId,
) -> Matrix {
    let cfunc = get_contrast_function(cfid);

    let mut ret_weights = empty_matrix(n_components, n_components);

    for comp_i in 0..n_components {
        // weight column vector
        let mut weights_col: Matrix = randu::<f32>(dim(n_components, 1));

        for _ in 0..max_iter {
            // calculate a new weight column
            let n_weights_col = {
                let temp = update_weights(&weights_col, whitened_matrix, &cfunc, alpha);

                if comp_i >= 1 {
                    // Avoid convergeing in a local minima after the first iteration
                    let slice_col = col(&weights_col, comp_i);

                    gram_schmit_decorrelation(&temp, &slice_col)
                } else {
                    temp
                }
            };

            let conv_distance = distance(&weights_col, &n_weights_col);

            weights_col = n_weights_col;

            if conv_distance < conv_threshold {
                // set ret_weights component column to new weights
                ret_weights = set_col(&ret_weights, &weights_col, comp_i);
                break;
            }
        }
    }

    // return estimated components
    dot(&ret_weights, whitened_matrix, MatProp::NONE, MatProp::NONE)
}

fn distance(w: &Matrix, nw: &Matrix) -> f32 {
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
    max_val as f32
}

fn gram_schmit_decorrelation(nw: &Matrix, wcol: &Matrix) -> Matrix {
    /*
        x = dotProduct(w_new, W[col_i]')
        x = dotProduct(x, W[col_i])
        w_new - x
    */

    let rw = dot(nw, wcol, MatProp::NONE, MatProp::TRANS);
    let rw = dot(&rw, wcol, MatProp::NONE, MatProp::NONE);
    sub(nw, &rw, false)
}

fn update_weights(
    weights: &Matrix,
    whitened_mat: &Matrix,
    cf: &ContrastFunc,
    alpha: f32,
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
