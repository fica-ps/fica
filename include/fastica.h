//
// Created by adriano on 7/2/19.
//

#ifndef FICA_FASTICA_H
#define FICA_FASTICA_H

#include "contrast.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct {
        const Eigen::MatrixXd& matrix;
        const Eigen::MatrixXd& ini_weights;
        unsigned int n_components;
        double conv_threshold;
        double alpha;
        ContrastFunctionId cont_func_id;
        bool verbose;
    } ICA_Params;

#ifdef __cplusplus
}

namespace fastica {

    Eigen::MatrixXd *fast_ica(
            const Eigen::MatrixXd& dataset,
            const Eigen::MatrixXd& white_matrix,
            const Eigen::MatrixXd& weights,
            ICA_Params parameters
    );

}
#endif // C++ header files and definitions

#endif //FICA_FASTICA_H
