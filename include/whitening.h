#ifndef FICA_WHITENING_H
#define FICA_WHITENING_H

#ifdef __cplusplus
extern "C" {
#endif

    typedef enum {
        EIGEN = 0,
        PCA   = 1,
        ZCA   = 2
    } WhiteningTypeId;

#ifdef __cplusplus
}
#endif

#endif //FICA_WHITENING_H
