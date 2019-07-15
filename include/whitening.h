#ifndef FICA_WHITENING_H
#define FICA_WHITENING_H

#include "Eigen/Dense"

#ifdef __cplusplus
extern "C" {
#endif

    typedef enum {
        EIGEN = 0,
        PCA   = 1,
        ZCA   = 2
    } WhiteningTypeId;

    typedef Eigen::MatrixXd (*WhiteningMatrixGen)(const Eigen::MatrixXd&);

    namespace whitening {
        Eigen::MatrixXd pca(const Eigen::MatrixXd& matrix);
        Eigen::MatrixXd zca(const Eigen::MatrixXd& matrix);
    }

#ifdef __cplusplus
}
#endif

#endif //FICA_WHITENING_H
