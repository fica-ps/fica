// Autogenerated header *DO NOT* modify manually

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t MatrixHandler;

typedef struct {
  MatrixHandler u;
  MatrixHandler s;
  MatrixHandler v;
} SVDHandler;

MatrixHandler create_matrix(const float *values, uint64_t rows, uint64_t cols);

SVDHandler normalized_svd(MatrixHandler matrix);

void print_matrix(MatrixHandler matrix);
