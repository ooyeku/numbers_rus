//! Integer predicates and divisor-based functions.
//!
//! The `perfect_power` detector here supersedes the `is_perfect_power` from
//! pre-1.0 releases, which was a bug-for-bug copy of `is_perfect_square`.

/// True iff `n` is even.
pub fn is_even(n: i64) -> bool {
    n & 1 == 0
}

/// True iff `n` is odd.
pub fn is_odd(n: i64) -> bool {
    n & 1 != 0
}

/// True iff `n == k² for some non-negative integer `k`.
pub fn is_perfect_square(n: u64) -> bool {
    let approx = (n as f64).sqrt() as u64;
    (approx.saturating_sub(1)..=approx.saturating_add(1))
        .any(|k| k.checked_mul(k) == Some(n))
}

/// True iff `n == k³ for some integer `k`.
pub fn is_perfect_cube(n: i64) -> bool {
    let abs = n.unsigned_abs();
    let approx = (abs as f64).cbrt().round() as u64;
    (approx.saturating_sub(1)..=approx.saturating_add(1))
        .any(|k| k.checked_pow(3) == Some(abs))
}

/// Detect whether `n` is a non-trivial perfect power `b^e` with `b ≥ 2, e ≥ 2`.
///
/// Returns `Some((b, e))` for the **smallest** valid base (equivalently the
/// **largest** exponent) when such a representation exists, else `None`.
/// `n < 4` always returns `None`.
///
/// ```
/// use numbers_rus::integers::properties::perfect_power;
///
/// assert_eq!(perfect_power(8), Some((2, 3)));
/// assert_eq!(perfect_power(81), Some((3, 4)));
/// assert_eq!(perfect_power(1024), Some((2, 10)));
/// assert_eq!(perfect_power(12), None);
/// ```
pub fn perfect_power(n: u64) -> Option<(u64, u32)> {
    if n < 4 {
        return None;
    }
    let max_exp = 63 - n.leading_zeros();
    for exp in (2..=max_exp).rev() {
        let approx = (n as f64).powf(1.0 / exp as f64).round() as u64;
        for candidate in approx.saturating_sub(1)..=approx.saturating_add(1) {
            if candidate < 2 {
                continue;
            }
            if let Some(p) = candidate.checked_pow(exp) {
                if p == n {
                    return Some((candidate, exp));
                }
            }
        }
    }
    None
}

/// All positive divisors of `n`, ascending. Empty for `n == 0`.
pub fn divisors(n: u64) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i: u64 = 1;
    while i.saturating_mul(i) <= n {
        if n % i == 0 {
            out.push(i);
            let paired = n / i;
            if paired != i {
                out.push(paired);
            }
        }
        i += 1;
    }
    out.sort_unstable();
    out
}

/// τ(n): the number of positive divisors of `n`.
pub fn divisor_count(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    super::primes::prime_factorize(n)
        .iter()
        .map(|&(_, e)| e as u64 + 1)
        .product::<u64>()
        .max(1)
}

/// σ(n): the sum of positive divisors of `n`.
pub fn divisor_sum(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    super::primes::prime_factorize(n)
        .into_iter()
        .map(|(p, e)| (p.pow(e + 1) - 1) / (p - 1))
        .product()
}

/// True iff σ(n) − n == n; i.e., the proper divisors of `n` sum to `n`.
pub fn is_perfect_number(n: u64) -> bool {
    n > 0 && divisor_sum(n) == 2 * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity() {
        assert!(is_even(0) && is_even(-2) && is_even(42));
        assert!(is_odd(-1) && is_odd(3) && is_odd(i64::MAX));
    }

    #[test]
    fn squares_and_cubes() {
        assert!(is_perfect_square(0));
        assert!(is_perfect_square(1));
        assert!(is_perfect_square(144));
        assert!(!is_perfect_square(145));
        assert!(is_perfect_cube(27));
        assert!(is_perfect_cube(-27));
        assert!(!is_perfect_cube(28));
    }

    #[test]
    fn perfect_power_cases() {
        // Prior-era bug: this used to silently return false.
        assert_eq!(perfect_power(8), Some((2, 3)));
        assert_eq!(perfect_power(27), Some((3, 3)));
        assert_eq!(perfect_power(81), Some((3, 4)));
        assert_eq!(perfect_power(1024), Some((2, 10)));
        assert_eq!(perfect_power(2_u64.pow(30)), Some((2, 30)));
        // Smallest base should win when multiple representations exist:
        // 64 = 2^6 = 4^3 = 8^2 → (2, 6).
        assert_eq!(perfect_power(64), Some((2, 6)));

        assert_eq!(perfect_power(0), None);
        assert_eq!(perfect_power(3), None);
        assert_eq!(perfect_power(12), None);
        assert_eq!(perfect_power(1_000_003), None);
    }

    #[test]
    fn divisor_functions() {
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisor_count(12), 6);
        assert_eq!(divisor_sum(12), 28);
        assert!(is_perfect_number(6));
        assert!(is_perfect_number(28));
        assert!(is_perfect_number(496));
        assert!(!is_perfect_number(27));
    }
}
