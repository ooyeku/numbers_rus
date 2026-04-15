//! Integer algorithms, grouped by family.
//!
//! - [`primes`] — primality testing, sieves, factorization.
//! - [`arith`] — GCD/LCM, modular exponentiation, modular inverse, CRT,
//!   Euler's totient.
//! - [`properties`] — divisor-based predicates and the general perfect-power
//!   detector.
//! - [`sequences`] — classical sequences (factorial, Fibonacci, triangular)
//!   implemented iteratively with overflow checks.

pub mod primes;
pub mod arith;
pub mod properties;
pub mod sequences;
