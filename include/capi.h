#ifndef FICA_CAPI_H
#define FICA_CAPI_H

#include "fastica.h"

#ifdef __cplusplus
#include <cstdio>
#else
#include <stdlib.h>
#endif

#ifdef COMPILING_SHARED_LIB
#define FICA_EXPORT __declspec(dllexport)
#else
#define FICA_EXPORT __declspec(dllimport)
#endif

#ifdef __cplusplus
extern "C" {
#endif

    typedef void* Matrix;

    FICA_EXPORT Matrix new_Matrix_c(const double* values, size_t rows, size_t cols);

    FICA_EXPORT Matrix new_Matrix_r(const double *values, size_t rows, size_t cols);

    FICA_EXPORT void free_Matrix(Matrix m);

    FICA_EXPORT void print_Matrix(Matrix matrix, const char *message);

    FICA_EXPORT Matrix fast_ica(Matrix dataset, Matrix ini_weights, Matrix white_mat, ICA_Params parameters);

    FICA_EXPORT double *copy_Matrix(Matrix matrix, double* buffer, bool dealloc);

    //FICA_EXPORT Matrix mul(Matrix m1, Matrix m2, bool in_place);

    //FICA_EXPORT Matrix add(Matrix m1, Matrix m2, bool in_place);



#ifdef __cplusplus
}
#endif

#endif //FICA_CAPI_H