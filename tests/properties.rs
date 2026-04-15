//! Property tests for algebraic identities.

use numbers_rus::equation::Polynomial;
use numbers_rus::integers::{arith, primes, sequences};
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
}
