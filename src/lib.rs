//! # numbers_rus
//!
//! A number-theory and exact-arithmetic toolkit for Rust, organised around
//! three pillars:
//!
//! 1. **Number theory** — deterministic Miller-Rabin primality, Sieve of
//!    Eratosthenes, wheel-factored integer factorization, GCD/LCM via the
//!    binary Euclidean algorithm, modular exponentiation and inversion,
//!    Euler's totient, and the Chinese Remainder Theorem.
//!    See [`integers::primes`] and [`integers::arith`].
//! 2. **Exact arithmetic** — generic [`rational::Rational`] over any signed
//!    integer type (auto-reducing), and generic [`complex::Complex`] with
//!    full operator overloading.
//! 3. **Symbolic objects** — typed [`equation::Equation`] and single-variable
//!    [`equation::Polynomial`] with Horner evaluation, differentiation, and
//!    arithmetic.
//!
//! A lightweight [`stats`] module rounds this out with `mean`, `median`,
//! `variance`, `std_dev`, `quartiles`, and `iqr` over any slice of a numeric
//! type convertible to `f64`.
//!
//! ## Quick tour
//!
//! ```
//! use numbers_rus::integers::{primes, arith, sequences};
//! use numbers_rus::rational::Rational;
//! use numbers_rus::equation::{Equation, Op, Polynomial};
//!
//! // Primality in microseconds, even for large u64.
//! assert!(primes::is_prime(1_000_003));
//! assert!(!primes::is_prime(1_000_004));
//!
//! // Modular arithmetic for crypto-adjacent problems.
//! assert_eq!(arith::mod_pow(2, 10, 1000), 24);
//! assert_eq!(arith::mod_inverse(3, 11), Some(4));
//!
//! // Exact rationals.
//! let r = Rational::new(2i64, 4) + Rational::new(1, 3);
//! assert_eq!(format!("{}", r), "5/6");
//!
//! // Typed equations (no sentinel-value bugs).
//! let mut eq = Equation::new(0i64, 0, Op::Add);
//! assert_eq!(eq.solve(), 0); // zero is a valid solution, not "not computed"
//!
//! // Polynomials.
//! let p = Polynomial::new(vec![1, 0, -1]); // 1 - x^2
//! assert_eq!(p.eval(3), -8);
//! ```
//!
//! ## Feature flags
//!
//! - `std` *(default)* — enables `std`-dependent APIs (currently everything;
//!   `no_std` support is planned for a future release).
//! - `serde` — derives `Serialize` and `Deserialize` on [`rational::Rational`],
//!   [`complex::Complex`], [`equation::Equation`], [`equation::Op`], and
//!   [`equation::Polynomial`].
//!
//! ## Migrating from 0.2
//!
//! The 1.0 release is a substantial reshape. See the [CHANGELOG] for the full
//! rename table; the highlights:
//!
//! - `solve::equation::Equation::get_sol()` is now [`equation::Equation::solve`],
//!   backed by `Option<T>` rather than a `0` sentinel.
//! - `integers::base::is_prime` is replaced by [`integers::primes::is_prime`]
//!   (Miller-Rabin, deterministic for `u64`).
//! - `single::single_vector::*` is replaced by [`stats`], which takes `&[T]`
//!   and returns numeric results instead of `String`.
//! - `structures::dataframe`, `vector::vector`, and `numbers::complex` have
//!   been removed.
//!
//! [CHANGELOG]: https://github.com/ooyeku/numbers_rus/blob/master/CHANGELOG.md

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod integers;
pub mod rational;
pub mod complex;
pub mod equation;
pub mod stats;
