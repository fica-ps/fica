#ifndef FICA_WHITENING_H
#define FICA_WHITENING_H

#include "Eigen/Dense"

#ifdef __cplusplus
extern "C" {
#endif

    typedef enum {
        PCA   = false,
        ZCA   = true
    } WhiteningTypeId;

#ifdef __cplusplus
}

namespace whitening {

    typedef Eigen::MatrixXd (*WhiteningMatrixGen)(const Eigen::MatrixXd&);

    Eigen::MatrixXd pca(const Eigen::MatrixXd& matrix, bool zca);
}
#endif

#endif //FICA_WHITENING_H
