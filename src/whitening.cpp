#include "../include/whitening.h"
#include <iostream>
#include <Eigen/Eigenvalues>

namespace whitening {

    using namespace Eigen;

    static WhiteningMatrixGen WHITE_MAT_GEN[2] = {
            &whitening::pca,
            &whitening::zca
    };

    Eigen::MatrixXd  get_white_matrix(Eigen::MatrixXd matrix, WhiteningTypeId wtid)
    {
        return WHITE_MAT_GEN[wtid](matrix);
    }

    Eigen::MatrixXd pca(const Eigen::MatrixXd& matrix) {
        if(matrix.rows() == 1) {
            std::cerr << "Matrix must have more than 1 column";
            exit(-1);
        }
        /*
        MatrixXd centered = matrix.rowwise() - matrix.colwise().mean();
        MatrixXd cov = (centered.adjoint() * centered) / double(matrix.rows() - 1);
        EigenSolver<MatrixXd> eig(cov, true);


        MatrixXd eigvals = eig.eigenvalues();
        MatrixXd eigvecs = eig.eigenvectors();
        */
        return matrix;
    }

    Eigen::MatrixXd zca(const Eigen::MatrixXd& matrix) {
        return matrix;
    }

}