#include <fica.h>
#include <fastica.h>
#include <Eigen/Dense>
#include <iostream>

using namespace fastica;
using namespace std;


int main() {
    double f[4] = { 1.1, 2.2, 3.3, 4.4 };

    auto f1 = (const double*)f;

    Matrix m = new_matrix(f1,2,2);

    print_matrix(m, "matrix:");

    free_matrix(m);
}
