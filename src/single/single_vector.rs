pub mod single_vector {
    // Single Vector Operations ------------------------------------------------------------------------

/// Returns the sum of all elements in a vector
pub fn vector_sum(vector: Vec<i128>) -> i128 {
        vector.iter().sum()
    }
/// Returns the sum of all elements in a float vector
pub fn vector_sum_float(vector: Vec<f64>) -> f64 {
        vector.iter().sum()
    }
/// Returns the product of all elements in a vector
pub fn vector_product(vector: Vec<i128>) -> i128 {
        vector.iter().fold(1, |acc, &x| acc * x)
    }
/// Returns the product of all elements ina  vector
pub fn vector_product_float(vector: Vec<f64>) -> f64 {
        vector.iter().fold(1.0, |acc, &x| acc * x)
    }
/// Returns the mean of all elements in a vector
pub fn vector_mean(vector: Vec<i128>) -> i128 {
        vector_sum(vector.clone()) / vector.len() as i128
    }
/// Returns the mean of all elements in a float vector
pub fn vector_mean_float(vector: Vec<f64>) -> f64 {
        vector_sum_float(vector.clone()) / vector.len() as f64
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
}
