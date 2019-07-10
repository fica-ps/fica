#include <fica.h>

int main() {

    double f[4] = { 1.1, 2.2, 3.3, 4.4 };

    Matrix m = new_matrix(f,2,2);

    print_matrix(m, "matrix:");

    free_matrix(m);

}