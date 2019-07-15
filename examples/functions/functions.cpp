//
// Created by Adriano on 12/07/2019.
//

#include <cmath>
#include <fica.h>
#include <iostream>
#include <Eigen/Dense>
#define dbl(a) ((double)(a))

using namespace std;

Eigen::MatrixXd pca(const Eigen::MatrixXd& matrix) {
    if(matrix.rows() == 1) {
        std::cerr << "Matrix must have more than 1 column";
        exit(-1);
    }

    Eigen::MatrixXd centered = matrix.rowwise() - matrix.colwise().mean();
    Eigen::MatrixXd cov = (centered.adjoint() * centered) / double(matrix.rows() - 1);
    //Eigen::EigenSolver<Eigen::MatrixXd> eig(cov, true);


    //Eigen::MatrixXd eigvals = eig.eigenvalues();
    //Eigen::MatrixXd eigvecs = eig.eigenvectors();

    //cout << eigvals;

    //cout << eigvecs;

    return matrix;
}

int main() {



    double f[2][1000] = {0};

    for(int i = 1; i < 1001; ++i) {
        f[0][i - 1] = sin(dbl(i) / 20.0);
    }

    for(int i = 0; i < 1000; ++i) {
            f[1][i] = (dbl(i % 200) + 1.0 - 100.0) / 100.0;
    }

    auto matrix_fs = (Eigen::MatrixXd*)new_Matrix_c((double *)f,1000,2);

    Eigen::MatrixXd mat(2,2);
    mat << 0.291, -0.5439 , 0.6557, 0.5572;

    *matrix_fs *= mat;

    //print_Matrix(matrix_fs, "\ncol-1: f1  col-2: f2 ");

    double w_vals[4] = {1.02081, 0.408655, -1.92523, -0.756068};
    Matrix ini_w = new_Matrix_r(w_vals,2,2);

    ICA_Params params = {
        2,
        0.0001,
        1.0,
        EIGEN,
        LOGCOSH,
        true,
        nullptr,
    };

    double white_vals[4] = {0.823944 , 1.72176, -2.22348, 1.06404};
    Matrix white_mat = new_Matrix_r(white_vals,2,2);

    Matrix retW = fast_ica(matrix_fs, white_mat ,ini_w, params);
    print_Matrix(retW,"retW");

    free_Matrix(retW);
    free_Matrix(matrix_fs);

}