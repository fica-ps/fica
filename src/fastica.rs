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
pub fn fast_ica(whitened_matrix: &Matrix, weights: &Matrix, 
                max_iter: u64, n_components: u64, 
                conv_threshold: f32, alpha: f32) -> Matrix {
    
    let ret_weights = empty_matrix(n_components, n_components);

    for comp_i in 0..n_components {
        
        // weight column vector
        let mut weights_col: Matrix = randu::<f32>(dim(n_components, 1));
        
        for _iter_i in 0..max_iter {
            
            let n_weights_col = { 
                let nw_temp = update_weights(&weights_col, &whitened_matrix, alpha);
                if comp_i >= 1 { 
                    gram_schmit_decorrelation(&nw_temp, &whitened_matrix)
                } else {
                    nw_temp
                }
            };
             
            let conv_distance = distance(&weights_col, &n_weights_col);
            
            weights_col = n_weights_col;

            if conv_distance < conv_threshold { 
                break; 
            }
        }

        // set (ret_weights) column to weights_col
    }

    unimplemented!()
}

fn distance(w: &Matrix, nw: &Matrix) -> f32 {
    /*
        chg = maximum(abs.(abs.(sum(w1 .* wp)) .- 1.0))
     */
    unimplemented!()
}

fn gram_schmit_decorrelation(nw: &Matrix, whitened_mat:&Matrix) -> Matrix  {
    /*
        w_new -= np.dot(np.dot(w_new, W[:i].T), W[:i])
    */
    unimplemented!()
}

fn update_weights(nw: &Matrix, whitened_mat:&Matrix, alpha: f32) -> Matrix {
    unimplemented!()
} 
