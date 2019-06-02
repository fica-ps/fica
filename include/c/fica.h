// Autogenerated header *DO NOT* modify manually

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void *MatrixHandle;

typedef struct {
    MatrixHandle u;
    MatrixHandle s;
    MatrixHandle v;
} SVDHandle;

void copy_matrix(MatrixHandle hmatrix, double *to, uintptr_t size);

MatrixHandle create_matrix(double *values, uint64_t cols, uint64_t rows);

MatrixHandle fast_ica(MatrixHandle whitened_matrix,
                      uint64_t n_components,
                      uint64_t max_iter,
                      double conv_threshold,
                      double alpha,
                      uint32_t cfid);

void free_handle(MatrixHandle hmatrix);

void free_svd_handle(SVDHandle hsvd);

void get_size(MatrixHandle hmatrix, uint64_t *cols, uint64_t *rows);

void move_matrix(MatrixHandle hmatrix, double *to, uintptr_t size);

SVDHandle normalized_svd(MatrixHandle hmatrix);

MatrixHandle pca_whitening(MatrixHandle hmatrix, const SVDHandle *svd_h);

void print_matrix(MatrixHandle hmatrix);

MatrixHandle reduced_dimension_repr(MatrixHandle hmatrix,
                                    MatrixHandle h_svd_u,
                                    uint64_t ncols);

MatrixHandle rotated_data_matrix(MatrixHandle hmatrix, MatrixHandle h_svd_u);

void set_backend(uintptr_t backend_id);

MatrixHandle zca_whitening(MatrixHandle hmatrix, SVDHandle svd_h);