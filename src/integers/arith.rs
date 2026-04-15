//! Modular arithmetic and number-theoretic algorithms.
//!
//! # Examples
//!
//! ```
//! use numbers_rus::integers::arith::*;
//!
//! assert_eq!(gcd(24, 36), 12);
//! assert_eq!(lcm(4, 6), Some(12));
//! assert_eq!(mod_pow(2, 10, 1000), 24);
//! assert_eq!(mod_inverse(3, 11), Some(4));
//! assert_eq!(euler_totient(36), 12);
//!
//! // CRT: x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → x = 23 (mod 105).
//! assert_eq!(chinese_remainder(&[2, 3, 2], &[3, 5, 7]), Some((23, 105)));
//! ```

use core::mem;

/// Greatest common divisor via Stein's binary algorithm.
///
/// `gcd(0, 0) == 0`. `gcd(a, 0) == a` and `gcd(0, b) == b`.
pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    let mut a = a >> a.trailing_zeros();
    let mut b = b >> b.trailing_zeros();
    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    a << shift
}

/// Least common multiple. Returns `None` on `u64` overflow.
pub fn lcm(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    (a / gcd(a, b)).checked_mul(b)
}

/// Extended Euclidean algorithm.
///
/// Returns `(g, x, y)` such that `a·x + b·y = g = gcd(|a|, |b|)`.
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1i64, 0i64);
    let (mut old_t, mut t) = (0i64, 1i64);
    while r != 0 {
        let q = old_r.div_euclid(r);
        let (nr, ns, nt) = (old_r - q * r, old_s - q * s, old_t - q * t);
        old_r = mem::replace(&mut r, nr);
        old_s = mem::replace(&mut s, ns);
        old_t = mem::replace(&mut t, nt);
    }
    if old_r < 0 {
        (-old_r, -old_s, -old_t)
    } else {
        (old_r, old_s, old_t)
    }
}

/// Modular exponentiation: `(base^exp) mod modulus`.
///
/// Returns `0` when `modulus == 1`, by convention.
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    assert!(modulus != 0, "mod_pow: modulus must be non-zero");
    if modulus == 1 {
        return 0;
    }
    base %= modulus;
    let mut result: u64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}

/// Modular multiplicative inverse: finds `x` with `a·x ≡ 1 (mod m)`.
///
/// Returns `None` iff `gcd(a, m) ≠ 1`.
pub fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    assert!(m > 0, "mod_inverse: modulus must be positive");
    let a = a.rem_euclid(m);
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None
    } else {
        Some(x.rem_euclid(m))
    }
}

/// Solve a system of congruences `x ≡ residues[i] (mod moduli[i])`.
///
/// Returns `(x, N)` where `N = lcm(moduli)` and `0 ≤ x < N`. Returns `None`
/// if the system is inconsistent. Moduli need not be pairwise coprime.
///
/// # Panics
/// If the slices have different lengths, or any modulus is `≤ 0`.
pub fn chinese_remainder(residues: &[i64], moduli: &[i64]) -> Option<(i64, i64)> {
    assert_eq!(
        residues.len(),
        moduli.len(),
        "chinese_remainder: length mismatch"
    );
    let mut x: i64 = 0;
    let mut n: i64 = 1;
    for (&r, &m) in residues.iter().zip(moduli.iter()) {
        assert!(m > 0, "chinese_remainder: moduli must be positive");
        let (g, p, _) = extended_gcd(n, m);
        let diff = r - x;
        if diff.rem_euclid(g) != 0 {
            return None;
        }
        let lcm = n / g * m;
        let step = (diff / g).rem_euclid(m / g) * p;
        x = (x + n * step).rem_euclid(lcm);
        n = lcm;
    }
    Some((x, n))
}

/// Euler's totient φ(n): count of integers in `[1, n]` coprime to `n`.
///
/// φ(0) = 0, φ(1) = 1.
pub fn euler_totient(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut result = n;
    for (p, _) in super::primes::prime_factorize(n) {
        result -= result / p;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_basics() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(0, 9), 9);
        assert_eq!(gcd(24, 36), 12);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(1_000_000, 10), 10);
    }

    #[test]
    fn lcm_basics() {
        assert_eq!(lcm(4, 6), Some(12));
        assert_eq!(lcm(0, 5), Some(0));
        assert_eq!(lcm(u64::MAX, u64::MAX - 1), None); // overflows
    }

    #[test]
    fn extended_gcd_bezout() {
        let (g, x, y) = extended_gcd(240, 46);
        assert_eq!(g, 2);
        assert_eq!(240 * x + 46 * y, g);
    }

    #[test]
    fn modpow_reference() {
        assert_eq!(mod_pow(0, 0, 7), 1);
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(7, 128, 13), 3);
        assert_eq!(mod_pow(u64::MAX - 5, 37, 1_000_000_007), {
            // recompute with u128 for the test oracle
            let (b, e, m) = (u64::MAX - 5, 37u64, 1_000_000_007u64);
            let (mut base, mut exp, mut result) = (b as u128 % m as u128, e, 1u128);
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result * base % m as u128;
                }
                exp >>= 1;
                base = base * base % m as u128;
            }
            result as u64
        });
    }

    #[test]
    fn inverses() {
        assert_eq!(mod_inverse(3, 11), Some(4));
        assert_eq!(mod_inverse(10, 17), Some(12));
        assert_eq!(mod_inverse(6, 9), None); // gcd = 3
    }

    #[test]
    fn crt_classic() {
        assert_eq!(chinese_remainder(&[2, 3, 2], &[3, 5, 7]), Some((23, 105)));
        assert_eq!(chinese_remainder(&[1, 2], &[4, 6]), None); // inconsistent
        assert_eq!(chinese_remainder(&[2, 8], &[3, 12]), Some((8, 12)));
    }

    #[test]
    fn totient() {
        assert_eq!(euler_totient(0), 0);
        assert_eq!(euler_totient(1), 1);
        assert_eq!(euler_totient(9), 6);
        assert_eq!(euler_totient(36), 12);
        // Prime p: φ(p) = p - 1.
        for &p in &[2u64, 3, 13, 101, 1009, 999_983] {
            assert_eq!(euler_totient(p), p - 1);
        }
    }

    #[test]
    fn gcd_idempotent_and_symmetric() {
        for n in 1..200u64 {
            assert_eq!(gcd(n, n), n);
            for m in 1..200u64 {
                assert_eq!(gcd(n, m), gcd(m, n));
            }
        }
    }

    #[test]
    fn lcm_overflow_detection() {
        // Two distinct primes near sqrt(u64::MAX) — product just barely fits.
        let p = 4_294_967_291u64;
        let q = 4_294_967_279u64;
        assert!(lcm(p, q).is_some());
        // But scaling pushes it over.
        assert!(lcm(p, q.saturating_mul(2)).is_none());
    }

    #[test]
    fn extended_gcd_negatives_and_zero() {
        let (g, x, y) = extended_gcd(-12, 18);
        assert_eq!(g, 6);
        assert_eq!(-12 * x + 18 * y, g);

        let (g, x, _) = extended_gcd(7, 0);
        assert_eq!((g, x), (7, 1));
        let (g, _, y) = extended_gcd(0, 9);
        assert_eq!((g, y), (9, 1));
    }

    #[test]
    fn mod_pow_edges() {
        // Anything mod 1 is 0.
        assert_eq!(mod_pow(123, 456, 1), 0);
        // 0^0 by convention is 1; 0^positive is 0.
        assert_eq!(mod_pow(0, 0, 7), 1);
        assert_eq!(mod_pow(0, 5, 7), 0);
        // a^0 = 1.
        assert_eq!(mod_pow(13, 0, 7), 1);
        // a^1 = a mod m.
        assert_eq!(mod_pow(13, 1, 7), 6);
    }

    #[test]
    #[should_panic]
    fn mod_pow_zero_modulus_panics() {
        let _ = mod_pow(2, 3, 0);
    }

    #[test]
    fn mod_inverse_roundtrip() {
        for m in [11i64, 13, 17, 23, 101, 1_000_000_007] {
            for a in 1..50i64 {
                if let Some(inv) = mod_inverse(a, m) {
                    assert_eq!((a * inv).rem_euclid(m), 1, "a={}, m={}", a, m);
                }
            }
        }
    }

    #[test]
    fn crt_satisfies_each_congruence() {
        let residues = [1i64, 2, 3, 4];
        let moduli = [5i64, 7, 11, 13];
        let (x, n) = chinese_remainder(&residues, &moduli).unwrap();
        for (&r, &m) in residues.iter().zip(moduli.iter()) {
            assert_eq!(x.rem_euclid(m), r);
        }
        // n should be the product (all coprime).
        assert_eq!(n, 5 * 7 * 11 * 13);
    }

    #[test]
    fn crt_single_element() {
        assert_eq!(chinese_remainder(&[5], &[7]), Some((5, 7)));
        // Out of canonical range: 17 mod 7 = 3.
        assert_eq!(chinese_remainder(&[17], &[7]), Some((3, 7)));
    }

    #[test]
    fn totient_powers_of_two() {
        // φ(2^k) = 2^(k-1) for k >= 1.
        for k in 1..20u32 {
            let n = 1u64 << k;
            assert_eq!(euler_totient(n), n / 2);
        }
    }

    #[test]
    fn totient_multiplicative_on_coprime() {
        // φ(mn) = φ(m)φ(n) when gcd(m, n) = 1.
        let pairs = [(3u64, 5), (7, 11), (4, 9), (8, 15), (13, 17)];
        for (m, n) in pairs {
            assert_eq!(gcd(m, n), 1);
            assert_eq!(euler_totient(m * n), euler_totient(m) * euler_totient(n));
        }
    }
}
