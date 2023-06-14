pub mod base_float {
    /// Add for floating point numbers.
pub fn add_float(left: f64, right: f64) -> f64 {
        left + right
    }
/// Returns the difference of two floating point numbers.
pub fn subtract_float(left: f64, right: f64) -> f64 {
        left - right
    }
/// Multiply left and right floating point numbers.
pub fn multiply_float(left: f64, right: f64) -> f64 {
        left * right
    }
/// Returns the quotient of two floating point numbers.
pub fn divide_float(left: f64, right: f64) -> f64 {
        left / right
    }
/// Returns the remainder of two floating point numbers.
pub fn modulo_float(left: f64, right: f64) -> f64 {
        left % right
    }
/// Returns the left floating point number raised to the right floating point number.
pub fn power_float(left: f64, right: f64) -> f64 {
        left.powf(right)
    }
/// Returns the left floating point number to the root of the right floating point number.
pub fn root_float(left: f64, right: f64) -> f64 {
        let root = 1.0 / right;
        // pub fn root(left: i128, right: i128) -> i128 {
    //     let root = 1.0 / right as f64;
    //     (left as f64).powf(root).round() as i128
    // }
    left.powf(root)
    }
/// Returns the factorial of a floating point number.
pub fn factorial_float(number: f64) -> f64 {
        if number == 0.0 {
            1.0
        } else {
            number * factorial_float(number - 1.0)
        }
    }
/// Returns the fibbonacci of a floating point number.
pub fn fibonacci_float(number: f64) -> f64 {
        if number == 0.0 {
            0.0
        } else if number == 1.0 {
            1.0
        } else {
            fibonacci_float(number - 1.0) + fibonacci_float(number - 2.0)
        }
    }
/// Returns true if the floating point number is prime.
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
/// Returns true if the floating point number is even.
pub fn is_even_float(number: f64) -> bool {
        number % 2.0 == 0.0
    }
/// Returns true if the floating point number is odd.
pub fn is_odd_float(number: f64) -> bool {
        number % 2.0 != 0.0
    }
/// Returns true if the floating point number is perfect square.
pub fn is_perfect_square_float(number: f64) -> bool {
        let sqrt = number.sqrt();
        sqrt * sqrt == number
    }
/// Returns true if the floating point number is perfect cube.
pub fn is_perfect_cube_float(number: f64) -> bool {
        let cbrt = number.cbrt();
        cbrt * cbrt * cbrt == number
    }
/// Returns true if the floating point number is perfect power.
pub fn is_perfect_power_float(number: f64) -> bool {
        let sqrt = number.sqrt();
        sqrt * sqrt == number
    }
}