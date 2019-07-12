//
// Created by Adriano on 12/07/2019.
//

#include <cmath>
#include <fica.h>
#include <iostream>

int main() {

    double f[2][1000] = {0};

    for(int i = 1; i < 1001; ++i) {
        f[0][i - 1] = sin((double) i);
    }

    for(int i = 0; i < 1000; ++i) {
        f[1][i] = 1.0;
    }

    Matrix matrix_fs = new_Matrix_c((double *)f,1000,2);

    print_Matrix(matrix_fs, "\ncol-1: f1  col-2: f2 ");

    double *x = copy_Matrix(matrix_fs, nullptr, true);

    for (size_t row = 0; row < 1000; row++){
        for (size_t col = 0; col < 2; col += 2) {
            std::cout << "(" << x[col * 1000 + row] << ", " <<  x[(col + 1) * 1000 + row] << ")" << std::endl;
        }
    }

    free(x);

}