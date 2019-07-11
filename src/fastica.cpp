#include "../include/fastica.h"
#include <iostream>

using namespace Eigen;
using namespace contrast;
using namespace std;

double distance(const RowVectorXd &w, const RowVectorXd &nw)
{
    return abs(abs(w.cwiseProduct(nw).sum()) - 1.0);
}

VectorXd decorrelate(const VectorXd &row_mat, const MatrixXd &ret_w, size_t n_comps)
{

    VectorXd t(n_comps);

    for (size_t i = 0; i < n_comps - 1; ++i)
    {

        VectorXd rw_row_i = ret_w.row(i).transpose();

        auto k = row_mat.cwiseProduct(rw_row_i).sum();

        t += k * rw_row_i;
    }

    return row_mat - t;
}

MatrixXd *fast_ica_impl(
    const MatrixXd &matrix,
    const MatrixXd &ini_weights,
    unsigned int n_components,
    double conv_threshold,
    double alpha,
    ContrastFunction contrast_function,
    bool verbose)
{

    auto ret_weights = new MatrixXd(n_components, n_components);

    for (size_t comp_i = 0; comp_i < n_components; ++comp_i)
    {

        VectorXd wp = ini_weights.row(comp_i).transpose();

        if (verbose)
            cout << "Begin search for component " << comp_i << endl;

        if (comp_i > 0)
            wp = decorrelate(wp, *ret_weights, comp_i);

        wp = wp.normalized();

        for (size_t iter_i = 0;; ++iter_i)
        {

            if (verbose)
                cout << "Begin iteration " << iter_i << endl;

            auto wx = wp.transpose() * matrix;

            MatPair dxg_pair = contrast_function(wx, alpha);
            MatrixXd dg = get<0>(dxg_pair);
            MatrixXd d2g = get<1>(dxg_pair);

            MatrixXd xdg = matrix.cwiseProduct(dg);

            MatrixXd v1 = xdg.rowwise().mean();

            VectorXd v2 = d2g.mean() * wp;

            VectorXd nw = v1 - v2;

            if (comp_i > 0)
                nw = decorrelate(nw, *ret_weights, comp_i);

            nw = nw.normalized();

            double dist = distance(&wp, &nw);

            if (verbose)
                cout << "distance: " << dist << endl;

            wp.noalias() = nw;

            if (dist < conv_threshold)
            {
                cout << "converged in iteration: " << iter_i << endl;

                ret_weights->row(comp_i) = wp;
                break;
            }
        }
    }

    return ret_weights;
}

// TODO implement
MatrixXd *fastica::fast_ica(
    const Eigen::MatrixXd &dataset,
    const Eigen::MatrixXd &white_matrix,
    const Eigen::MatrixXd &weights,
    ICA_Params parameters)
{

    if (parameters.alpha < 1.0 && parameters.alpha > 2.0)
        cerr << "alpha must be between 1.0 and 2.0" << endl;

    return fast_ica_impl(
        dataset,
        weights,
        parameters.n_components,
        parameters.conv_threshold,
        parameters.alpha,
        get_contrast_function(parameters.cont_func_id),
        parameters.verbose);
}
