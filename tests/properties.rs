//! Property tests for algebraic identities.

use numbers_rus::complex::Complex;
use numbers_rus::equation::Polynomial;
use numbers_rus::integers::{arith, primes, properties, sequences};
use numbers_rus::rational::Rational;
use proptest::prelude::*;

proptest! {
    /// `gcd(a, b) * lcm(a, b) == a * b` whenever the lcm fits.
    #[test]
    fn gcd_lcm_identity(a in 1u64..10_000, b in 1u64..10_000) {
        let g = arith::gcd(a, b);
        let l = arith::lcm(a, b).expect("lcm fits in u64 for these bounds");
        prop_assert_eq!(g * l, a * b);
    }

    /// Miller-Rabin agrees with naive trial division on small inputs.
    #[test]
    fn primality_matches_trial_division(n in 0u64..20_000) {
        fn naive(n: u64) -> bool {
            if n < 2 { return false; }
            let mut i = 2u64;
            while i * i <= n {
                if n % i == 0 { return false; }
                i += 1;
            }
            true
        }
        prop_assert_eq!(primes::is_prime(n), naive(n));
    }

    /// Prime-factor product reconstructs the input.
    #[test]
    fn factorization_roundtrip(n in 2u64..100_000) {
        let product: u64 = primes::prime_factorize(n)
            .into_iter()
            .map(|(p, e)| p.pow(e))
            .product();
        prop_assert_eq!(product, n);
    }

    /// Modular inverse is an inverse.
    #[test]
    fn mod_inverse_is_inverse(a in 1i64..1_000_000, m in 2i64..1_000_000) {
        if let Some(inv) = arith::mod_inverse(a, m) {
            prop_assert_eq!((a * inv).rem_euclid(m), 1);
        }
    }

    /// Rational addition is commutative.
    #[test]
    fn rational_add_commutes(a in -1000i64..1000, b in 1i64..1000, c in -1000i64..1000, d in 1i64..1000) {
        let x = Rational::new(a, b);
        let y = Rational::new(c, d);
        prop_assert_eq!(x + y, y + x);
    }

    /// A rational remains equal after a trivial rescaling.
    #[test]
    fn rational_rescale(n in -1000i64..1000, d in 1i64..1000, k in 1i64..100) {
        let a = Rational::new(n, d);
        let b = Rational::new(n * k, d * k);
        prop_assert_eq!(a, b);
    }

    /// Horner evaluation matches the naive power-based eval.
    #[test]
    fn horner_matches_naive(coeffs in proptest::collection::vec(-50i64..50, 0..8), x in -20i64..20) {
        let p = Polynomial::new(coeffs.clone());
        let naive: i64 = coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| c * x.pow(i as u32))
            .sum();
        prop_assert_eq!(p.eval(x), naive);
    }

    /// Fibonacci satisfies F(n) = F(n-1) + F(n-2).
    #[test]
    fn fibonacci_recurrence(n in 2u64..80) {
        let a = sequences::fibonacci(n - 2).unwrap();
        let b = sequences::fibonacci(n - 1).unwrap();
        let c = sequences::fibonacci(n).unwrap();
        prop_assert_eq!(a + b, c);
    }

    /// GCD is symmetric.
    #[test]
    fn gcd_symmetric(a in 0u64..100_000, b in 0u64..100_000) {
        prop_assert_eq!(arith::gcd(a, b), arith::gcd(b, a));
    }

    /// GCD divides both inputs.
    #[test]
    fn gcd_divides_inputs(a in 1u64..100_000, b in 1u64..100_000) {
        let g = arith::gcd(a, b);
        prop_assert!(g > 0);
        prop_assert_eq!(a % g, 0);
        prop_assert_eq!(b % g, 0);
    }

    /// Extended GCD returns Bézout coefficients.
    #[test]
    fn extended_gcd_bezout_identity(a in -10_000i64..10_000, b in -10_000i64..10_000) {
        let (g, x, y) = arith::extended_gcd(a, b);
        prop_assert_eq!(a * x + b * y, g);
    }

    /// Sum of φ(d) over divisors d of n equals n. (Gauss's identity.)
    #[test]
    fn totient_divisor_sum(n in 1u64..2_000) {
        let s: u64 = properties::divisors(n)
            .into_iter()
            .map(arith::euler_totient)
            .sum();
        prop_assert_eq!(s, n);
    }

    /// Fermat's little theorem: a^(p-1) ≡ 1 (mod p) for any 0 < a < p, p prime.
    #[test]
    fn fermat_little_theorem(p_idx in 1usize..200, a_off in 1u64..1_000) {
        let p = primes::nth_prime(p_idx);
        let a = a_off % (p - 1) + 1;
        prop_assert_eq!(arith::mod_pow(a, p - 1, p), 1);
    }

    /// CRT round-trip: each output residue matches the corresponding input.
    #[test]
    fn crt_roundtrip(rs in proptest::collection::vec(0i64..50, 1..5)) {
        // Use the first len() distinct primes as moduli — guaranteed coprime.
        let moduli: Vec<i64> = (1..=rs.len())
            .map(|i| primes::nth_prime(i) as i64)
            .collect();
        let residues: Vec<i64> = rs.iter().zip(&moduli).map(|(r, m)| r % m).collect();
        let (x, _) = arith::chinese_remainder(&residues, &moduli).unwrap();
        for (&r, &m) in residues.iter().zip(&moduli) {
            prop_assert_eq!(x.rem_euclid(m), r);
        }
    }

    /// Rational addition is associative.
    #[test]
    fn rational_add_associative(
        a in -200i64..200, b in 1i64..200,
        c in -200i64..200, d in 1i64..200,
        e in -200i64..200, f in 1i64..200,
    ) {
        let x = Rational::new(a, b);
        let y = Rational::new(c, d);
        let z = Rational::new(e, f);
        prop_assert_eq!((x + y) + z, x + (y + z));
    }

    /// Rational division equals multiplication by reciprocal.
    #[test]
    fn rational_div_is_mul_recip(
        a in -200i64..200, b in 1i64..200,
        c in -200i64..200, d in 1i64..200,
    ) {
        prop_assume!(c != 0);
        let x = Rational::new(a, b);
        let y = Rational::new(c, d);
        prop_assert_eq!(x / y, x * y.recip());
    }

    /// Sum of polynomials evaluates to sum of evaluations.
    #[test]
    fn polynomial_eval_distributes_over_addition(
        p_coeffs in proptest::collection::vec(-20i64..20, 0..6),
        q_coeffs in proptest::collection::vec(-20i64..20, 0..6),
        x in -10i64..10,
    ) {
        let p = Polynomial::new(p_coeffs);
        let q = Polynomial::new(q_coeffs);
        prop_assert_eq!((p.clone() + q.clone()).eval(x), p.eval(x) + q.eval(x));
    }

    /// Complex norm is multiplicative: |z·w|² = |z|²·|w|².
    #[test]
    fn complex_norm_multiplicative(
        a in -100i64..100, b in -100i64..100,
        c in -100i64..100, d in -100i64..100,
    ) {
        let z: Complex<i64> = Complex::new(a, b);
        let w: Complex<i64> = Complex::new(c, d);
        prop_assert_eq!((z * w).norm_sqr(), z.norm_sqr() * w.norm_sqr());
    }
}
