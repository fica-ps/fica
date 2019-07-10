#include "../headers/capi.h"
#include <Eigen/Dense>
#include <iostream>

#define eigmat(matrix) ((Eigen::MatrixXd*)matrix)
#define ficamat(emat) ((Matrix)emat)

using namespace std;

extern "C" {

    Matrix new_matrix(const double* values, size_t rows, size_t cols) {
        auto mat = new Eigen::MatrixXd(rows, cols);

        for(size_t r = 0; r < rows; r++)
            for(size_t c = 0; c < cols; c++)
                (*mat)(r,c) = values[r * cols + c];

        return ficamat(mat);
    }

    void free_matrix(Matrix m) {
        delete eigmat(m);
    }

    void print_matrix(Matrix matrix, const char *message) {
        auto m = *eigmat(matrix);
        cout << message << "\n\n" << m << "\n" << std::endl;
    }

    Matrix fast_ica(Matrix dataset, Matrix ini_weights, Matrix white_mat, ICA_Params parameters) {

        Eigen::MatrixXd *mat = eigmat(dataset);
        Eigen::MatrixXd *iw  = eigmat(ini_weights);
        Eigen::MatrixXd *wm  = eigmat(white_mat);

        Eigen::MatrixXd *m = fastica::fast_ica(*mat,*iw, *wm, parameters);

        return ficamat(m);

    }

}