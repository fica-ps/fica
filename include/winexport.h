#ifndef FICA_WINEXPORT_H
#define FICA_WINEXPORT_H

#ifndef _WINDOWS
    #define FICA_EXPORT
#elif COMPILING_DLL
    #define FICA_EXPORT __declspec(dllexport)
#else
    #define FICA_EXPORT __declspec(dllimport)
#endif

#endif