
//! single_vector module provides a set of functions for performing statistical calculations on vectors of 128-bit signed integers and 64-bit floating-point numbers.
//!
//! The module includes functions for calculating the following statistics:
//! - Sum
//! - Product
//! - Mean
//! - Median
//! - Mode
//! - Range
//! - Interquartile Range
//! - Variance
//! - Standard Deviation
//! - Quartiles (Q1, Q2, and Q3)
//!
pub mod single_vector {

/// Returns the sum of all elements in a vector of 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * i128 - The sum of all elements in the input vector
pub fn vector_sum(vector: Vec<i128>) -> i128 {
        vector.iter().sum()
    }
/// Computes the sum of all elements in a given vector of floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The sum of all elements in the input vector
pub fn vector_sum_float(vector: Vec<f64>) -> f64 {
        vector.iter().sum()
    }

/// Returns the product of all elements in a vector of 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * i128 - The product of all elements in the input vector
pub fn vector_product(vector: Vec<i128>) -> i128 {
        vector.iter().fold(1, |acc, &x| acc * x)
    }
/// Computes the product of all elements in a given vector of floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The product of all elements in the input vector
pub fn vector_product_float(vector: Vec<f64>) -> f64 {
        vector.iter().fold(1.0, |acc, &x| acc * x)
    }

/// Calculates the mean of a given vector of 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * i128 - The mean of all elements in the input vector
pub fn vector_mean(vector: Vec<i128>) -> i128 {
        vector_sum(vector.clone()) / vector.len() as i128
    }

/// Calculates the mean of a given vector of 64-bit floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The mean of all elements in the input vector
pub fn vector_mean_float(vector: Vec<f64>) -> f64 {
        vector_sum_float(vector.clone()) / vector.len() as f64
    }
/// Calculates the median value of a vector containing 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * i128 - The median value of the input vector
pub fn vector_median(vector: Vec<i128>) -> i128 {
        let mut sorted_vector = vector.clone();
        sorted_vector.sort();

        if sorted_vector.len() % 2 == 0 {
            (sorted_vector[sorted_vector.len() / 2] + sorted_vector[sorted_vector.len() / 2 - 1]) / 2
        } else {
            sorted_vector[sorted_vector.len() / 2]
        }
    }

/// Calculates the median value of a given vector of 64-bit floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The median value of the input vector
pub fn vector_median_float(vector: Vec<f64>) -> f64 {
        let mut sorted_vector = vector.clone();
        sorted_vector.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sorted_vector.len() % 2 == 0 {
            (sorted_vector[sorted_vector.len() / 2] + sorted_vector[sorted_vector.len() / 2 - 1]) / 2.0
        } else {
            sorted_vector[sorted_vector.len() / 2]
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

/// Calculates the range of a given vector of 128-bit signed integers, which is the difference between the maximum and
/// minimum values in the vector.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * i128 - The range of the input vector, calculated as the difference between the maximum and minimum values
pub fn vector_range(vector: Vec<i128>) -> i128 {
        let mut sorted = vector.clone();
        sorted.sort();
        sorted[sorted.len() - 1] - sorted[0]
    }

/// Calculates the range of a given vector of 64-bit floating-point numbers, which is the difference between the maximum and
/// minimum values in the vector.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The range of the input vector, calculated as the difference between the maximum and minimum values
pub fn vector_range_float(vector: Vec<f64>) -> f64 {
        let mut sorted = vector.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[sorted.len() - 1] - sorted[0]
    }

/// Computes the interquartile range (IQR) of a given vector of 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * i128 - The interquartile range of the input vector, calculated as the difference between the third (Q3) and first (Q1) quartiles
pub fn vector_interquartile_range(vector: Vec<i128>) -> i128 {
        let mut sorted = vector.clone();
        sorted.sort();
        let q1 = vector_median(sorted[0..sorted.len() / 2].to_vec());
        let q3 = vector_median(sorted[sorted.len() / 2..sorted.len()].to_vec());
        q3 - q1
    }
/// Computes the interquartile range (IQR) of a given vector of 64-bit floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The interquartile range of the input vector, calculated as the difference between the third (Q3) and first (Q1) quartiles
pub fn vector_interquartile_range_float(vector: Vec<f64>) -> f64 {
        let mut sorted = vector.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let q1 = vector_median_float(sorted[0..sorted.len() / 2].to_vec());
        let q3 = vector_median_float(sorted[sorted.len() / 2..sorted.len()].to_vec());
        q3 - q1
    }

/// Calculates the variance of a given vector of 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * String - The variance of the input vector, formatted as a string with two decimal places
pub fn vector_variance(vector: Vec<i128>) -> String {
        let mean = vector_mean(vector.clone());
        let mut sum = 0;

        for i in 0..vector.len() {
            sum += (vector[i] - mean).pow(2);
        }

    format!("{:.2}", sum as f64 / vector.len() as f64)
    }

/// Calculates the variance of a given vector of 64-bit floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The variance of the input vector
pub fn vector_variance_float(vector: Vec<f64>) -> f64 {
        let mean = vector_mean_float(vector.clone());
        let mut sum = 0.0;

        for i in 0..vector.len() {
            sum += (vector[i] - mean).powf(2.0);
        }

    sum / vector.len() as f64
    }

/// Calculates the standard deviation of a given vector of 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * String - The standard deviation of the input vector, formatted as a string with two decimal places
pub fn vector_standard_deviation(vector: Vec<i128>) -> String {
        let variance = vector_variance(vector.clone());
        format!("{:.2}", variance.parse::<f64>().unwrap().sqrt())
    }

/// Calculates the standard deviation of a given vector of 64-bit floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * f64 - The standard deviation of the input vector
pub fn vector_standard_deviation_float(vector: Vec<f64>) -> f64 {
        let variance = vector_variance_float(vector.clone());
        variance.sqrt()
    }

/// Computes the first (Q1), second (Q2), and third (Q3) quartiles of a given vector of 128-bit signed integers.
///
/// # Arguments
///
/// * `vector` - A vector of signed 128-bit integers
///
/// # Returns
///
/// * String - A formatted string displaying the first, second, and third quartiles as "Q1: {}, Q2: {}, Q3: {}"
pub fn vector_quartiles(vector: Vec<i128>) -> String {
        let mut sorted = vector.clone();
        sorted.sort();

        let q1 = sorted[sorted.len() / 4];
        let q2 = sorted[sorted.len() / 2];
        let q3 = sorted[sorted.len() * 3 / 4];

        format!("Q1: {}, Q2: {}, Q3: {}", q1, q2, q3)
    }

/// Returns a formatted string containing the first (Q1), second (Q2), and third (Q3) quartiles of a given vector of 64-bit
/// floating-point numbers.
///
/// # Arguments
///
/// * `vector` - A vector of 64-bit floating-point numbers
///
/// # Returns
///
/// * String - A formatted string containing the calculated Q1, Q2, and Q3 quartiles, in the format "Q1: {Q1}, Q2: {Q2}, Q3: {Q3}"
pub fn vector_quartiles_float(vector: Vec<f64>) -> String {
        let mut sorted = vector.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let q1 = sorted[sorted.len() / 4];
        let q2 = sorted[sorted.len() / 2];
        let q3 = sorted[sorted.len() * 3 / 4];

        format!("Q1: {}, Q2: {}, Q3: {}", q1, q2, q3)
    }
}
