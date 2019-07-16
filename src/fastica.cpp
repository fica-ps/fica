#include "../include/fastica.h"
#include <iostream>
#include <time.h>

using namespace Eigen;
using namespace contrast;
using namespace std;

double conv_distance(const RowVectorXd& w, const RowVectorXd& nw)
{
    return abs(abs(w.cwiseProduct(nw).sum()) - 1.0);
}

VectorXd decorrelate(const VectorXd &row_mat, const MatrixXd &ret_w, size_t n_comps)
{

    VectorXd t = VectorXd::Constant(ret_w.cols(), 0.0);

    for (size_t i = 0; i < n_comps - 1; ++i)
    {

        VectorXd rw_row_i = ret_w.row(i).transpose();

        double k = row_mat.cwiseProduct(rw_row_i).sum();

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
    cout << "dataset" << endl << matrix << "\n\n" << endl;

    auto ret_weights = new MatrixXd(n_components, n_components);
    *ret_weights = MatrixXd::Zero(n_components, n_components);

    for (size_t comp_i = 0; comp_i < n_components; ++comp_i)
    {

        VectorXd wp = ini_weights.row(comp_i).transpose();

        if (verbose)
            cout << "Begin search for component " << comp_i << endl;

        if (comp_i > 0) {
            wp = decorrelate(wp, *ret_weights, comp_i);
            cout << "wp decorrelated" << endl << wp << "\n\n" << endl;
        }
        wp = wp.normalized();

        cout << "wp normalized" << endl << wp << "\n\n" << endl;

        for (size_t iter_i = 0;; ++iter_i) {

            if (verbose)
                cout << "Begin iteration " << iter_i << endl;

            MatrixXd wx = wp.transpose() * matrix;
            wx.eval();
            cout << "wx" << endl << wx << "\n\n" << endl;

            MatPair dxg_pair = contrast_function(wx, alpha);
            MatrixXd dg = get<0>(dxg_pair).array().colwise().replicate(2);
            MatrixXd d2g = get<1>(dxg_pair);

            cout << "dg" << endl << dg << "\n\n" << endl;

            cout << "d2g" << endl << d2g << "\n\n" << endl;

            MatrixXd xdg = matrix.cwiseProduct(dg);

            cout << "xdg" << endl << xdg << "\n\n" << endl;

            MatrixXd v1 = xdg.rowwise().mean();

            cout << "v1\n\n\n" << endl << v1 << "\n\n" << endl;

            cout << "d2g mean" << d2g.mean();

            VectorXd v2 = d2g.mean() * wp;

            cout << "v2" << endl << v2 << "\n\n" << endl;

            VectorXd nw = v1 - v2;

            cout << "nw" << endl << nw << "\n\n" << endl;

            if (comp_i > 0) {
                nw = decorrelate(nw, *ret_weights, comp_i);
                cout << "nw decorrelated" << endl << nw << "\n\n" << endl;
            }
            nw = nw.normalized();

            cout << "nw normalized" << endl << nw << "\n\n" << endl;

            double dist = conv_distance(wp, nw);

            if (verbose)
                cout << "distance: " << dist << endl;

            wp = nw;

            if (dist < conv_threshold)
            {
                cout << "converged in iteration: " << iter_i << endl;

                ret_weights->row(comp_i) = wp;
                cout << "ret_weights" << endl << *ret_weights << "\n\n" << endl;
                break;
            }
        }
    }

    return ret_weights;
}

// TODO implement
MatrixXd *fastica::fast_ica(
    const Eigen::MatrixXd &dataset,
    const Eigen::MatrixXd *white_matrix,
    const Eigen::MatrixXd *weights,
    ICA_Params parameters)
{

    if (parameters.alpha < 1.0 && parameters.alpha > 2.0) {
        cerr << "alpha must be between 1.0 and 2.0" << endl;
        return nullptr;
    }

    MatrixXd ws;
    if(weights == nullptr) {
        srand(parameters.seed != nullptr ? *parameters.seed : ((unsigned int)time(nullptr)));
        ws = MatrixXd::Random(parameters.n_components, parameters.n_components);
    } else {
        ws = *weights;
    }

    MatrixXd wt = white_matrix != nullptr ? white_matrix->transpose() : dataset;
    cout <<"wt\n\n" << wt << endl;
    MatrixXd dt = dataset * wt; // TODO whitening
    cout <<"dt\n\n" << dt << endl;
    return fast_ica_impl(
        dt.transpose(),
        ws,
        parameters.n_components,
        parameters.conv_threshold,
        parameters.alpha,
        get_contrast_function(parameters.cont_func_id),
        parameters.verbose);
}
