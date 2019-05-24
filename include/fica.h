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

MatrixHandle create_matrix(float *values, uint64_t rows, uint64_t cols);

MatrixHandle fast_ica(MatrixHandle whitened_matrix,
                      uint64_t n_components,
                      uint64_t max_iter,
                      float conv_threshold,
                      float alpha,
                      uint32_t cfid);

void get_matrix(MatrixHandle hmatrix, float *to, uintptr_t size);

SVDHandle normalized_svd(MatrixHandle hmatrix);

MatrixHandle pca_whitening(MatrixHandle hmatrix, SVDHandle svd_h);

void print_matrix(MatrixHandle hmatrix);

MatrixHandle reduced_dimension_repr(MatrixHandle hmatrix,
                                    MatrixHandle h_svd_u,
                                    uint64_t ncols);

MatrixHandle rotated_data_matrix(MatrixHandle hmatrix, MatrixHandle h_svd_u);

MatrixHandle zca_whitening(MatrixHandle hmatrix, SVDHandle svd_h);
