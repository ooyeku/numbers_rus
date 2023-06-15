# Numbers Library
**authors: "Ola Yeku"**
## Purpose:
This library is used to perform various numerical operations in Rust.  
The library is designed to be used in a variety of applications, including web services, command line utilities, and other Rust based applications.

## Download:
#### From crates.io:
This library can be downloaded from [crates.io](https://crates.io/crates/numbers_rus) using cargo:
```
cargo install numbers_rus
```
Add the following to your cargo.toml file:
```
[dependencies]
numbers_rus = "0.1.6"
```
Current crates.io version: 0.1.6


#### From Github:
The source code for this library can be found on [Github](https://github.com/ooyeku/numbers_rus)
and can be downloaded using git:
```
git clone https://github.com/ooyeku/numbers_rus.git
```
Current GitHub version: 0.1.7

## Examples:
The following examples show how to use this library:

``` 
// Import the library
use numbers_rus::numbers_rus::*
// Basic operations
add(1, 2)
subtract(1, 2)
multiply(1, 2)
divide(1, 2)
```
```
// Floating point operations
add_float(1.0, 2.0)
subtract_float(1.0, 2.0)
multiply_float(1.0, 2.0)
````
````
// Conditional Checks
is_even(2)
is_odd(2)
is_prime(2)
````
````
// Vector operations
vector_sum(vec![1, 2, 3])
vector_mean(vec![1, 2, 3])
vector_median(vec![1, 2, 3])
````

````
// Equation operations
let eq = Equation::new(2,4,'+');
let mut result = eq.solve();
eq.set_a(3);
let result = eq.solve();
````



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
- [ ] Add support for matrices
- [ ] Add support for tensors
### Version 0.2.0
- [ ] Add support for statistics
- [ ] Add support for probability
- [ ] Add support for linear algebra


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
