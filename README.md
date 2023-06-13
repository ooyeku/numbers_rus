# Numbers Library
**authors: "Ola Yeku"**
## Purpose:
This library is used to perform various numerical operations in Rust.  
The library is designed to be used in a variety of applications, including web services, command line utilities, and other Rust based applications.

## Testing:
To test this library, run the following command:
```
cargo test
```
## Usage: 
To use this library, add the numbers_rus crate to your project:
```
cargo add numbers_rus 
```


Add the following to your cargo.toml file:
```
[dependencies]
numbers_rus = "0.1.4"
```


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
The following features are planned for this library:

### Version 0.1.0
- [x] Basic operations
- [x] Add support for floating point numbers
- [ ] Add support for complex numbers
- [x] Add support for vectors
- [ ] Add support for matrices
- [ ] Add support for tensors
### Version 0.2.0
- [ ] Add support for statistics
- [ ] Add support for probability
- [ ] Add support for linear algebra
