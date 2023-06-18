# numbers_rus
## a modular general purpose numbers library for Rust
### with the ambition of being useful and intuitive.


**authors: "ola yeku"**

## Purpose:
This library is used to perform various numerical operations in Rust.  
The library is designed to be used in a variety of applications, including web services, command line utilities, and other Rust based applications.
The project is currently in the early stages of development and is not yet ready for production use, but is available for testing and experimentation.
The library is designed to be easy to use and understand, and is designed to be used by both beginners and advanced users. 

Incremental updates will be made to the library as new features are added and bugs are fixed (possibly even introduced). 
Future versions of this library will include support for more advanced numerical operations, including data analysis, statistics, probability, and 
visualization.  Feel free to contribute to this project by submitting a pull request or by opening an issue and reporting and feedback or bugs.



## Download:
#### From crates.io:
This library can be downloaded from [crates.io](https://crates.io/crates/numbers_rus) using cargo:
```
cargo add numbers_rus
```
Add the following to your cargo.toml file:
```
[dependencies]
numbers_rus = "0.1.7"
```
Current crates.io version: 0.1.8


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
cargo test
```
open the documentation:
``` bash
cargo doc --open
```
Current GitHub version: 0.1.8

**Note:** The GitHub version may not be stable and will typically be ahead of the crates.io version.
## Examples:
The following examples show how to use this library:

``` rust
use numbers_rus::equation::equation::equation::Equation;
use rand::Rng;

fn main() {

    /// Demonstrates using the Equations struct from the equation module in the numbers_rus crate.
    /// The Example below creates an equation with two random numbers between 1 and 1,000,000 and
    /// then prints the equation and the solution.  It then checks to see if the solution is prime
    /// and if it is odd.  It then changes the operator to subtraction and repeats the process.

    // Create random number generator
    let mut rng = rand::thread_rng();

    // start timer
    let start = std::time::Instant::now();
    let reps = 100;
    for _ in 0..reps{
        let a = rng.gen_range(1..=1_000_000);
        let b = rng.gen_range(1..=1_000_000);
        let mut c = Equation::new(a, b, '+');
        println!("{} + {} = {}", a, b, c.get_sol());
        if numbers_rus::integers::base::base::is_prime(c.get_sol()) {
            println!("{} is prime", c.get_sol());
        } else {
            println!("{} is not prime", c.get_sol());
        }
        if numbers_rus::integers::base::base::is_odd(c.get_sol()) {
            println!("{} is odd", c.get_sol());
        } else {
            println!("{} is even", c.get_sol());
        }
        // Change operator
        c.set_operator('-');
        println!("{} - {} = {}", a, b, c.get_sol());
        if numbers_rus::integers::base::base::is_prime(c.get_sol()) {
            println!("{} is prime", c.get_sol());
        } else {
            println!("{} is not prime", c.get_sol());
        }
        if numbers_rus::integers::base::base::is_odd(c.get_sol()) {
            println!("{} is odd", c.get_sol());
        } else {
            println!("{} is even", c.get_sol());
        }
    }
    // end timer
    let duration = start.elapsed();
    println!("Time to run {} reps: {:?}",reps, duration);
}

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
- [x] Add support for vectors and matrices
### Version 0.2.0
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
