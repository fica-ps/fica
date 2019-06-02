# FICA

This is the main implementation for FICA

At the moment there's an ongoing implementation using:

1. "Arrayfire" to perform high performance computations in multiple devices.
2. "Rust" as a consumer of the arrayfire API and exposing a C/C++ interface (with cbindgen).

## Pre-requisites:

To use this library, you need to install [Arrayfire](http://arrayfire.org/docs/installing.htm).

In order to build the library you also need to install the [rust toolchain](https://rustup.rs/).

## Building the libraries

To build the library open the terminal in the desired parent directory. 
Then clone this repo using:

1. ``git clone https://github.com/fica-ps/fica.git`` or ``git clone https://github.com/fica-ps/fica.git <directory>`` to clone into ``<directory>``.

2. 3hange directory into the directory where the repo was cloned.

3. Finally build it using the command ``cargo build --release``.

The necessary SO/DLL and static library stubs will be generated on ``<cloned-directory>/target/release/``.

## header file location

There are two header files one for linking with C consumer, the other to link with a C++ consumer:

* C header: ``<cloned-directory>/include/c/``

* Cpp header: ``<cloned-directory>/include/cpp/``

## Example: transforming an array into Matrix and back:

```C
#include <stdio.h>
#include <fica.h>


int main()
{
	
	double arr[6]   = { 1.0, 2.0, 3.0, 4.0, 5.0, 6.0};
	Matrix mhandle = create_matrix(arr, 3, 2);
	
	print_matrix(mhandle);

	double ret_mat[6] = {0.0};
	uint64_t cols = 0;
	uint64_t lines = 0;

	matrix_dims(mhandle, &cols, &lines);

	copy_matrix(mhandle, ret_mat, cols * lines); 
	free_matrix(mhandle); 
	// or move_matrix(mhandler, ret_mat, cols * lines) - which frees the handle.
	

	printf("Matrix %lldx%lld:\n", cols, lines);
	
	for (int c = 0; c < cols; ++c) {
		printf("| ");
		for (int l = 0;l < lines; ++l) {
			printf("%f ",ret_mat[l*cols + c]);
		}
		printf("|\n");
	}
}
```

output:
```
[2 3 1 1]
    1.0000     3.0000     5.0000
    2.0000     4.0000     6.0000

Matrix 2x3:
| 1.000000 3.000000 5.000000 |
| 2.000000 4.000000 6.000000 |
```
