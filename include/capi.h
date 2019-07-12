#ifndef FICA_CAPI_H
#define FICA_CAPI_H

#include "fastica.h"

#ifdef __cplusplus
#include <cstdio>
#else
#include <stdlib.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif


    typedef void* Matrix;

    Matrix new_Matrix_c(const double* values, size_t rows, size_t cols);

    Matrix new_Matrix_r(const double *values, size_t rows, size_t cols);

    void free_Matrix(Matrix m);

    void print_Matrix(Matrix matrix, const char *message);

    Matrix fast_ica(Matrix dataset, Matrix ini_weights, Matrix white_mat, ICA_Params parameters);

#ifdef __cplusplus
}
#endif

#endif //FICA_CAPI_H