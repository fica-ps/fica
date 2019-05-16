// Autogenerated header *DO NOT* modify manually

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t MatrixHandle;

typedef struct {
    MatrixHandle u;
    MatrixHandle s;
    MatrixHandle v;
} SVDHandle;

MatrixHandle create_matrix(const float *values, uint64_t rows, uint64_t cols);

SVDHandle normalized_svd(MatrixHandle hmatrix);

MatrixHandle pca_whitening(MatrixHandle hmatrix,
                           MatrixHandle h_svd_u,
                           MatrixHandle h_svd_s);

void print_matrix(MatrixHandle hmatrix);

MatrixHandle reduced_dimension_repr(MatrixHandle hmatrix,
                                    MatrixHandle h_svd_u,
                                    uint64_t ncols);

MatrixHandle rotated_data_matrix(MatrixHandle hmatrix, MatrixHandle h_svd_u);

MatrixHandle zca_whitening(MatrixHandle hmatrix,
                           MatrixHandle h_svd_u,
                           MatrixHandle h_svd_s);
