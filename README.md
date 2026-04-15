# numbers_rus

[![Crates.io](https://img.shields.io/crates/v/numbers_rus.svg)](https://crates.io/crates/numbers_rus)
[![Documentation](https://docs.rs/numbers_rus/badge.svg)](https://docs.rs/numbers_rus)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)](./LICENSE)

**Number-theory primitives and exact arithmetic for Rust — built for
competitive programming, teaching, and recreational math.**

A curated, well-documented toolkit for the discrete-math corner of numerics.
Narrow on purpose: you get fast, correct implementations of the primitives
that actually show up in contests, classrooms, and Project Euler, without
decoding a meta-crate's trait hierarchy first.

Three pillars:

1. **Number theory.** Deterministic Miller-Rabin primality for the full `u64`
   range, the Sieve of Eratosthenes, wheel-factored prime factorization,
   GCD/LCM via Stein's binary algorithm, modular exponentiation and inversion,
   Euler's totient, the Chinese Remainder Theorem, and divisor functions.
2. **Exact arithmetic.** Generic `Rational<T>` and `Complex<T>` with the
   operator overloads you'd expect, a trimmed `Polynomial<T>` with Horner
   evaluation and formal differentiation, and typed `Equation<T>` objects
   that don't silently hide zero-valued answers.
3. **Descriptive statistics.** `mean`, `median`, `variance`, `std_dev`,
   `quartiles`, `iqr`, `mode` — everything over `&[T]`, returning numbers
   instead of `String`s.

## Is this for you?

**Reach for `numbers_rus` when you're:**
- Solving Project Euler, Codeforces, Advent of Code, or other contest-style
  problems in Rust and tired of re-rolling primality and modular arithmetic.
- Teaching or studying discrete math, intro number theory, or crypto
  fundamentals and want a readable Rust companion to the textbook.
- Prototyping a toy RSA, Diffie-Hellman, Shamir secret sharing, or other
  classroom-scale crypto demo.
- Building a small CAS, a constant-folder, or a units library that needs
  exact rationals, complex numbers, or polynomial evaluation.
- Exploring recreational math — perfect numbers, amicable pairs, aliquot
  sequences, OEIS — and want the primitives out of the box.

**Reach for something else when you need:**
- **Linear algebra, matrices, SIMD numerics** → [`nalgebra`], [`ndarray`],
  [`faer`].
- **DataFrames or tabular data** → [`polars`].
- **Production statistics (distributions, hypothesis tests, regression)** →
  [`statrs`].
- **Arbitrary-precision or constant-time cryptography** → [`num-bigint`] +
  [`rug`], [`rsa`], [`k256`]. `numbers_rus` is `u64`-ceiling and **not**
  hardened against timing attacks.
- **Floating-point-heavy scientific computing** → the `nalgebra` / `ndarray`
  ecosystem plus [`libm`].

[`nalgebra`]: https://crates.io/crates/nalgebra
[`ndarray`]: https://crates.io/crates/ndarray
[`faer`]: https://crates.io/crates/faer
[`polars`]: https://crates.io/crates/polars
[`statrs`]: https://crates.io/crates/statrs
[`num-bigint`]: https://crates.io/crates/num-bigint
[`rug`]: https://crates.io/crates/rug
[`rsa`]: https://crates.io/crates/rsa
[`k256`]: https://crates.io/crates/k256
[`libm`]: https://crates.io/crates/libm

## Install

```toml
[dependencies]
numbers_rus = "1.0"
```

Optional features:

```toml
numbers_rus = { version = "1.0", features = ["serde"] }
```

## Five-minute tour

```rust
use numbers_rus::integers::{arith, primes, properties, sequences};
use numbers_rus::rational::Rational;
use numbers_rus::complex::Complex;
use numbers_rus::equation::{Equation, Op, Polynomial};
use numbers_rus::stats;

// ─── Number theory ──────────────────────────────────────────────────────────

assert!(primes::is_prime(1_000_003));
assert!(primes::is_prime((1u64 << 61) - 1)); // Mersenne M_61
assert_eq!(primes::prime_factorize(360), vec![(2, 3), (3, 2), (5, 1)]);

assert_eq!(arith::gcd(252, 105), 21);
assert_eq!(arith::mod_pow(2, 256, 97), 61);
assert_eq!(arith::mod_inverse(17, 101), Some(6));
assert_eq!(arith::chinese_remainder(&[2, 3, 2], &[3, 5, 7]), Some((23, 105)));

assert!(properties::is_perfect_number(496));
assert_eq!(properties::perfect_power(1024), Some((2, 10)));

assert_eq!(sequences::factorial(20), Some(2_432_902_008_176_640_000));

// ─── Exact arithmetic ───────────────────────────────────────────────────────

let r = Rational::new(2i64, 4) + Rational::new(1, 3);
assert_eq!(format!("{}", r), "5/6");

let z = Complex::new(1.0, 2.0) * Complex::new(3.0, -4.0);
assert_eq!(z, Complex::new(11.0, 2.0));

let p = Polynomial::new(vec![5i64, -2, 0, 3]); // 3x³ - 2x + 5
assert_eq!(p.eval(2), 25);
assert_eq!(p.derivative().coefficients(), &[-2, 0, 9]);

// ─── Equations that don't lie about zero ────────────────────────────────────

let mut eq = Equation::new(3i64, 3, Op::Sub);
assert_eq!(eq.solve(), 0); // zero is a real answer, not "not computed"

// ─── Stats ──────────────────────────────────────────────────────────────────

let xs = [2.0f64, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
assert_eq!(stats::mean(&xs), Some(5.0));
assert_eq!(stats::std_dev(&xs), Some(2.0));
```

## Module map

| Module | What's inside |
| --- | --- |
| [`integers::primes`] | `is_prime`, `sieve`, `next_prime`, `nth_prime`, `prime_factorize` |
| [`integers::arith`] | `gcd`, `lcm`, `extended_gcd`, `mod_pow`, `mod_inverse`, `chinese_remainder`, `euler_totient` |
| [`integers::properties`] | `is_even`, `is_odd`, `is_perfect_square`, `is_perfect_cube`, `perfect_power`, `divisors`, `divisor_count`, `divisor_sum`, `is_perfect_number` |
| [`integers::sequences`] | `factorial`, `fibonacci`, `triangular` |
| [`rational::Rational`] | Generic auto-reducing rational over a signed integer |
| [`complex::Complex`] | Generic complex number |
| [`equation::Equation`] | Typed binary equation with an `Op` enum |
| [`equation::Polynomial`] | Single-variable polynomial with Horner eval and differentiation |
| [`stats`] | `mean`, `median`, `variance`, `std_dev`, `range`, `quartiles`, `iqr`, `mode`, `sum`, `product` |

[`integers::primes`]: https://docs.rs/numbers_rus/latest/numbers_rus/integers/primes/index.html
[`integers::arith`]: https://docs.rs/numbers_rus/latest/numbers_rus/integers/arith/index.html
[`integers::properties`]: https://docs.rs/numbers_rus/latest/numbers_rus/integers/properties/index.html
[`integers::sequences`]: https://docs.rs/numbers_rus/latest/numbers_rus/integers/sequences/index.html
[`rational::Rational`]: https://docs.rs/numbers_rus/latest/numbers_rus/rational/struct.Rational.html
[`complex::Complex`]: https://docs.rs/numbers_rus/latest/numbers_rus/complex/struct.Complex.html
[`equation::Equation`]: https://docs.rs/numbers_rus/latest/numbers_rus/equation/struct.Equation.html
[`equation::Polynomial`]: https://docs.rs/numbers_rus/latest/numbers_rus/equation/polynomial/struct.Polynomial.html
[`stats`]: https://docs.rs/numbers_rus/latest/numbers_rus/stats/index.html

## Examples

```bash
cargo run --example primes_tour --release
cargo run --example solver --release
```

## Benchmarks

```bash
cargo bench
```

Criterion output lands in `target/criterion/`. The benchmark suite exercises
`is_prime`, `sieve`, and `prime_factorize` on inputs from ~10⁶ up to the
Mersenne prime `2⁶¹ − 1`.

## Testing

```bash
cargo test            # unit + integration + property tests
cargo test --doc      # doctests
cargo test --all-features
```

Property tests (in `tests/properties.rs`, driven by `proptest`) verify:
- `gcd(a, b) · lcm(a, b) = a · b`
- Miller-Rabin ≡ trial division on `[0, 20 000]`
- Prime factorization round-trips
- Modular inverses actually invert
- `Rational` equality under rescaling
- Horner evaluation ≡ the naive power-sum form
- The Fibonacci recurrence

## Migrating from 0.2

1.0 is a substantial reshape. The complete rename table and removal list is
in [CHANGELOG.md](CHANGELOG.md). The ones you're most likely to hit:

| Before (0.2) | After (1.0) |
| --- | --- |
| `solve::equation::Equation::get_sol` | `equation::Equation::solve` |
| `solve::equation::Equation::new(a, b, '+')` | `equation::Equation::new(a, b, Op::Add)` |
| `solve::complex_float_equations`, `solve::rational_integer_equation`, … | `equation::Equation<T>` (one generic type) |
| `integers::base::is_prime` | `integers::primes::is_prime` (now Miller-Rabin) |
| `integers::base::factorial` (recursive) | `integers::sequences::factorial` → `Option<u128>` |
| `integers::base::fibonacci` (recursive) | `integers::sequences::fibonacci` → `Option<u128>` |
| `integers::base::is_perfect_power` (buggy) | `integers::properties::perfect_power` |
| `numbers::complex_floats::Complex` | `complex::Complex<f64>` |
| `integers::complex_integers::Complex` | `complex::Complex<i64>` |
| `rational::rational_integer::Rational`, `rational::rational_float::Rational` | `rational::Rational<T>` |
| `single::single_vector::vector_mean` | `stats::mean` (returns `f64`) |
| `single::single_vector::vector_variance` | `stats::variance` (returns `f64`, not `String`) |
| `vector::vector::vector_add` | use `Vec` iterators: `a.iter().zip(&b).map(|(x, y)| x + y).collect()` |
| `structures::dataframe::DataFrame` | removed (use `polars` or `ndarray`) |

## Roadmap

The 1.0 surface is stable. Targeted additions for the 1.x series, prioritised
around the contest / teaching / recreational use cases:

- Pollard ρ factorization for numbers beyond the wheel-division sweet spot
  (the contest killer for large semiprimes).
- Segmented Sieve of Eratosthenes for prime enumeration past ~10⁸.
- A `bigint` feature forwarding `Rational<BigInt>` and `Complex<BigInt>`,
  unlocking Project Euler problems that overflow `u64`.
- Rational and polynomial GCD, squarefree factorization, and rational
  polynomial roots — the next layer of CAS-style primitives.
- `#![no_std]` support for the number-theory core (WASM and embedded
  demos).

Breaking additions wait for 2.0.

## Contributing

Issues and pull requests welcome. Please keep changes focused — a new
algorithm with a correctness test, a property test, and a benchmark carries
a lot more weight than a formatting pass.

## License

Dual-licensed under MIT or Apache-2.0, at your option.
