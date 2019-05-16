# FICA

This is the main implementation for FICA:

At the moment there's an ongoing implementation using:

1. "Arrayfire" to perform high performance computations in multiple devices.
2. "Rust" as a consumer of the arrayfire API and exposing a C interface (with cbindgen).