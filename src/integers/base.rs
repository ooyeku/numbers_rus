pub mod base {
    /// Returns the sum of two numbers.
pub fn add(left: i128, right: i128) -> i128 {
        left + right
    }
/// Returns the difference of two numbers.
pub fn subtract(left: i128, right: i128) -> i128 {
        left - right
    }
/// Multiply left and right numbers.
pub fn multiply(left: i128, right: i128) -> i128 {
        left * right
    }
/// Returns the quotient of two numbers.
pub fn divide(left: i128, right: i128) -> i128 {
        left / right
    }
/// Returns the remainder of two numbers.
pub fn modulo(left: i128, right: i128) -> i128 {
        left % right
    }
/// Returns the left number raised to the right number.
pub fn power(left: i128, right: i128) -> i128 {
        if right < 0 {
            panic!("Exponentiation with negative exponent is not supported");
        }

    let mut base = left;
        let mut result = 1;

        let mut exponent = right;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result *= base;
            }
            base *= base;
            exponent /= 2;
        }

    result
    }
/// Returns the left number to the root of the right number.
pub fn root(left: i128, right: i128) -> i128 {
        // Check for edge cases
    if left < 0 || right <= 0 {
        panic!("Invalid input: left should be non-negative, and right should be positive");
    }

    // Calculate the root
    let root = 1.0 / right as f64;
    (left as f64).powf(root).round() as i128
    }
/// Returns the factorial of a number.
pub fn factorial(number: i128) -> i128 {
        if number == 0 {
            1
        } else {
            number * factorial(number - 1)
        }
    }
/// Returns the fibonacci of a number.
pub fn fibonacci(number: i128) -> i128 {
        if number == 0 {
            0
        } else if number == 1 {
            1
        } else {
            fibonacci(number - 1) + fibonacci(number - 2)
        }
    }
/// Returns true if the number is prime.
pub fn is_prime(number: i128) -> bool {
        if number <= 1 {
            return false;
        }

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
    }
/// Returns true if the number is even.
pub fn is_even(number: i128) -> bool {
        number % 2 == 0
    }
/// Returns true if the number is odd.
pub fn is_odd(number: i128) -> bool {
        number % 2 != 0
    }
/// Returns true if the number is positive.
pub fn is_perfect_square(number: i128) -> bool {
        let sqrt = (number as f64).sqrt() as i128;
        sqrt * sqrt == number
    }
/// Returns true if the number is perfect cube.
pub fn is_perfect_cube(number: i128) -> bool {
        let cbrt = (number as f64).cbrt() as i128;
        cbrt * cbrt * cbrt == number
    }
/// Returns true if the number is perfect power.
pub fn is_perfect_power(number: i128) -> bool {
        let sqrt = (number as f64).sqrt() as i128;
        sqrt * sqrt == number
    }
}