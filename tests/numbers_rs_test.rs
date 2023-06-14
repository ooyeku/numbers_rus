use numbers_rus::*;
use crate::integers::base::*;
use crate::floats::base_float::*;
use crate::single::single_vector::*;
use crate::double::double_vector::*;

#[cfg(test)]
mod test_add {
    use crate::integers::base::*;
    use crate::floats::base_float::*;

    #[test]
    fn it_works() {
        let result = base::add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn add_negatives() {
        let result = base::add(-2,2);
        assert_eq!(result, 0);
    }
    #[test]
    fn add_floats() {
        let result = base_float::add_float(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
#[cfg(test)]
mod test_subtract {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::subtract(2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn subtract_negatives() {
        let result = base::subtract(-2, 2);
        assert_eq!(result, -4);
    }
    #[test]
    fn subtract_float() {
        let result = base_float::subtract_float(4.4, 2.2);
        assert_eq!(result, 2.2);
    }
}
#[cfg(test)]
mod test_multiply {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::multiply(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn multiply_negatives() {
        let result = base::multiply(-2, 2);
        assert_eq!(result, -4);
    }
    #[test]
    fn multiply_floats() {
        let result = base_float::multiply_float(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
#[cfg(test)]
mod test_divide {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::divide(2, 2);
        assert_eq!(result, 1);
    }
    #[test]
    fn divide_float() {
        let result = base_float::divide_float(8.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
#[cfg(test)]
mod test_modulo {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::modulo(2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn modulo_negatives() {
        let result = base::modulo(-2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn modulo_floats() {
        let result = base_float::modulo_float(2.0, 2.0);
        assert_eq!(result, 0.0);
    }
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
        let result = base::power(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn power_negatives() {
        let result = base::power(-2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn power_floats() {
        let result = base_float::power_float(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
#[cfg(test)]
mod test_root {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::root(4, 2);
        assert_eq!(result, 2);
    }
    #[test]
    fn root_floats() {
        let result = base_float::root_float(4.0, 2.0);
        assert_eq!(result, 2.0);
    }
}
#[cfg(test)]
mod test_factorial {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::factorial(4);
        assert_eq!(result, 24);
    }
    #[test]
    fn factorial_floats() {
        let result = base_float::factorial_float(4.0);
        assert_eq!(result, 24.0);
    }
}
#[cfg(test)]
mod test_fibonacci {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::fibonacci(4);
        assert_eq!(result, 3);
    }
    #[test]
    fn fibonacci_floats() {
        let result = base_float::fibonacci_float(4.0);
        assert_eq!(result, 3.0);
    }
}
#[cfg(test)]
mod test_is_prime {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::is_prime(4);
        assert_eq!(result, false);
    }
    #[test]
    fn is_prime_floats() {
        let result = base_float::is_prime_float(4.0);
        assert_eq!(result, false);
    }
}
#[cfg(test)]
mod test_is_even {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::is_even(4);
        assert_eq!(result, true);
    }
    #[test]
    fn is_even_floats() {
        let result = base_float::is_even_float(4.0);
        assert_eq!(result, true);
    }
}
#[cfg(test)]
mod test_is_odd {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::is_odd(4);
        assert_eq!(result, false);
    }
    #[test]
    fn is_odd_floats() {
        let result = base_float::is_odd_float(4.0);
        assert_eq!(result, false);
    }
}
#[cfg(test)]
mod test_is_perfect_square {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::is_perfect_square(4);
        assert_eq!(result, true);
    }
    #[test]
    fn is_perfect_square_floats() {
        let result = base_float::is_perfect_square_float(4.0);
        assert_eq!(result, true);
    }
}
#[cfg(test)]
mod test_is_perfect_cube {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::is_perfect_cube(4);
        assert_eq!(result, false);
    }
    #[test]
    fn is_perfect_cube_floats() {
        let result = base_float::is_perfect_cube_float(4.0);
        assert_eq!(result, false);
    }
}
#[cfg(test)]
mod test_is_perfect_power {
    use super::*;

    #[test]
    fn it_works() {
        let result = base::is_perfect_power(4);
        assert_eq!(result, true);
    }
    #[test]
    fn is_perfect_power_floats() {
        let result = base_float::is_perfect_power_float(4.0);
        assert_eq!(result, true);
    }
}
#[cfg(test)]
mod test_vector_sum {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_sum(vec![1, 2, 3]);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_sum_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 6.0);
    }
}
#[cfg(test)]
mod test_vector_product {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_product(vec![1, 2, 3]);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_product_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 6.0);
    }
}
#[cfg(test)]
mod test_vector_mean {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_mean(vec![1, 2, 3]);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_mean_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 2.0);
    }
}
#[cfg(test)]
mod test_vector_median {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_median(vec![1, 2, 3]);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_median_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 2.0);
    }
}
#[cfg(test)]
mod test_vector_mode {
    use super::*;


    #[test]
     fn it_works_floats() {
        let result = single_vector::vector_mode_float(vec![1.0, 2.0, 3.0, 1.0]);
        assert_eq!(result, 1.0);
    }
}
#[cfg(test)]
mod test_vector_range {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_range(vec![1, 2, 3]);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_range_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 2.0);
    }
}
#[cfg(test)]
mod test_vector_interquartile_range {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_interquartile_range(vec![1, 2, 3]);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_interquartile_range_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 1.5);
    }
}
#[cfg(test)]
mod test_vector_variance {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_variance(vec![1, 2, 3]);
        assert_eq!(result, "0.67");
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_variance_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 0.6666666666666666);
    }
}
#[cfg(test)]
mod test_vector_standard_deviation {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_standard_deviation(vec![1, 2, 3]);
        assert_eq!(result, "0.82");
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_standard_deviation_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, 0.816496580927726);
    }
}
#[cfg(test)]
mod test_vector_quartiles {
    use super::*;

    #[test]
    fn it_works() {
        let result = single_vector::vector_quartiles(vec![1, 2, 3]);
        assert_eq!(result, ("Q1: 1, Q2: 2, Q3: 3"));
    }
    #[test]
    fn it_works_floats() {
        let result = single_vector::vector_quartiles_float(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, ("Q1: 1, Q2: 2, Q3: 3"));
    }
}
#[cfg(test)]
mod test_vector_add {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_add(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "2, 4, 6");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_add_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "2, 4, 6");
    }
}
#[cfg(test)]
mod test_vector_subtract {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_subtract(vec![1, 2, 3], vec![1, 2, 3]);

        assert_eq!(result, "0, 0, 0");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_subtract_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);

        assert_eq!(result, "0, 0, 0");
    }
}
#[cfg(test)]
mod test_vector_multiply {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_multiply(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 4, 9");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_multiply_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 4, 9");
    }
}
#[cfg(test)]
mod test_vector_divide {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_divide(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 1, 1");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_divide_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 1, 1");
    }
}
#[cfg(test)]
mod test_vector_modulo {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_modulo(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "0, 0, 0");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_modulo_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "0, 0, 0");
    }
}
#[cfg(test)]
mod test_vector_power {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_power(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 4, 27");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_power_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 4, 27");
    }
}
#[cfg(test)]
mod test_vector_root {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_root(vec![1, 4, 27], vec![1, 2, 3]);
        assert_eq!(result, "1, 4, 1");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_root_float(vec![1.0, 4.0, 27.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}
#[cfg(test)]
mod test_vector_min {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_min(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_min_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}
#[cfg(test)]
mod test_vector_max {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_max(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_max_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}
#[cfg(test)]
mod test_vector_average {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::vector_average(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::vector_average_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}
#[cfg(test)]
mod test_two_vector_median {
    use super::*;

    #[test]
    fn it_works() {
        let result = double_vector::two_vector_median(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, "1, 2, 3");
    }
    #[test]
    fn it_works_floats() {
        let result = double_vector::two_vector_median_float(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]);
        assert_eq!(result, "1, 2, 3");
    }
}
#[cfg(test)]
mod vector_mode_tests {
    use super::*;

    #[test]
    fn test_two_vector_mode() {
        let left = vec![1, 2, 1, 2];
        let right = vec![2, 3, 2, 1];
        let result = double_vector::two_vector_mode(left, right).unwrap();
        assert_eq!(result, "2".to_string());
    }

    #[test]
    fn test_two_vector_mode_float() {
        let left = vec![1.0, 2.0, 3.0, 4.0];
        let right = vec![2.0, 3.0, 4.0, 4.0];
        let result = double_vector::two_vector_mode_float(left, right).unwrap();
        assert_eq!(result, "4".to_string());
    }

    #[test]
    fn test_two_vector_mode_no_repeats() {
        let left = vec![1, 2, 3];
        let right = vec![4, 5, 6];
        let result = double_vector::two_vector_mode(left, right);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "There is no distinct mode.");
    }

    #[test]
    fn test_two_vector_mode_float_no_repeats() {
        let left = vec![1.0, 2.0, 3.0];
        let right = vec![4.0, 5.0, 6.0];
        let result = double_vector::two_vector_mode_float(left, right);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "There is no distinct mode.");
    }
}
