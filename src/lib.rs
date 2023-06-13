
/// Returns the sum of two numbers.
pub fn add(left: i128, right: i128) -> i128 {
    left + right
}
/// Add for floating point numbers.
pub fn add_float(left: f64, right: f64) -> f64 {
    left + right
}
#[cfg(test)]
mod test_add {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn add_negatives() {
        let result = add(-2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn add_floats() {
        let result = add_float(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}

/// Returns the difference of two numbers.
pub fn subtract(left: i128, right: i128) -> i128 {
    left - right
}
/// Returns the difference of two floating point numbers.
pub fn subtract_float(left: f64, right: f64) -> f64 {
    left - right
}
#[cfg(test)]
mod test_subtract {
    use super::*;

    #[test]
    fn it_works() {
        let result = subtract(2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn subtract_negatives() {
        let result = subtract(-2, 2);
        assert_eq!(result, -4);
    }
}

/// Multiply left and right numbers.
pub fn multiply(left: i128, right: i128) -> i128 {
    left * right
}
/// Multiply left and right floating point numbers.
pub fn multiply_float(left: f64, right: f64) -> f64 {
    left * right
}
#[cfg(test)]
mod test_multiply {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiply(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn multiply_negatives() {
        let result = multiply(-2, 2);
        assert_eq!(result, -4);
    }
    #[test]
    fn multiply_floats() {
        let result = multiply_float(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}

/// Returns the quotient of two numbers.
pub fn divide(left: i128, right: i128) -> i128 {
    left / right
}
#[cfg(test)]
mod test_divide {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(2, 2);
        assert_eq!(result, 1);
    }
}

/// Returns the quotient of two floating point numbers.
pub fn divide_float(left: f64, right: f64) -> f64 {
    left / right
}
/// Returns the remainder of two numbers.
pub fn modulo(left: i128, right: i128) -> i128 {
    left % right
}
/// Returns the remainder of two floating point numbers.
pub fn modulo_float(left: f64, right: f64) -> f64 {
    left % right
}
#[cfg(test)]
mod test_modulo {
    use super::*;

    #[test]
    fn it_works() {
        let result = modulo(2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn modulo_negatives() {
        let result = modulo(-2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn modulo_floats() {
        let result = modulo_float(2.0, 2.0);
        assert_eq!(result, 0.0);
    }
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

/// Returns the left floating point number raised to the right floating point number.
pub fn power_float(left: f64, right: f64) -> f64 {
    left.powf(right)
}
#[cfg(test)]
mod test_power {
    use super::*;

    #[test]
    fn it_works() {
        let result = power(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn power_negatives() {
        let result = power(-2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn power_floats() {
        let result = power_float(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
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
/// Returns the left floating point number to the root of the right floating point number.
pub fn root_float(left: f64, right: f64) -> f64 {
    let root = 1.0 / right;pub fn root(left: i128, right: i128) -> i128 {
        let root = 1.0 / right as f64;
        (left as f64).powf(root).round() as i128
    }
    left.powf(root)
}
#[cfg(test)]
mod test_root {
    use super::*;

    #[test]
    fn it_works() {
        let result = root(4, 2);
        assert_eq!(result, 2);
    }
    #[test]
    fn root_floats() {
        let result = root_float(4.0, 2.0);
        assert_eq!(result, 2.0);
    }
}

/// Returns the factorial of a number.
pub fn factorial(number: i128) -> i128 {
    if number == 0 {
        1
    } else {
        number * factorial(number - 1)
    }
}
/// Returns the factorial of a floating point number.
pub fn factorial_float(number: f64) -> f64 {
    if number == 0.0 {
        1.0
    } else {
        number * factorial_float(number - 1.0)
    }
}
#[cfg(test)]
mod test_factorial {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(4);
        assert_eq!(result, 24);
    }
    #[test]
    fn factorial_floats() {
        let result = factorial_float(4.0);
        assert_eq!(result, 24.0);
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
#[cfg(test)]
mod test_fibonacci {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibonacci(4);
        assert_eq!(result, 3);
    }
    #[test]
    fn fibonacci_floats() {
        let result = fibonacci_float(4.0);
        assert_eq!(result, 3.0);
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
#[cfg(test)]
mod test_is_prime {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_prime(4);
        assert_eq!(result, false);
    }
    #[test]
    fn is_prime_floats() {
        let result = is_prime_float(4.0);
        assert_eq!(result, false);
    }
}

/// Returns true if the number is even.
pub fn is_even(number: i128) -> bool {
    number % 2 == 0
}
/// Returns true if the floating point number is even.
pub fn is_even_float(number: f64) -> bool {
    number % 2.0 == 0.0
}
#[cfg(test)]
mod test_is_even {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_even(4);
        assert_eq!(result, true);
    }
    #[test]
    fn is_even_floats() {
        let result = is_even_float(4.0);
        assert_eq!(result, true);
    }
}

/// Returns true if the number is odd.
pub fn is_odd(number: i128) -> bool {
    number % 2 != 0
}
/// Returns true if the floating point number is odd.
pub fn is_odd_float(number: f64) -> bool {
    number % 2.0 != 0.0
}
#[cfg(test)]
mod test_is_odd {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_odd(4);
        assert_eq!(result, false);
    }
    #[test]
    fn is_odd_floats() {
        let result = is_odd_float(4.0);
        assert_eq!(result, false);
    }
}


/// Returns true if the number is positive.
pub fn is_perfect_square(number: i128) -> bool {
    let sqrt = (number as f64).sqrt() as i128;
    sqrt * sqrt == number
}
/// Returns true if the floating point number is perfect square.
pub fn is_perfect_square_float(number: f64) -> bool {
    let sqrt = number.sqrt();
    sqrt * sqrt == number
}
#[cfg(test)]
mod test_is_perfect_square {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_perfect_square(4);
        assert_eq!(result, true);
    }
    #[test]
    fn is_perfect_square_floats() {
        let result = is_perfect_square_float(4.0);
        assert_eq!(result, true);
    }
}

/// Returns true if the number is perfect cube.
pub fn is_perfect_cube(number: i128) -> bool {
    let cbrt = (number as f64).cbrt() as i128;
    cbrt * cbrt * cbrt == number
}
/// Returns true if the floating point number is perfect cube.
pub fn is_perfect_cube_float(number: f64) -> bool {
    let cbrt = number.cbrt();
    cbrt * cbrt * cbrt == number
}
#[cfg(test)]
mod test_is_perfect_cube {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_perfect_cube(4);
        assert_eq!(result, false);
    }
    #[test]
    fn is_perfect_cube_floats() {
        let result = is_perfect_cube_float(4.0);
        assert_eq!(result, false);
    }
}

/// Returns true if the number is perfect power.
pub fn is_perfect_power(number: i128) -> bool {
    let sqrt = (number as f64).sqrt() as i128;
    sqrt * sqrt == number
}
/// Returns true if the floating point number is perfect power.
pub fn is_perfect_power_float(number: f64) -> bool {
    let sqrt = number.sqrt();
    sqrt * sqrt == number
}
#[cfg(test)]
mod test_is_perfect_power {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_perfect_power(4);
        assert_eq!(result, true);
    }
    #[test]
    fn is_perfect_power_floats() {
        let result = is_perfect_power_float(4.0);
        assert_eq!(result, true);
    }
}

// Single Vector Operations ------------------------------------------------------------------------

/// Returns the sum of all elements in a vector
pub fn vector_sum(vector: Vec<i128>) -> i128 {
    vector.iter().sum()
}
/// Returns the sum of all elements in a float vector
pub fn vector_sum_float(vector: Vec<f64>) -> f64 {
    vector.iter().sum()
}
#[cfg(test)]
mod test_vector_sum {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_sum(vec![1, 2, 3]);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works_floats() {
        let result = vector_sum_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 6.0);
    }
}

/// Returns the product of all elements in a vector
pub fn vector_product(vector: Vec<i128>) -> i128 {
    vector.iter().fold(1, |acc, &x| acc * x)
}
/// Returns the product of all elements ina  vector
pub fn vector_product_float(vector: Vec<f64>) -> f64 {
    vector.iter().fold(1.0, |acc, &x| acc * x)
}
#[cfg(test)]
mod test_vector_product {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_product(vec![1, 2, 3]);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works_floats() {
        let result = vector_product_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 6.0);
    }
}

/// Returns the mean of all elements in a vector
pub fn vector_mean(vector: Vec<i128>) -> i128 {
    vector_sum(vector.clone()) / vector.len() as i128
}
/// Returns the mean of all elements in a float vector
pub fn vector_mean_float(vector: Vec<f64>) -> f64 {
    vector_sum_float(vector.clone()) / vector.len() as f64
}
#[cfg(test)]
mod test_vector_mean {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_mean(vec![1, 2, 3]);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works_floats() {
        let result = vector_mean_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 2.0);
    }
}

/// Returns the median of all elements in a vector
pub fn vector_median(vector: Vec<i128>) -> i128 {
    let mut sorted_vector = vector.clone();
    sorted_vector.sort();

    if sorted_vector.len() % 2 == 0 {
        (sorted_vector[sorted_vector.len() / 2] + sorted_vector[sorted_vector.len() / 2 - 1]) / 2
    } else {
        sorted_vector[sorted_vector.len() / 2]
    }
}
/// Returns the median of all elements in a float vector
pub fn vector_median_float(vector: Vec<f64>) -> f64 {
    let mut sorted_vector = vector.clone();
    sorted_vector.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if sorted_vector.len() % 2 == 0 {
        (sorted_vector[sorted_vector.len() / 2] + sorted_vector[sorted_vector.len() / 2 - 1]) / 2.0
    } else {
        sorted_vector[sorted_vector.len() / 2]
    }
}
#[cfg(test)]
mod test_vector_median {
    use super::*;

    #[test]
    fn it_works() {
        let result = two_vector_median(vec![1, 2, 3], vec![4, 5, 6]);
        println!("{}", result.len());
    }
    #[test]
    fn it_works_floats() {
        let result = vector_median_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 2.0);
    }
}

/// Returns the mode of all elements in a vector
use std::collections::HashMap;
pub fn vector_mode(vector: Vec<i128>) -> i128 {
    let mut counts = HashMap::new();

    for &number in vector.iter() {
        let count = counts.entry(number).or_insert(0);
        *count += 1;
    }

    counts.into_iter()
          .max_by_key(|&(_key, value)| value)
          .map(|(key, _value)| key)
          .unwrap_or(0)
}
/// Returns the mode of all elements in a vector of floating points.
use ordered_float::OrderedFloat;
pub fn vector_mode_float(vector: Vec<f64>) -> f64 {
    let mut counts: HashMap<OrderedFloat<f64>, usize> = HashMap::new();
    let epsilon = 1e-9; // Adjust this value according to your desired precision

    for &number in vector.iter() {
        let key = counts
            .keys()
            .find(|&key| ((**key) - number).abs() < epsilon)
            .cloned();

        match key {
            Some(existing_key) => {
                let count = counts.get_mut(&existing_key).unwrap();
                *count += 1;
            }
            None => {
                counts.insert(OrderedFloat(number), 1);
            }
        }
    }

    counts.into_iter()
          .max_by_key(|&(_key, value)| value)
          .map(|(key, _value)| *key)
          .unwrap_or(f64::NAN)
}
#[cfg(test)]
mod test_vector_mode {
    use super::*;

    #[test]
    fn it_works() {
        let result = two_vector_mode(vec![1, 2, 3], vec![4, 5, 6]);
        println!("{}", result.len());
    }
     #[test]
     fn it_works_floats() {
         let result = vector_mode_float(vec![1.0, 2.0, 3.0, 1.0]);
         assert_eq!(result, 1.0);
     }
}

/// Returns the range of all elements in a vector
pub fn vector_range(vector: Vec<i128>) -> i128 {
    let mut sorted = vector.clone();
    sorted.sort();
    sorted[sorted.len() - 1] - sorted[0]
}
/// Returns the range of all elements in a float vector
pub fn vector_range_float(vector: Vec<f64>) -> f64 {
    let mut sorted = vector.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted[sorted.len() - 1] - sorted[0]
}
#[cfg(test)]
mod test_vector_range {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_range(vec![1, 2, 3]);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works_floats() {
        let result = vector_range_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 2.0);
    }
}
// -------------------------------------------------------------------------------------------------
/// Returns the inter quartile range of all elements in a vector
pub fn vector_interquartile_range(vector: Vec<i128>) -> i128 {
    let mut sorted = vector.clone();
    sorted.sort();
    let q1 = vector_median(sorted[0..sorted.len() / 2].to_vec());
    let q3 = vector_median(sorted[sorted.len() / 2..sorted.len()].to_vec());
    q3 - q1
}
/// Returns the inter quartile range of all elements in a float vector
pub fn vector_interquartile_range_float(vector: Vec<f64>) -> f64 {
    let mut sorted = vector.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q1 = vector_median_float(sorted[0..sorted.len() / 2].to_vec());
    let q3 = vector_median_float(sorted[sorted.len() / 2..sorted.len()].to_vec());
    q3 - q1
}
#[cfg(test)]
mod test_vector_interquartile_range {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_interquartile_range(vec![1, 2, 3]);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works_floats() {
        let result = vector_interquartile_range_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 1.5);
    }
}
// -------------------------------------------------------------------------------------------------
/// Returns the variance of all elements in a vector
pub fn vector_variance(vector: Vec<i128>) -> String {
    let mean = vector_mean(vector.clone());
    let mut sum = 0;

    for i in 0..vector.len() {
        sum += (vector[i] - mean).pow(2);
    }

    format!("{:.2}", sum as f64 / vector.len() as f64)
}
/// Returns the variance of all elements in a float vector
pub fn vector_variance_float(vector: Vec<f64>) -> f64 {
    let mean = vector_mean_float(vector.clone());
    let mut sum = 0.0;

    for i in 0..vector.len() {
        sum += (vector[i] - mean).powf(2.0);
    }

    sum / vector.len() as f64
}
#[cfg(test)]
mod test_vector_variance {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_variance(vec![1, 2, 3]);
        assert_eq!(result, "0.67");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_variance_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 0.6666666666666666);
    }
}

/// Returns the standard deviation of all elements in a vector
pub fn vector_standard_deviation(vector: Vec<i128>) -> String {
    let variance = vector_variance(vector.clone());
    format!("{:.2}", variance.parse::<f64>().unwrap().sqrt())
}
/// Returns the standard deviation of all elements in a float vector
pub fn vector_standard_deviation_float(vector: Vec<f64>) -> f64 {
    let variance = vector_variance_float(vector.clone());
    variance.sqrt()
}
#[cfg(test)]
mod test_vector_standard_deviation {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_standard_deviation(vec![1, 2, 3]);
        assert_eq!(result, "0.82");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_standard_deviation_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 0.816496580927726);
    }
}

/// Returns the quartiles of all elements in a vector
pub fn vector_quartiles(vector: Vec<i128>) -> String {
    let mut sorted = vector.clone();
    sorted.sort();

    let q1 = sorted[sorted.len() / 4];
    let q2 = sorted[sorted.len() / 2];
    let q3 = sorted[sorted.len() * 3 / 4];

    format!("Q1: {}, Q2: {}, Q3: {}", q1, q2, q3)
}
/// Returns the quartiles of all elements in a float vector
pub fn vector_quartiles_float(vector: Vec<f64>) -> String {
    let mut sorted = vector.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let q1 = sorted[sorted.len() / 4];
    let q2 = sorted[sorted.len() / 2];
    let q3 = sorted[sorted.len() * 3 / 4];

    format!("Q1: {}, Q2: {}, Q3: {}", q1, q2, q3)
}
#[cfg(test)]
mod test_vector_quartiles {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_quartiles(vec![1, 2, 3]);
        assert_eq!(result, ("Q1: 1, Q2: 2, Q3: 3"));
    }
    #[test]
    fn it_works_floats() {
        let result = vector_quartiles_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, ("Q1: 1, Q2: 2, Q3: 3"));
    }
}

// Multi Vector Operations -------------------------------------------------------------------------
use itertools::join;

/// Returns the sum of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise sum of the two input vectors
pub fn vector_add(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a + b)
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the sum of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise sum of the two input float vectors
pub fn vector_add_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a + b)
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_add {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_add(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "2, 4, 6");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_add_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "2, 4, 6");
    }
}

/// Returns the difference of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise difference of the two input vectors
pub fn vector_subtract(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a - b)
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the difference of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise difference of the two input float vectors
pub fn vector_subtract_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a - b)
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_subtract {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_subtract(vec![1, 2, 3], vec![1, 2, 3]);

        assert_eq!(result, "0, 0, 0");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_subtract_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);

        assert_eq!(result, "0, 0, 0");
    }
}

/// Returns the product of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise product of the two input vectors
pub fn vector_multiply(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a * b)
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the product of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise product of the two input float vectors
pub fn vector_multiply_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a * b)
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_multiply {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_multiply(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 4, 9");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_multiply_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 4, 9");
    }
}
/// Returns the quotient of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise quotient of the two input vectors
pub fn vector_divide(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a / b)
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the quotient of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise quotient of the two input float vectors
pub fn vector_divide_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a / b)
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_divide {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_divide(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 1, 1");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_divide_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 1, 1");
    }
}

/// Returns the remainder of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise remainder of the two input vectors
pub fn vector_modulo(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a % b)
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the modulo of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise modulo of the two input float vectors
pub fn vector_modulo_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a % b)
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}

#[cfg(test)]
mod test_vector_modulo {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_modulo(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "0, 0, 0");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_modulo_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "0, 0, 0");
    }
}

/// Returns the power of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise power of the two input vectors
pub fn vector_power(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.pow(b as u32))
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the power of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise power of the two input float vectors
pub fn vector_power_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.powf(b))
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_power {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_power(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 4, 27");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_power_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 4, 27");
    }
}

/// Returns the root of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise root of the two input vectors
pub fn vector_root(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.pow((1.0 / (b as f64)).round() as u32))
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the root of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise root of the two input float vectors
pub fn vector_root_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.powf(1.0 / b))
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_root {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_root(vec![1, 4, 27], vec![1, 2, 3]);
        assert_eq!(result, "1, 4, 1");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_root_float(vec![1.0, 4.0, 27.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}

/// Returns the minimum of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise minimum of the two input vectors
pub fn vector_min(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.min(b))
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the minimum of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise minimum of the two input float vectors
pub fn vector_min_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.min(b))
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_min {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_min(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_min_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}

/// Returns the maximum of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise maximum of the two input vectors
pub fn vector_max(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.max(b))
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the maximum of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise maximum of the two input float vectors
pub fn vector_max_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| a.max(b))
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_max {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_max(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_max_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}

/// Returns the average of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise average of the two input vectors
pub fn vector_average(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| (a + b) / 2)
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the average of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise average of the two input float vectors
pub fn vector_average_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| (a + b) / 2.0)
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_vector_average {
    use super::*;

    #[test]
    fn it_works() {
        let result = vector_average(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = vector_average_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}

/// Returns the median of two vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
///
/// # Returns
///
/// * A string representing the element-wise median of the two input vectors
pub fn two_vector_median(left: Vec<i128>, right: Vec<i128>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| (a + b) / 2)
                     .collect::<Vec<i128>>();

    join(&result, ", ")
}

/// Returns the median of two float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
///
/// # Returns
///
/// * A string representing the element-wise median of the two input float vectors
pub fn two_vector_median_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let result = left.into_iter()
                     .zip(right.into_iter())
                     .map(|(a, b)| (a + b) / 2.0)
                     .collect::<Vec<f64>>();

    join(&result, ", ")
}
#[cfg(test)]
mod test_two_vector_median {
    use super::*;

    #[test]
    fn it_works() {
        let result = two_vector_median(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = two_vector_median_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}

/// Returns the mode of two vectors
pub fn two_vector_mode(left: Vec<i128>, right: Vec<i128>) -> String {
    let mut result = Vec::new();

    for i in 0..left.len() {
        result.push((left[i] + right[i]) / 2);
    }
    // since Vec<usize> doesn't implement Display, we have to convert it to a string
    let mut string = String::new();
    for i in 0..result.len() {
        string.push_str(&result[i].to_string());
        // add a comma after each element except the last one
        if i != result.len() - 1 {
            string.push_str(", ");
        }
    }
    string
}
/// Returns the mode of two vectors float.
pub fn two_vector_mode_float(left: Vec<f64>, right: Vec<f64>) -> String {
    let mut result = Vec::new();

    for i in 0..left.len() {
        result.push((left[i] + right[i]) / 2.0);
    }
    // since Vec<usize> doesn't implement Display, we have to convert it to a string
    let mut string = String::new();
    for i in 0..result.len() {
        string.push_str(&result[i].to_string());
        // add a comma after each element except the last one
        if i != result.len() - 1 {
            string.push_str(", ");
        }
    }
    string
}
#[cfg(test)]
mod test_two_vector_mode {
    use super::*;

    #[test]
    fn it_works() {
        let result = two_vector_mode(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = two_vector_mode_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}

// Dataframe functions  ----------------------------------------------------------------------------

/// A dataframe struct similar to pythons pandas dataframe but with less functionality and only for integers
/// The dataframe is stored as a vector of vectors
/// The first vector contains the column names
/// The rest of the vectors contain the data
/// The data is stored as a vector of vectors
///
/// # Example
/// ```
/// use numbers_rus::DataFrame;
/// let mut df = DataFrame::new();
/// df.add_column("a", &vec![1, 2, 3]);
/// df.add_column("b", &vec![4, 5, 6]);
/// df.add_column("c", &vec![7, 8, 9]);
/// ```
///
pub struct DataFrame {
    columns: Vec<String>,
    data: Vec<Vec<i128>>,
}
impl DataFrame {
    /// Creates a new dataframe
    pub fn new() -> DataFrame {
        DataFrame {
            columns: Vec::new(),
            data: Vec::new(),
        }
    }
    /// Adds a column to the dataframe
    pub fn add_column(&mut self, name: &str, data: &Vec<i128>) {
        self.columns.push(name.to_string());
        self.data.push(data.clone());
    }
    /// Returns the column names
    pub fn get_columns(&self) -> Vec<String> {
        self.columns.clone()
    }
    /// Returns the data
    pub fn get_data(&self) -> Vec<Vec<i128>> {
        self.data.clone()
    }
    /// Returns the column names
    pub fn get_column(&self, name: &str) -> Result<Vec<i128>, &'static str> {
        let mut result = Vec::new();

        for i in 0..self.columns.len() {
            if self.columns[i] == name {
                result = self.data[i].clone();
            }
        }
        if result.len() == 0 {
            return Err("Column not found");
        }
        Ok(result)
    }
    /// Returns the column names
    pub fn get_column_index(&self, name: &str) -> Result<i128, &'static str> {
        let mut result = 0;

        for i in 0..self.columns.len() {
            if self.columns[i] == name {
                result = i;
            }
        }

        if result == 0 {
            return Err("Column not found");
        }

        Ok(result as i128)
    }
    /// Returns the column names
    pub fn get_column_name(&self, index: i128) -> Result<String, &'static str> {
        if index > self.columns.len() as i128 {
            return Err("Column not found");
        }

        Ok(self.columns[index as usize].clone())
    }
    /// Returns the column names
    pub fn get_column_count(&self) -> i128 {
        self.columns.len() as i128
    }
    /// Returns the column names
    pub fn get_row_count(&self) -> usize {
        self.data[0].len()
    }
}

/// Creates a new dataframe
pub fn dataframe_create(
    columns: Vec<String>,
    data: Vec<Vec<i128>>,
) -> Result<DataFrame, &'static str> {
    if columns.len() != data.len() {
        return Err("Column count does not match data count");
    }

    for i in 0..data.len() {
        if data[i].len() != data[0].len() {
            return Err("Data length does not match");
        }
    }

    Ok(DataFrame {
        columns,
        data,
    })
}
#[cfg(test)]
mod test_dataframe_create {
    use super::*;

    #[test]
    fn it_works() {
        let result = dataframe_create(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
        );

        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn display_dataframe() {
        let result = dataframe_create(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
        );

        assert_eq!(
            result.unwrap().get_columns(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}

/// Equation struct that can be used to solve equations with two numbers and an operator (+, -, *, /, %, ^)
/// Sol is the solution to the equation and is calculated when get_sol() is called.
///
/// # Example
/// ```
/// use numbers_rus::Equation;
/// let mut equation = Equation::new(1, 2, '+');
/// assert_eq!(equation.get_sol(), 3);
/// ```
pub struct Equation {
    a: i128,
    b: i128,
    operator: char,
    sol: i128,
}
impl Equation {
    /// Creates a new equation
    pub fn new(a: i128, b: i128, operator: char) -> Equation {
        Equation {
            a: a,
            b: b,
            operator: operator,
            sol: 0,
        }
    }
    /// Returns the solution to the equation
    pub fn get_sol(&mut self) -> i128 {
        if self.sol == 0 {
            self.sol = match self.operator {
                '+' => add(self.a, self.b),
                '-' => subtract(self.a, self.b),
                '*' => multiply(self.a, self.b),
                '/' => divide(self.a, self.b),
                '%' => modulo(self.a, self.b),
                '^' => power(self.a, self.b),
                _ => 0,
            };
        }
        self.sol
    }
    /// sets the a value (first number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_a(&mut self, a: i128) {
        self.a = a;
        self.sol = 0; // reset the solution
    }
    /// sets the b value (second number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_b(&mut self, b: i128) {
        self.b = b;
        self.sol = 0; // reset the solution
    }
    /// sets the operator in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_operator(&mut self, operator: char) {
        self.operator = operator;
        self.sol = 0; // reset the solution
    }
    /// returns the a value
    pub fn get_a(&self) -> i128 {
        self.a
    }
    /// returns the b value
    pub fn get_b(&self) -> i128 {
        self.b
    }
    /// returns the operator
    pub fn get_operator(&self) -> char {
        self.operator
    }
}
#[cfg(test)]
mod test_equation {
    use super::*;

    #[test]
    fn it_works() {
        let mut equation = Equation::new(1, 2, '+');
        assert_eq!(equation.get_sol(), 3);
    }
    #[test]
    fn set_a() {
        let mut equation = Equation::new(1, 2, '+');
        equation.set_a(3);
        assert_eq!(equation.get_a(), 3);
        assert_eq!(equation.get_sol(), 5);
    }
    #[test]
    fn set_b() {
        let mut equation = Equation::new(1, 2, '+');
        equation.set_b(3);
        assert_eq!(equation.get_b(), 3);
        assert_eq!(equation.get_sol(), 4);
    }
    #[test]
    fn set_operator() {
        let mut equation = Equation::new(1, 2, '+');
        equation.set_operator('-');
        assert_eq!(equation.get_operator(), '-');
        assert_eq!(equation.get_sol(), -1);
    }
    #[test]
    fn change_equation(){
        let mut equation = Equation::new(1, 2, '+');
        assert_eq!(equation.get_sol(), 3);
        equation.set_a(3);
        println!("a: {}", equation.get_a());
        println!("b: {}", equation.get_b());
        println!("sol: {}", equation.get_sol());
        assert_eq!(equation.get_sol(), 5);
        equation.set_b(3);
        println!("a: {}", equation.get_a());
        println!("b: {}", equation.get_b());
        assert_eq!(equation.get_sol(), 6);
    }
}

/// EquationF is an equation similar to Equation, but with f64 values.  Sol is calculated when get_sol() is called.
/// # Example
/// ```
/// use numbers_rus::EquationF;
/// let mut equation = EquationF::new(1.092, 2.435, '+');
/// assert_eq!(equation.get_sol(), 3.527);
/// ```

pub struct EquationF {
    a: f64,
    b: f64,
    operator: char,
    sol: f64,
}
impl EquationF {
    pub fn new(a: f64, b: f64, operator: char) -> EquationF {
        EquationF {
            a,
            b,
            operator,
            sol: 0.0,
        }
    }
    pub fn get_sol(&mut self) -> f64 {
        if self.sol == 0.0 {
            self.sol = match self.operator {
                '+' => add_float(self.a, self.b),
                '-' => subtract_float(self.a, self.b),
                '*' => multiply_float(self.a, self.b),
                '/' => divide_float(self.a, self.b),
                '%' => modulo_float(self.a, self.b),
                '^' => power_float(self.a, self.b),
                _ => 0.0,
            };
        }
        self.sol
    }
    /// sets the a value (first number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_a(&mut self, a: f64) {
        self.a = a;
        self.sol = 0.0; // reset the solution
    }
    /// sets the b value (second number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_b(&mut self, b: f64) {
        self.b = b;
        self.sol = 0.0; // reset the solution
    }
    /// sets the operator in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_operator(&mut self, operator: char) {
        self.operator = operator;
        self.sol = 0.0; // reset the solution
    }
    pub fn get_a(&self) -> f64 {
        self.a
    }
    pub fn get_b(&self) -> f64 {
        self.b
    }
    pub fn get_operator(&self) -> char {
        self.operator
    }
}
#[cfg(test)]
mod test_equation_f {
    use super::*;

    #[test]
    fn it_works() {
        let mut equation = EquationF::new(1.0, 2.4, '+');
        assert_eq!(equation.get_sol(), 3.4);
    }
    #[test]
    fn set_a() {
        let mut equation = EquationF::new(1.0, 2.0, '+');
        equation.set_a(3.5);
        assert_eq!(equation.get_a(), 3.5);
        assert_eq!(equation.get_sol(), 5.5);
    }
    #[test]
    fn set_b() {
        let mut equation = EquationF::new(1.0, 2.0, '+');
        equation.set_b(3.883);
        assert_eq!(equation.get_b(), 3.883);
        assert_eq!(equation.get_sol(), 4.883);
    }
    #[test]
    fn set_operator() {
        let mut equation = EquationF::new(1.0, 2.0, '+');
        equation.set_operator('-');
        assert_eq!(equation.get_operator(), '-');
        assert_eq!(equation.get_sol(), -1.0);
        equation.set_operator('*');
        assert_eq!(equation.get_operator(), '*');
        assert_eq!(equation.get_sol(), 2.0);
        equation.set_operator('/');
        assert_eq!(equation.get_operator(), '/');
        assert_eq!(equation.get_sol(), 0.5);
        equation.set_operator('%');
        assert_eq!(equation.get_operator(), '%');
        assert_eq!(equation.get_sol(), 1.0);
        equation.set_operator('^');
        assert_eq!(equation.get_operator(), '^');
        assert_eq!(equation.get_sol(), 1.0);
    }
    #[test]
    fn change_equation(){
        let mut equation = EquationF::new(1.0, 2.0, '+');
        assert_eq!(equation.get_sol(), 3.0);
        equation.set_a(3.7);
        println!("a: {}", equation.get_a());
        println!("b: {}", equation.get_b());
        println!("sol: {}", equation.get_sol());
        assert_eq!(equation.get_sol(), 5.7);
        equation.set_b(3.2);
        println!("a: {}", equation.get_a());
        println!("b: {}", equation.get_b());
        assert_eq!(equation.get_sol(), 6.9);
        equation.set_a(14.042134);
        println!("a: {}", equation.get_a());
        println!("b: {}", equation.get_b());
        assert_eq!(equation.get_sol(), 17.242134);
        equation.set_operator('-');
        println!("a: {}", equation.get_a());
        println!("b: {}", equation.get_b());
        assert_eq!(equation.get_sol(), 10.842134000000001);
    }
}

/// ZeroEquation is an equation with a list of values on one side and a solution on the other.
/// Similar to Equation, but with a list of values (in a Vec) that will add up to the solution
/// (or be subtracted if the number is negative).  Sol is set to 0 and values can be moved to
/// the solution side of the equation with move_to_sol().
///
/// # Example
/// ```
/// use numbers_rus::ZeroEquation;
/// let mut equation = ZeroEquation::new(vec![1, 2, 3]);
/// assert_eq!(equation.get_sol(), 0);
/// ```
pub struct ZeroEquation {
    values: Vec<i128>,  // accepts lists of positive or negative numbers
    sol: i128,  // zero by default, but can be set to any number
    is_valid: bool,  // true if the values add up to the solution
}

impl ZeroEquation {
    pub fn new(values: Vec<i128>) -> ZeroEquation {
        ZeroEquation {
            values,
            sol: 0,
            is_valid: false,
        }
    }
    /// returns all the values in the equation
    pub fn get_values(&self) -> Vec<i128> {
        self.values.clone()
    }
    /// returns the value at the given index
    pub fn get_value(&self, index: usize) -> i128 {
        self.values[index]
    }
    /// adds a single value to the list of values
    pub fn add_value(&mut self, value: i128) {
        self.values.push(value);
        // chcek equation after altering to see if it is valid
        self.check_sol();
    }
    /// removes a single value from the list of values
    pub fn remove_value(&mut self, index: usize) {
        self.values.remove(index);
        // chcek equation after altering to see if it is valid
        self.check_sol();
    }
    /// returns the solution
    pub fn get_sol(&self) -> i128 {
        self.sol
    }
    // moves a value to the solution side of the equation
    pub fn move_to_sol(&mut self, index: usize) -> i128 {
        self.sol += self.values[index];
        self.values.remove(index);
        self.sol
    }
    /// checks if the values add up to the solution
    fn check_sol(&mut self) {
        let mut sum = 0;
        for value in &self.values {
            sum += value;
        }
        if sum == self.sol {
            self.is_valid = true;
        }
    }
    /// returns true if the values add up to the solution
    pub fn is_valid(&mut self) -> bool {
        self.check_sol();
        self.is_valid
    }
    /// returns the difference between the solution and the sum of the values if the values do not add up to the solution
    pub fn get_error(&mut self) -> i128 {
        self.check_sol();
        if self.is_valid {
            0
        } else {
            self.sol - self.values.iter().sum::<i128>()
        }
    }
}
#[cfg(test)]
mod test_zero_equation {
    use super::*;

    #[test]
    fn it_works() {
        let mut equation = ZeroEquation::new(vec![1, 2, 3]);
        assert_eq!(equation.get_sol(), 0);
        assert_eq!(equation.get_values(), vec![1, 2, 3]);
        assert_eq!(equation.get_value(0), 1);
        assert_eq!(equation.get_value(1), 2);
        assert_eq!(equation.get_value(2), 3);
        assert_eq!(equation.move_to_sol(2), 3);
        assert_eq!(equation.get_sol(), 3);
        assert_eq!(equation.get_values(), vec![1, 2]);
        assert_eq!(equation.get_value(0), 1);
        assert_eq!(equation.get_value(1), 2);
        assert_eq!(equation.is_valid(), true);
        assert_eq!(equation.get_error(), 0);
    }
    #[test]
    fn add_value() {
        let mut equation = ZeroEquation::new(vec![1, 2, 3]);
        equation.add_value(4);
        assert_eq!(equation.get_values(), vec![1, 2, 3, 4]);
    }
    #[test]
    fn remove_value() {
        let mut equation = ZeroEquation::new(vec![1, 2, 3]);
        equation.remove_value(1);
        assert_eq!(equation.get_values(), vec![1, 3]);
    }
    #[test]
    fn get_sol() {
        let equation = ZeroEquation::new(vec![1, 2, 3]);
        assert_eq!(equation.get_sol(), 0);
    }
    #[test]
    fn move_to_sol() {
        let mut equation = ZeroEquation::new(vec![1, 2, 3]);
        assert_eq!(equation.move_to_sol(2), 3);
    }
    #[test]
    fn is_valid() {
        let mut equation = ZeroEquation::new(vec![1, 2, 3]);
        assert_eq!(equation.is_valid(), false);
    }
    #[test]
    fn get_error() {
        let mut equation = ZeroEquation::new(vec![1, 2, 3]);
        assert_eq!(equation.get_error(), -6);
        equation.move_to_sol(2);
        // equation is now 1 + 2 = 3 (valid) so error should be 0
        assert_eq!(equation.get_error(), 0);
        println!("solution: {}", equation.get_sol());
        println!("values: {:?}", equation.get_values());

        equation.add_value(4);
        println!("solution: {}", equation.get_sol());
        println!("values: {:?}", equation.get_values());
        println!("error: {}", equation.is_valid());
    }
}



