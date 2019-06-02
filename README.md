# FICA

This is the main implementation for FICA

At the moment there's an ongoing implementation using:

1. "Arrayfire" to perform high performance computations in multiple devices.
2. "Rust" as a consumer of the arrayfire API and exposing a C interface (with cbindgen).

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
