#ifndef FICA_CAPI_H
#define FICA_CAPI_H

#ifdef __cplusplus
#include <cstdio>
#else
#include <stdlib.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

    typedef void* Matrix;

    Matrix new_matrix(const double* values, size_t rows, size_t cols);

    void free_matrix(Matrix m);

    void print_matrix(Matrix matrix, const char *message);

#ifdef __cplusplus
}
#endif

#endif //FICA_CAPI_H