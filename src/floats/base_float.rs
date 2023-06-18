//! base_float.rs contains functions that perform basic arithmetic operations on `f64` numbers. The functions are used by the `Float` struct.
//! The functions include:
//! * `add_float` - returns the sum of two `f64` numbers
//! * `subtract_float` - returns the difference of two `f64` numbers
//! * `multiply_float` - returns the product of two `f64` numbers
//! * `divide_float` - returns the quotient of two `f64` numbers
//! * `modulo_float` - returns the modulo of two `f64` numbers
//! * `power_float` - returns the first `f64` number raised to the power of the second `f64` number
//! * `root_float` - returns the first `f64` number raised to the power of the inverse of the second `f64` number
//!
//! # Examples
//! ```
//! use numbers_rus::floats::base_float;
//!
//! let result = base_float::add_float(1.0, 2.0);
//! assert_eq!(result, 3.0);
//! ```

 /// This function takes two `f64` numbers as arguments, `left` and `right`, and returns their sum as an `f64`. The calculation is performed using native
pub fn add_float(left: f64, right: f64) -> f64 {
        left + right
    }
/// This function takes two `f64` numbers as arguments, `left` and `right`, and returns their difference as an `f64` where left is substracted by right.
pub fn subtract_float(left: f64, right: f64) -> f64 {
        left - right
    }
/// This function takes two `f64` numbers as arguments, `left` and `right`, and returns their product as an `f64`. The calculation is performed using native
pub fn multiply_float(left: f64, right: f64) -> f64 {
        left * right
    }
/// This function takes two `f64` numbers as arguments, `left` and `right`, and returns their quotient as an `f64`. The calculation is performed using native
pub fn divide_float(left: f64, right: f64) -> f64 {
        left / right
    }
/// This function takes two `f64` numbers as arguments, `left` and `right`, and returns the result of the modulo operation (`left` modulo `right`) as an `f64`. The calculation is performed using the native modulo operator.
pub fn modulo_float(left: f64, right: f64) -> f64 {
        left % right
    }
/// This function takes two `f64` numbers as arguments, `left` and `right`, and returns the `left` raised to the power of `right` as an `f64`. The calculation is performed using the native `powf` method.
pub fn power_float(left: f64, right: f64) -> f64 {
        left.powf(right)
    }
/// This function takes two `f64` numbers as arguments: `left` and `right`, and returns the `left` raised to the power of `1 / right` as an `f64`. The calculation is performed using the native `powf` method.
pub fn root_float(left: f64, right: f64) -> f64 {
        let root = 1.0 / right;
    left.powf(root)
    }
/// Calculates the factorial of a given `f64` number. Returns the factorial result as an `f64`. It uses recursion to calculate the factorial.
pub fn factorial_float(number: f64) -> f64 {
        if number == 0.0 {
            1.0
        } else {
            number * factorial_float(number - 1.0)
        }
    }
/// Calculates the Fibonacci number at a given `f64` index, using recursion. Returns the Fibonacci result as `f64`.
pub fn fibonacci_float(number: f64) -> f64 {
        if number == 0.0 {
            0.0
        } else if number == 1.0 {
            1.0
        } else {
            fibonacci_float(number - 1.0) + fibonacci_float(number - 2.0)
        }
    }
/// Returns true if the given `f64` number is prime. This function checks for primality by iterating through numbers from 2 to the square root of the given number
pub fn is_prime_float(number: f64) -> bool {
        if number <= 1.0 {
            return false;
        }

    for i in 2..number as i128 {
        if number as i128 % i == 0 {
            return false;
        }
    }
    true
    }
/// Checks whether the given `f64` number is even. Returns true if the number is even, otherwise false. The function calculates the remainder of the input number divided by 2.0 and compares it to 0.0. If they are equal, the input number is even.
pub fn is_even_float(number: f64) -> bool {
        number % 2.0 == 0.0
    }
/// Checks whether the given `f64` number is odd. Returns true if the number is odd, otherwise false. The function calculates the remainder of the input number divided by 2.0 and compares it to 0.0. If they are not equal, the input number is odd.
pub fn is_odd_float(number: f64) -> bool {
        number % 2.0 != 0.0
    }
/// Determines whether `number` is a perfect square by comparing its square root squared with the original `number`. Returns true if the number is a perfect square, otherwise false.
pub fn is_perfect_square_float(number: f64) -> bool {
        let sqrt = number.sqrt();
        sqrt * sqrt == number
    }

/// Determines whether `number` is a perfect cube by comparing its cube root cubed with the original `number`. Returns true if the number is a perfect cube, otherwise false.
pub fn is_perfect_cube_float(number: f64) -> bool {
        let cbrt = number.cbrt();
        cbrt * cbrt * cbrt == number
    }
/// Determines if the given `f64` number is a perfect power by comparing its square root squared with the original number. Returns
/// true if the number is a perfect power, otherwise false.
pub fn is_perfect_power_float(number: f64) -> bool {
        let sqrt = number.sqrt();
        sqrt * sqrt == number
}