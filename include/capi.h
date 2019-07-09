#ifndef FICA_CAPI_H
#define FICA_CAPI_H

#include <cstdio>

extern "C" {

    typedef void* Matrix;

    Matrix new_matrix(const double* values, size_t rows, size_t cols);

    void free_matrix(Matrix m);

    void print_matrix(Matrix matrix, const char *message);

}

#endif //FICA_CAPI_H
