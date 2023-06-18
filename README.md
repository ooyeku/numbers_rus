# numbers_rus
## a modular general purpose numbers library for Rust with the ambition of being useful, performant, and easy to use.
[![Crates.io](https://img.shields.io/crates/v/numbers_rus.svg)](https://crates.io/crates/numbers_rus)
[![Documentation](https://docs.rs/numbers_rus/badge.svg)](https://docs.rs/numbers_rus)



**authors: Ola Yeku**

## Purpose:
This library is used to perform various numerical operations in Rust.  It is designed to be used in a variety of applications, including web services, command line utilities, and other Rust based applications.


The project is currently in the early stages of development and is not yet ready for production use, but is available for testing and experimentation.
The library is designed to be easy to use and understand, and is designed to be used by both beginners and advanced users. 

Incremental updates will be made to the library as new features are added and bugs are fixed (possibly even introduced). 
Future versions of this library will include support for more advanced numerical operations, including data analysis, statistics, probability, and 
visualization.  Feel free to contribute to this project by submitting a pull request or by opening an issue and reporting and feedback or bugs.

## Download:
#### From crates.io:
```
cargo add numbers_rus
```
Add the following to your cargo.toml file:
```
[dependencies]
numbers_rus = "0.2.0"
```
Current crates.io version: 0.2.0

#### From Github:
The source code for this library can be found on [Github](https://github.com/ooyeku/numbers_rus)
and can be downloaded using git:
```
git clone https://github.com/ooyeku/numbers_rus.git
```
change directory to numbers_rus and test the library:
```bash
cd numbers_rus
```
``` bash
cargo test --lib
```
open the documentation:
``` bash
cargo doc --open
```
Current GitHub version: 0.2.0

**Note:** The GitHub version may not be stable and will typically be ahead of the crates.io version.
## Examples:

examples can be found in the examples directory of the source code.
To run the examples, change directory to the source code directory and run the following command:  
```bash
cargo run --package numbers_rus --example solver --release
```


## Roadmap:

This is an active project and will be updated regularly.
Breaking changes may occur between versions, but will strive to achieve backwards compatibility.
The goal is to have a stable version by version 1.0.0.

The following features are planned for this library:

### Version 0.1.0
- [x] Basic operations
- [x] Add support for floating point numbers
- [x] Add support for complex numbers
- [x] Add support for vectors 
### Version 0.2.0
- [ ] Add support for matrices
- [ ] Add specialized data analysis functions
- [ ] Add support for graphics
### Version 0.3.0
- [ ] Add support for regression
- [ ] Add support for probability
- [ ] Add support for statistics


## Contributing:
Contributions are welcome and encouraged!
To contribute to this project, please follow the steps below:
1. Fork this repository
2. Create a new branch
3. Make your changes
4. Commit your changes
5. Push your changes to your fork
6. Submit a pull request
7. Wait for your pull request to be reviewed
8. Make any changes if requested
9. Wait for your pull request to be merged
10. Celebrate! and Thank you for your contribution!
