#include "../include/whitening.h"
#include <iostream>
#include <Eigen/Eigenvalues>

namespace whitening {

    using namespace Eigen;

    Eigen::MatrixXd  get_white_matrix(Eigen::MatrixXd matrix, WhiteningTypeId wtid)
    {
        return pca(matrix, wtid);
    }

    Eigen::MatrixXd pca(const Eigen::MatrixXd& matrix, bool zca) {
        if(matrix.rows() == 1) {
            std::cerr << "Matrix must have more than 1 column";
            exit(-1);
        }

        Eigen::MatrixXd centered = matrix.rowwise() - matrix.colwise().mean();
        Eigen::MatrixXd cov = (centered.transpose() * centered) / double( matrix.rows() - 1 );

        Eigen::EigenSolver<Eigen::MatrixXd> es(cov);

        Eigen::VectorXd vals = es.eigenvalues().real().reverse();
        Eigen::MatrixXd vecs = es.eigenvectors().real().rowwise().reverse();
        Eigen::MatrixXd ret = (1.0 / vals.array().sqrt()).matrix().asDiagonal() * vecs.transpose();

        if(zca)
            return vecs * ret;
        return ret;
    }

}