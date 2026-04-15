//! Classical integer sequences, implemented iteratively with overflow checks.
//!
//! Every function here returns `Option<u128>`, yielding `None` on overflow
//! rather than panicking or producing a silently truncated result.

/// Factorial `n!`.
///
/// `factorial(0) == Some(1)`. Overflows at `n == 35` (since `34! < 2^128 <
/// 35!`).
pub fn factorial(n: u64) -> Option<u128> {
    let mut result: u128 = 1;
    for k in 2..=(n as u128) {
        result = result.checked_mul(k)?;
    }
    Some(result)
}

/// The `n`-th Fibonacci number, with `fibonacci(0) == 0` and `fibonacci(1) == 1`.
///
/// Overflows a little past `n == 186`.
pub fn fibonacci(n: u64) -> Option<u128> {
    if n == 0 {
        return Some(0);
    }
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 1..n {
        let next = a.checked_add(b)?;
        a = b;
        b = next;
    }
    Some(b)
}

/// The `n`-th triangular number `T(n) = n(n+1)/2`.
pub fn triangular(n: u64) -> Option<u128> {
    let nn = n as u128;
    nn.checked_mul(nn + 1).map(|v| v / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorials() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(1), Some(1));
        assert_eq!(factorial(5), Some(120));
        assert_eq!(factorial(20), Some(2_432_902_008_176_640_000));
        // Known to fit: 34!. Known to overflow u128: 35!.
        assert!(factorial(34).is_some());
        assert_eq!(factorial(35), None);
    }

    #[test]
    fn fibonaccis() {
        let expected = [0u128, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        for (i, &v) in expected.iter().enumerate() {
            assert_eq!(fibonacci(i as u64), Some(v));
        }
        assert!(fibonacci(186).is_some());
        assert_eq!(fibonacci(200), None);
    }

    #[test]
    fn triangulars() {
        assert_eq!(triangular(0), Some(0));
        assert_eq!(triangular(1), Some(1));
        assert_eq!(triangular(10), Some(55));
        assert_eq!(triangular(100), Some(5050));
    }
}
