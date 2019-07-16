#include <cmath>
#include <fica.h>
#include <iostream>
#include <Eigen/Dense>
#define dbl(a) ((double)(a))

using namespace std;

int main() {

    double f[2][100] = {0};

    for(int i = 1; i < 101; ++i) {
        f[0][i - 1] = sin(dbl(i) / 20.0);
    }

    for(int i = 0; i < 100; ++i) {
            f[1][i] = (dbl(i % 200) + 1.0 - 100.0) / 100.0;
    }

    auto matrix_fs = (Eigen::MatrixXd*)new_Matrix_c((double *)f,100,2);

    double w_vals[4] = {1.02081, 0.408655, -1.92523, -0.756068};
    Matrix ini_w = new_Matrix_r(w_vals,2,2);

    double avals[4] = {0.291, -0.5439, 0.6557, 0.5572};
    auto a = (Eigen::MatrixXd*)new_Matrix_r(avals,2,2);


    ICA_Params params = {
        2,
        0.0001,
        1.0,
        100,
        PCA,
        LOGCOSH,
        true,
        nullptr,

    };

    double white_vals[4] = { -1.0376, 1.99405, -18.3002, -9.5225 };
    Matrix white_mat = new_Matrix_r(white_vals,2,2);

    Eigen::MatrixXd mat(2,2);
    mat << 0.291, -0.5439 , 0.6557, 0.5572;
    *matrix_fs *= mat;

    print_Matrix(matrix_fs,"matrix fs");

    Matrix retW = fast_ica(matrix_fs, white_mat ,ini_w, params);
    print_Matrix(retW,"retW");
    free_Matrix(retW);

    free_Matrix(matrix_fs);

}