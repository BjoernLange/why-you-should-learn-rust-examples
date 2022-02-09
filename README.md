# Why you should learn Rust: Examples

This repository contains the examples from the talk "Why you should learn Rust".
All examples demonstrate that the Rust compiler catches hard to find errors at compilation time while many other languages fail at runtime or produce wrong and unexpected results.
Thus, the Rust example code usually does not compile.
Hints or reworked examples are included that show possible fixes.

## Running the examples

All examples except the Python one include `build.py` and `run.py` scripts that automate building and running so no specific knowledge about the languages or its tools is required.

The following tools are required on the `PATH`:

- Java: Java 11 or higher, Maven
- C++: GCC
- Python: Python 3.9 or higher
- Rust: A standard Rust installation including `cargo` and `rustc`, supporting the 2021 edition, e.g. version 1.57 or higher
