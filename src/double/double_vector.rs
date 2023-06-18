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
    use std::error::Error;
    use std::collections::HashMap;
    // Helper function to find the integer mode of a combined vector
fn find_int_mode(merged: Vec<i128>) -> Result<i128, Box<dyn Error>> {
        let mut frequency_map = HashMap::new();
        for number in merged {
            *frequency_map.entry(number).or_insert(0) += 1;
        }

    frequency_map
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .filter(|(_, count)| *count > 1)
        .map(|(number, _)| number)
        .ok_or_else(|| "There is no distinct mode.".into())
    }

/// Returns the mode of two combined integer vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of i128 integers
/// * `right` - A vector of i128 integers
pub fn two_vector_mode(left: Vec<i128>, right: Vec<i128>) -> Result<String, Box<dyn Error>> {
        let combined = [left, right].concat();
        Ok(find_int_mode(combined)?.to_string())
    }

/// Returns the mode of two combined float vectors as a string
///
/// # Arguments
///
/// * `left` - A vector of f64 floating-point numbers
/// * `right` - A vector of f64 floating-point numbers
// Helper function to find the float mode of a combined vector
fn find_float_mode(merged: Vec<f64>) -> Result<f64, Box<dyn Error>> {
        let mut frequency_map = HashMap::new();
        for number in merged {
            *frequency_map.entry(number.to_bits()).or_insert(0) += 1;
        }

    frequency_map
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .filter(|(_, count)| *count > 1)
        .map(|(number_bits, _)| f64::from_bits(number_bits))
        .ok_or_else(|| "There is no distinct mode.".into())
    }

pub fn two_vector_mode_float(left: Vec<f64>, right: Vec<f64>) -> Result<String, Box<dyn Error>> {
        let combined = [left, right].concat();
        Ok(find_float_mode(combined)?.to_string())
    }




