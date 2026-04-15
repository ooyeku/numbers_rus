//! Descriptive statistics for numeric slices.
//!
//! Every function takes `&[T]` (no ownership required) and returns a numeric
//! result rather than a pre-formatted `String`. Functions that may be
//! undefined on an empty input return `Option<_>`.
//!
//! Aggregates that involve division (`mean`, `variance`, `std_dev`,
//! `quartiles`, etc.) return `f64`. For the sum or product over any
//! `Num + Copy` type, use [`sum`] and [`product`].
//!
//! # Examples
//!
//! ```
//! use numbers_rus::stats::*;
//!
//! let xs = [1i64, 2, 3, 4, 5];
//! assert_eq!(sum(&xs), 15);
//! assert_eq!(mean(&xs), Some(3.0));
//! assert_eq!(median(&xs), Some(3.0));
//! assert_eq!(range(&xs), Some(4.0));
//! ```

use core::hash::Hash;
use std::collections::HashMap;

use num_traits::{Num, ToPrimitive};

/// Sum of all elements. Returns `T::zero()` for an empty slice.
pub fn sum<T: Num + Copy>(xs: &[T]) -> T {
    xs.iter().copied().fold(T::zero(), |a, b| a + b)
}

/// Product of all elements. Returns `T::one()` for an empty slice.
pub fn product<T: Num + Copy>(xs: &[T]) -> T {
    xs.iter().copied().fold(T::one(), |a, b| a * b)
}

/// Arithmetic mean, as `f64`.
pub fn mean<T: Copy + ToPrimitive>(xs: &[T]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let s: f64 = xs.iter().map(|x| x.to_f64().unwrap_or(f64::NAN)).sum();
    Some(s / xs.len() as f64)
}

/// Population variance (divides by `N`, not `N - 1`).
pub fn variance<T: Copy + ToPrimitive>(xs: &[T]) -> Option<f64> {
    let m = mean(xs)?;
    let n = xs.len() as f64;
    let var = xs
        .iter()
        .map(|x| x.to_f64().unwrap_or(f64::NAN))
        .map(|x| (x - m).powi(2))
        .sum::<f64>()
        / n;
    Some(var)
}

/// Population standard deviation (divides by `N`).
pub fn std_dev<T: Copy + ToPrimitive>(xs: &[T]) -> Option<f64> {
    variance(xs).map(f64::sqrt)
}

/// Median. Returns the average of the two middle values for even-length input.
pub fn median<T: Copy + ToPrimitive>(xs: &[T]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let mut v: Vec<f64> = xs.iter().map(|x| x.to_f64().unwrap_or(f64::NAN)).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
    Some(median_sorted(&v))
}

fn median_sorted(v: &[f64]) -> f64 {
    let n = v.len();
    if n % 2 == 0 {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    } else {
        v[n / 2]
    }
}

/// Range `max − min`.
pub fn range<T: Copy + ToPrimitive>(xs: &[T]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let (min, max) = xs
        .iter()
        .map(|x| x.to_f64().unwrap_or(f64::NAN))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
            (lo.min(v), hi.max(v))
        });
    Some(max - min)
}

/// The three quartiles `(Q1, Q2, Q3)` computed with the "exclusive median"
/// rule (Tukey): split the sorted sequence at Q2, take medians of each half,
/// excluding Q2 itself for odd-length inputs.
pub fn quartiles<T: Copy + ToPrimitive>(xs: &[T]) -> Option<(f64, f64, f64)> {
    if xs.is_empty() {
        return None;
    }
    let mut v: Vec<f64> = xs.iter().map(|x| x.to_f64().unwrap_or(f64::NAN)).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
    let n = v.len();
    let q2 = median_sorted(&v);
    let (lower, upper) = if n % 2 == 0 {
        (&v[..n / 2], &v[n / 2..])
    } else {
        (&v[..n / 2], &v[n / 2 + 1..])
    };
    let q1 = if lower.is_empty() { q2 } else { median_sorted(lower) };
    let q3 = if upper.is_empty() { q2 } else { median_sorted(upper) };
    Some((q1, q2, q3))
}

/// Interquartile range `Q3 − Q1`.
pub fn iqr<T: Copy + ToPrimitive>(xs: &[T]) -> Option<f64> {
    let (q1, _, q3) = quartiles(xs)?;
    Some(q3 - q1)
}

/// Most-frequent element. Ties are broken by which value is seen first.
///
/// Requires hashability, so this does not apply to `f32` / `f64` directly —
/// round or bucket floats before calling.
pub fn mode<T: Eq + Hash + Copy>(xs: &[T]) -> Option<T> {
    if xs.is_empty() {
        return None;
    }
    let mut counts: HashMap<T, (usize, usize)> = HashMap::new();
    for (i, &x) in xs.iter().enumerate() {
        let entry = counts.entry(x).or_insert((0, i));
        entry.0 += 1;
    }
    counts
        .into_iter()
        .max_by(|(_, (c1, i1)), (_, (c2, i2))| c1.cmp(c2).then(i2.cmp(i1)))
        .map(|(k, _)| k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_and_product() {
        let xs = [1i64, 2, 3, 4];
        assert_eq!(sum(&xs), 10);
        assert_eq!(product(&xs), 24);
        let empty: [i64; 0] = [];
        assert_eq!(sum(&empty), 0);
        assert_eq!(product(&empty), 1);
    }

    #[test]
    fn central_tendency() {
        let xs = [1i64, 2, 3, 4, 5];
        assert_eq!(mean(&xs), Some(3.0));
        assert_eq!(median(&xs), Some(3.0));
        let even = [1.0f64, 2.0, 3.0, 4.0];
        assert_eq!(mean(&even), Some(2.5));
        assert_eq!(median(&even), Some(2.5));
    }

    #[test]
    fn dispersion() {
        let xs = [2.0f64, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(variance(&xs), Some(4.0));
        assert_eq!(std_dev(&xs), Some(2.0));
        assert_eq!(range(&xs), Some(7.0));
    }

    #[test]
    fn quartiles_and_iqr() {
        let xs = [1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let (q1, q2, q3) = quartiles(&xs).unwrap();
        assert_eq!(q1, 2.5);
        assert_eq!(q2, 4.5);
        assert_eq!(q3, 6.5);
        assert_eq!(iqr(&xs), Some(4.0));
    }

    #[test]
    fn mode_picks_most_frequent() {
        assert_eq!(mode(&[1i64, 2, 2, 3, 3, 3, 4]), Some(3));
        assert_eq!(mode::<i64>(&[]), None);
    }

    #[test]
    fn empty_inputs_return_none() {
        let empty: [f64; 0] = [];
        assert_eq!(mean(&empty), None);
        assert_eq!(median(&empty), None);
        assert_eq!(range(&empty), None);
        assert_eq!(quartiles(&empty), None);
    }

    #[test]
    fn single_element_slice() {
        let one = [42.0f64];
        assert_eq!(mean(&one), Some(42.0));
        assert_eq!(median(&one), Some(42.0));
        assert_eq!(variance(&one), Some(0.0));
        assert_eq!(std_dev(&one), Some(0.0));
        assert_eq!(range(&one), Some(0.0));
        let (q1, q2, q3) = quartiles(&one).unwrap();
        assert_eq!((q1, q2, q3), (42.0, 42.0, 42.0));
        assert_eq!(iqr(&one), Some(0.0));
    }

    #[test]
    fn all_equal_slice_has_zero_dispersion() {
        let xs = [7.0f64; 10];
        assert_eq!(mean(&xs), Some(7.0));
        assert_eq!(variance(&xs), Some(0.0));
        assert_eq!(std_dev(&xs), Some(0.0));
        assert_eq!(range(&xs), Some(0.0));
        assert_eq!(iqr(&xs), Some(0.0));
    }

    #[test]
    fn negative_inputs() {
        let xs = [-3.0f64, -1.0, 0.0, 1.0, 3.0];
        assert_eq!(mean(&xs), Some(0.0));
        assert_eq!(median(&xs), Some(0.0));
        assert_eq!(range(&xs), Some(6.0));
    }

    #[test]
    fn mode_tie_breaking_picks_earliest() {
        // Both 1 and 2 appear twice; the earliest first occurrence wins.
        assert_eq!(mode(&[1i64, 2, 1, 2, 3]), Some(1));
        assert_eq!(mode(&[2i64, 1, 1, 2, 3]), Some(2));
    }

    #[test]
    fn variance_matches_definition() {
        // var(x) = E[x²] - E[x]²
        let xs = [1.0f64, 2.0, 3.0, 4.0, 5.0];
        let m = mean(&xs).unwrap();
        let manual = xs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / xs.len() as f64;
        assert!((variance(&xs).unwrap() - manual).abs() < 1e-12);
    }

    #[test]
    fn integer_inputs_promote_to_f64() {
        let xs = [1u32, 2, 3, 4, 5];
        assert_eq!(mean(&xs), Some(3.0));
        let xs = [10i128, 20, 30];
        assert_eq!(mean(&xs), Some(20.0));
    }
}
