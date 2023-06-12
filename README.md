# Numbers Library
**authors: "Ola Yeku"**
## Purpose:
This library is used to perform various numberical operations in Rust.  
The library is designed to be used in a variety of applications, including web services, command line utilities, and other Rust based applications.

## How to build:
This library is built using the standard Rust build tools.  To build the library, run the following command:

```
cargo build
```

```
cargo test
```
## Usage: 
To use this library, add the following to your Cargo.toml file:

```
[dependencies]
numbers = { git = "https://git.jetbrains.space/dataalchemist/prod/numbers.git" }
```

## Examples:
The following examples show how to use this library:

``` 
// Import the library
use numbers;
// Basic operations
add(1, 2)
subtract(1, 2)
multiply(1, 2)
divide(1, 2)
```



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