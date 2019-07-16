#include "../include/capi.h"
#include "../include/whitening.h"
#include "../include/contrast.h"
#include <Eigen/Dense>
#include <iostream>

#define eigmat(matrix) ((Eigen::MatrixXd *)matrix)
#define ficamat(emat) ((Matrix)emat)

#define index_copy(mat,values,idx_expr) for (size_t row = 0; row < rows; row++){ for (size_t col = 0; col < cols; col++) { (*mat)(row, col) = values[idx_expr]; } }

using namespace std;

extern "C"
{

    Matrix new_Matrix_c(const double *values, size_t rows, size_t cols)
    {
        auto mat = new Eigen::MatrixXd(rows, cols);

        index_copy(mat, values, col * rows + row);

        return ficamat(mat);
    }

    Matrix new_Matrix_r(const double *values, size_t rows, size_t cols)
    {
        auto mat = new Eigen::MatrixXd(rows, cols);

        index_copy(mat, values, row * cols + col);

        return ficamat(mat);
    }

    void free_Matrix(Matrix m)
    {
        delete eigmat(m);
    }

    void print_Matrix(Matrix matrix, const char *message)
    {
        cout << message << "\n\n"
             << *eigmat(matrix) << "\n"
             << std::endl;
    }

    double *copy_Matrix(Matrix matrix, double* buffer, bool dealloc) {
        auto m = eigmat(matrix);

        auto msize = m->size();

        double *to = buffer;

        if(to == nullptr)
            to = (double*)malloc(sizeof(double) * msize);

        for(size_t i = 0; i < m->size(); ++i)
            to[i] = m->data()[i];

        if(dealloc)
            delete m;

        return to;
    }

    unsigned int rows(Matrix matrix) {
        return eigmat(matrix)->rows();
    }

    unsigned int cols(Matrix matrix) {
        return eigmat(matrix)->cols();
    }

    unsigned int size(Matrix matrix) {
        return eigmat(matrix)->size();
    }

    Matrix fast_ica(Matrix dataset, Matrix ini_weights, Matrix white_mat, ICA_Params parameters)
    {

        Eigen::MatrixXd mat  = *eigmat(dataset);
        Eigen::MatrixXd *iw  = eigmat(ini_weights);
        Eigen::MatrixXd *wm  = eigmat(white_mat);

        Eigen::MatrixXd *m = fastica::fast_ica(mat, iw, wm, parameters);

        return ficamat(m);
    }

    FICA_EXPORT ICA_Params default_ica_params(Matrix m) {
        return ICA_Params {
            cols(m),
            0.0001,
            1.0,
            100,
            PCA,
            LOGCOSH,
            false,
            nullptr
        };
    }
}