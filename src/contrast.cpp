#include "../include/contrast.h"
#include <Eigen/Dense>

namespace contrast
{

static ContrastFunction CONTRAST_FUNCTIONS[2] = {
    &contrast::logcosh,
    &contrast::exp};

ContrastFunction get_contrast_function(ContrastFunctionId cfid)
{
    return CONTRAST_FUNCTIONS[cfid];
}

MatPair logcosh(const Eigen::MatrixXd &mat, double alpha)
{
    // todo
    auto x = Eigen::MatrixXd(0, 0);
    return MatPair(x, x);
}

MatPair exp(const Eigen::MatrixXd &mat, double alpha)
{
    return logcosh(mat, alpha); // todo
}

} // namespace contrast