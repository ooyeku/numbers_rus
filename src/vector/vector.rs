use itertools::join;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

/// A helper function to execute and collect the results of a binary operation on two vectors.
fn execute_op<T, F>(left: Vec<T>, right: Vec<T>, op: F) -> Vec<T>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    left.into_iter().zip(right).map(|(a, b)| op(a, b)).collect()
}

/// A helper function to join a vector of elements into a string.
fn join_elements<T>(elements: Vec<T>) -> String
where
    T: Display,
{
    join(&elements, ", ")
}

/// Performs addition on two vectors and returns the result as a string.
pub fn vector_add<T>(left: Vec<T>, right: Vec<T>) -> String
where
    T: Copy + Add<Output = T> + Display,
{
    let result = execute_op(left, right, |a, b| a + b);
    join_elements(result)
}

/// Performs subtraction on two vectors and returns the result as a string.
pub fn vector_subtract<T>(left: Vec<T>, right: Vec<T>) -> String
where
    T: Copy + Sub<Output = T> + Display,
{
    let result = execute_op(left, right, |a, b| a - b);
    join_elements(result)
}

/// Performs multiplication on two vectors and returns the result as a string.
pub fn vector_multiply<T>(left: Vec<T>, right: Vec<T>) -> String
where
    T: Copy + Mul<Output = T> + Display,
{
    let result = execute_op(left, right, |a, b| a * b);
    join_elements(result)
}

/// Performs division on two vectors and returns the result as a string.
pub fn vector_divide<T>(left: Vec<T>, right: Vec<T>) -> String
where
    T: Copy + Div<Output = T> + Display,
{
    let result = execute_op(left, right, |a, b| a / b);
    join_elements(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_add() {
        let left = vec![1, 2, 3];
        let right = vec![4, 5, 6];
        let result = vector_add(left, right);
        assert_eq!(result, "5, 7, 9");
    }

    #[test]
    fn test_vector_subtract() {
        let left = vec![1, 2, 3];
        let right = vec![4, 5, 6];
        let result = vector_subtract(left, right);
        assert_eq!(result, "-3, -3, -3");
    }

    #[test]
    fn test_vector_multiply() {
        let left = vec![1, 2, 3];
        let right = vec![4, 5, 6];
        let result = vector_multiply(left, right);
        assert_eq!(result, "4, 10, 18");
    }

    #[test]
    fn test_vector_divide() {
        let left = vec![1, 2, 3];
        let right = vec![4, 5, 6];
        let result = vector_divide(left, right);
        assert_eq!(result, "0, 0, 0");
    }
}
