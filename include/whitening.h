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

    typedef Eigen::MatrixXd (*WhiteningMatrixGen)(const Eigen::MatrixXd&);

    namespace whitening {
        Eigen::MatrixXd pca(const Eigen::MatrixXd& matrix, bool zca);
    }

#ifdef __cplusplus
}
#endif

#endif //FICA_WHITENING_H
