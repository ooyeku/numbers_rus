//! Prime detection, enumeration, and factorization.
//!
//! The primality test is a **deterministic** Miller-Rabin check for the full
//! `u64` range: the witness set `{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}`
//! is proven sufficient below 3.3·10¹⁴ and covers every value representable
//! in `u64` up to `2⁶⁴ − 1`.
//!
//! # Examples
//!
//! ```
//! use numbers_rus::integers::primes::{is_prime, sieve, next_prime, prime_factorize};
//!
//! assert!(is_prime(1_000_003));
//! assert!(!is_prime(1_000_004));
//!
//! assert_eq!(sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
//! assert_eq!(next_prime(100), 101);
//! assert_eq!(prime_factorize(360), vec![(2, 3), (3, 2), (5, 1)]);
//! ```

use super::arith::mod_pow;

const SMALL_PRIMES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

/// Deterministic primality test for any `u64`.
///
/// Runs in `O(k · log³ n)` time where `k` is the number of Miller-Rabin
/// witnesses (12). In practice this is microseconds even for values near
/// `u64::MAX`.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &SMALL_PRIMES {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }

    // Decompose n - 1 = d · 2^r with d odd.
    let mut d = n - 1;
    let mut r: u32 = 0;
    while d & 1 == 0 {
        d >>= 1;
        r += 1;
    }

    'witness: for &a in &SMALL_PRIMES {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r.saturating_sub(1) {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

#[inline]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// All primes ≤ `limit`, returned in ascending order.
///
/// Implementation is a plain Sieve of Eratosthenes — `O(n log log n)` time
/// and `O(n)` space. For very large bounds, prefer a segmented sieve (not
/// yet in this crate).
pub fn sieve(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    let size = (limit as usize) + 1;
    let mut sieve = vec![true; size];
    sieve[0] = false;
    sieve[1] = false;
    let mut i: u64 = 2;
    while i.saturating_mul(i) <= limit {
        if sieve[i as usize] {
            let mut j = i * i;
            while j <= limit {
                sieve[j as usize] = false;
                j += i;
            }
        }
        i += 1;
    }
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(idx, keep)| keep.then_some(idx as u64))
        .collect()
}

/// Smallest prime strictly greater than `n`.
///
/// Panics only on `u64` overflow (i.e., when there is no prime representable
/// in `u64` above `n`, which happens for `n > 18446744073709551557`).
pub fn next_prime(n: u64) -> u64 {
    if n < 2 {
        return 2;
    }
    let mut candidate = n + 1;
    loop {
        if is_prime(candidate) {
            return candidate;
        }
        candidate = candidate
            .checked_add(1)
            .expect("next_prime: no prime fits in u64 above this value");
    }
}

/// The `n`-th prime, 1-indexed (`nth_prime(1) == 2`, `nth_prime(6) == 13`).
///
/// Uses a Dusart-style upper bound `n·(ln n + ln ln n)` to size the sieve,
/// so cost scales as the sieve on `O(n log n)`.
///
/// # Panics
/// If `n == 0`.
pub fn nth_prime(n: usize) -> u64 {
    assert!(n >= 1, "nth_prime: n must be >= 1");
    // Hard-coded for very small n where the asymptotic bound underestimates.
    const SMALL: [u64; 6] = [2, 3, 5, 7, 11, 13];
    if n <= SMALL.len() {
        return SMALL[n - 1];
    }
    let nf = n as f64;
    let upper = (nf * (nf.ln() + nf.ln().ln()) * 1.25).ceil() as u64;
    let primes = sieve(upper);
    primes[n - 1]
}

/// Prime factorization of `n` as `(prime, exponent)` pairs.
///
/// Uses a 2,3 bootstrap followed by a 6k±1 wheel. Returns an empty vector
/// for `n < 2`.
pub fn prime_factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    if n < 2 {
        return out;
    }
    for &p in &[2u64, 3] {
        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }
        if count > 0 {
            out.push((p, count));
        }
    }
    let mut i: u64 = 5;
    while i.saturating_mul(i) <= n {
        for &p in &[i, i + 2] {
            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                out.push((p, count));
            }
        }
        i += 6;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primality_small() {
        for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29] {
            assert!(is_prime(p), "{} should be prime", p);
        }
        for &c in &[0u64, 1, 4, 6, 8, 9, 10, 12, 14, 15, 100] {
            assert!(!is_prime(c), "{} should be composite", c);
        }
    }

    #[test]
    fn primality_large() {
        // Known large primes / composites.
        assert!(is_prime(1_000_003));
        assert!(is_prime(2_147_483_647)); // M_31
        assert!(is_prime((1u64 << 61) - 1)); // M_61
        assert!(!is_prime(1_000_003 * 1_000_033));
        assert!(!is_prime(3_215_031_751)); // strong pseudoprime to base 2,3,5,7
    }

    #[test]
    fn sieve_matches_is_prime() {
        let limit = 10_000;
        let sieved = sieve(limit);
        for n in 2..=limit {
            assert_eq!(sieved.binary_search(&n).is_ok(), is_prime(n), "n = {}", n);
        }
    }

    #[test]
    fn next_and_nth() {
        assert_eq!(next_prime(0), 2);
        assert_eq!(next_prime(1), 2);
        assert_eq!(next_prime(2), 3);
        assert_eq!(next_prime(10), 11);
        assert_eq!(next_prime(100), 101);

        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(6), 13);
        assert_eq!(nth_prime(100), 541);
        assert_eq!(nth_prime(1000), 7919);
    }

    #[test]
    fn factorize() {
        assert_eq!(prime_factorize(0), vec![]);
        assert_eq!(prime_factorize(1), vec![]);
        assert_eq!(prime_factorize(2), vec![(2, 1)]);
        assert_eq!(prime_factorize(12), vec![(2, 2), (3, 1)]);
        assert_eq!(prime_factorize(360), vec![(2, 3), (3, 2), (5, 1)]);
        assert_eq!(prime_factorize(1_000_003), vec![(1_000_003, 1)]);
        // Product of two primes.
        assert_eq!(
            prime_factorize(1_000_003 * 1_000_033),
            vec![(1_000_003, 1), (1_000_033, 1)]
        );
    }

    #[test]
    fn factorize_multiplies_back() {
        for n in 2..1000u64 {
            let product: u64 = prime_factorize(n)
                .into_iter()
                .map(|(p, e)| p.pow(e))
                .product();
            assert_eq!(product, n);
        }
    }

    #[test]
    fn primality_at_u64_boundaries() {
        // The largest prime that fits in u64.
        assert!(is_prime(18_446_744_073_709_551_557));
        // u64::MAX is 2^64 - 1 = 3 · 5 · 17 · 257 · 641 · 65537 · 6700417.
        assert!(!is_prime(u64::MAX));
        // 2^32 + 1 (Fermat number F5) = 641 · 6700417, composite.
        assert!(!is_prime(4_294_967_297));
    }

    #[test]
    fn sieve_edge_cases() {
        assert_eq!(sieve(0), Vec::<u64>::new());
        assert_eq!(sieve(1), Vec::<u64>::new());
        assert_eq!(sieve(2), vec![2]);
        assert_eq!(sieve(3), vec![2, 3]);
        // Correct count: π(100) = 25.
        assert_eq!(sieve(100).len(), 25);
        // π(1000) = 168.
        assert_eq!(sieve(1000).len(), 168);
    }

    #[test]
    fn next_prime_skips_composites() {
        assert_eq!(next_prime(13), 17);
        assert_eq!(next_prime(23), 29);
        // Largest 32-bit prime: next prime after 4_294_967_290 should be 4_294_967_291.
        assert_eq!(next_prime(4_294_967_290), 4_294_967_291);
    }

    #[test]
    fn nth_prime_progression() {
        // Verify monotonicity over a small range.
        let primes_small: Vec<u64> = (1..=200).map(nth_prime).collect();
        for window in primes_small.windows(2) {
            assert!(window[0] < window[1]);
            assert!(is_prime(window[1]));
        }
    }

    #[test]
    #[should_panic]
    fn nth_prime_zero_panics() {
        let _ = nth_prime(0);
    }

    #[test]
    fn factorize_prime_powers() {
        // Pure prime powers should produce a single entry.
        assert_eq!(prime_factorize(1u64 << 32), vec![(2, 32)]);
        assert_eq!(prime_factorize(3u64.pow(20)), vec![(3, 20)]);
        assert_eq!(prime_factorize(7u64.pow(10)), vec![(7, 10)]);
    }

    #[test]
    fn factorize_factors_are_prime_and_sorted() {
        for n in 2..2000u64 {
            let factors = prime_factorize(n);
            for &(p, e) in &factors {
                assert!(is_prime(p), "factor {} of {} is not prime", p, n);
                assert!(e >= 1);
            }
            // Strictly ascending.
            for window in factors.windows(2) {
                assert!(window[0].0 < window[1].0, "n = {}", n);
            }
        }
    }
}
