//
// Created by adriano on 7/8/19.
//

#ifndef FICA_CONTRAST_H
#define FICA_CONTRAST_H

#include <Eigen/Dense>

extern "C" {

    typedef enum {
        LOGCOSH = 0,
        EXPONENTIAL = 1
    } ContrastFunctionId;

}

namespace contrast {

    typedef std::tuple<Eigen::MatrixXd,Eigen::MatrixXd> MatPair;

    typedef MatPair (*ContrastFunction)(const Eigen::MatrixXd&, double);

    ContrastFunction get_contrast_function(ContrastFunctionId cfid);

    MatPair logcosh(const Eigen::MatrixXd& mat, double alpha);
    MatPair exp(const Eigen::MatrixXd& mat, double alpha);

}

#endif //FICA_CONTRAST_H
