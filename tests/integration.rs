//! End-to-end use-case tests that exercise multiple modules together.
//!
//! These also serve as executable documentation for the use cases described
//! in the README: contest problems, crypto prototyping, recreational math.

use numbers_rus::complex::Complex;
use numbers_rus::integers::{arith, primes, properties, sequences};
use numbers_rus::rational::Rational;

/// Toy RSA round-trip: generate keys, encrypt, decrypt, verify.
///
/// Uses small `u64`-fitting primes — illustrative only, not secure.
#[test]
fn toy_rsa_roundtrip() {
    // 1. Pick two primes.
    let p: u64 = 61;
    let q: u64 = 53;
    assert!(primes::is_prime(p) && primes::is_prime(q));

    // 2. Modulus and totient.
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    assert_eq!(phi, arith::euler_totient(n));

    // 3. Public exponent (coprime to phi).
    let e: u64 = 17;
    assert_eq!(arith::gcd(e, phi), 1);

    // 4. Private exponent: d ≡ e⁻¹ (mod phi).
    let d = arith::mod_inverse(e as i64, phi as i64).expect("inverse exists") as u64;
    assert_eq!((e * d) % phi, 1);

    // 5. Encrypt / decrypt every plaintext in [0, n).
    for m in 0..n {
        let cipher = arith::mod_pow(m, e, n);
        let plain = arith::mod_pow(cipher, d, n);
        assert_eq!(plain, m, "m = {}", m);
    }
}

/// Project Euler #3 style: largest prime factor of a given number.
#[test]
fn largest_prime_factor() {
    let n: u64 = 600_851_475_143;
    let factors = primes::prime_factorize(n);
    let largest = factors.iter().map(|&(p, _)| p).max().unwrap();
    assert_eq!(largest, 6857);
}

/// Project Euler #7 style: the 10001st prime.
#[test]
fn ten_thousand_first_prime() {
    assert_eq!(primes::nth_prime(10_001), 104_743);
}

/// Project Euler #21 style: sum of all amicable numbers below 10000.
///
/// Two numbers are amicable when σ_proper(a) = b and σ_proper(b) = a, with a ≠ b.
#[test]
fn amicable_numbers_under_10000() {
    let proper_sum = |n: u64| properties::divisor_sum(n) - n;
    let mut total: u64 = 0;
    for a in 2..10_000u64 {
        let b = proper_sum(a);
        if b > a && b < 10_000 && proper_sum(b) == a {
            total += a + b;
        }
    }
    assert_eq!(total, 31626);
}

/// Rational arithmetic preserves exactness across many operations — proof by
/// constructing a sum that would lose precision in `f64`.
#[test]
fn rational_sum_is_exact() {
    // Sum of 1/k for k in 1..=10 equals 7381/2520.
    let s = (1..=10i64)
        .map(|k| Rational::new(1, k))
        .fold(Rational::<i64>::zero(), |acc, r| acc + r);
    assert_eq!(s, Rational::new(7381, 2520));
}

/// Complex arithmetic: roots of unity multiply correctly.
#[test]
fn complex_roots_of_unity() {
    use std::f64::consts::TAU;
    // 6th roots of unity multiplied together equal -1 (or +1 depending on
    // count; we pick all 6 → product is (-1)^(6-1) = -1 for primitive case).
    let roots: Vec<Complex<f64>> = (0..6)
        .map(|k| Complex::from_polar(1.0, TAU * k as f64 / 6.0))
        .collect();
    // Each root has unit norm.
    for r in &roots {
        assert!((r.norm() - 1.0).abs() < 1e-12);
    }
    // Product of all n-th roots of unity is 1 if n is odd, -1 if n is even.
    let product = roots.into_iter().fold(Complex::<f64>::one(), |acc, r| acc * r);
    assert!((product.re - (-1.0)).abs() < 1e-9);
    assert!(product.im.abs() < 1e-9);
}

/// Pythagorean triple search using only this crate's primitives.
#[test]
fn pythagorean_triple_with_perfect_square() {
    // Find a primitive triple (a, b, c) with c < 30: classic (3, 4, 5).
    let mut found = None;
    'outer: for a in 1u64..30 {
        for b in a..30 {
            let c_sq = a * a + b * b;
            if properties::is_perfect_square(c_sq) {
                let c = (c_sq as f64).sqrt() as u64;
                if arith::gcd(a, b) == 1 && c < 30 {
                    found = Some((a, b, c));
                    break 'outer;
                }
            }
        }
    }
    assert_eq!(found, Some((3, 4, 5)));
}

/// Diffie-Hellman key exchange: shared secret matches on both sides.
#[test]
fn diffie_hellman_shared_secret() {
    // Public parameters: prime modulus and generator.
    let p: u64 = 23;
    let g: u64 = 5;
    assert!(primes::is_prime(p));

    // Alice and Bob pick private exponents.
    let alice_priv: u64 = 6;
    let bob_priv: u64 = 15;

    // Public values.
    let alice_pub = arith::mod_pow(g, alice_priv, p);
    let bob_pub = arith::mod_pow(g, bob_priv, p);

    // Shared secrets (computed independently).
    let alice_shared = arith::mod_pow(bob_pub, alice_priv, p);
    let bob_shared = arith::mod_pow(alice_pub, bob_priv, p);
    assert_eq!(alice_shared, bob_shared);
}

/// Recreational: verify that 28 is perfect by definition (sum of proper divisors).
#[test]
fn perfect_number_definition() {
    let n: u64 = 28;
    let proper: Vec<u64> = properties::divisors(n).into_iter().filter(|&d| d != n).collect();
    assert_eq!(proper, vec![1, 2, 4, 7, 14]);
    assert_eq!(proper.iter().sum::<u64>(), n);
    assert!(properties::is_perfect_number(n));
}

/// Combinatorics: C(n, k) = n! / (k! · (n-k)!) using `factorial`.
#[test]
fn binomial_coefficient_via_factorial() {
    fn binomial(n: u64, k: u64) -> u128 {
        sequences::factorial(n).unwrap()
            / (sequences::factorial(k).unwrap() * sequences::factorial(n - k).unwrap())
    }
    // Pascal's triangle row 6.
    let row: Vec<u128> = (0..=6).map(|k| binomial(6, k)).collect();
    assert_eq!(row, vec![1, 6, 15, 20, 15, 6, 1]);
}
