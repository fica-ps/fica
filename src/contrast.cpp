#include "../include/contrast.h"
#include <Eigen/Dense>

using namespace Eigen;

namespace contrast
{

    static ContrastFunction CONTRAST_FUNCTIONS[2] = {
        &contrast::logcosh,
        &contrast::exp
    };

    ContrastFunction get_contrast_function(ContrastFunctionId cfid)
    {
        return CONTRAST_FUNCTIONS[cfid];
    }

    MatPair logcosh(const Eigen::MatrixXd &mat, double alpha)
    {
        // todo
        MatrixXd factored_mat = alpha * mat;

        MatrixXd dg  = factored_mat.array().tanh();

        MatrixXd d2g = 1 - dg.array().pow(2);

        return MatPair(dg, alpha * d2g);
    }

    MatPair exp(const Eigen::MatrixXd &mat, double alpha)
    {
        MatrixXd dg  =  mat.array().pow(2) * 1.0 / alpha;

        MatrixXd d2g =  mat.array().pow(alpha - 1.0);

        return MatPair(dg, d2g);
    }

} // namespace contrast