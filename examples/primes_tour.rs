//! A short tour of the `integers::primes` and `integers::arith` modules.
//!
//! Run with: `cargo run --example primes_tour --release`.

use numbers_rus::integers::{arith, primes, properties, sequences};

fn main() {
    println!("numbers_rus :: primes tour\n");

    // 1. Primality.
    println!("First 25 primes: {:?}", primes::sieve(100));
    println!("is_prime(1_000_003)   = {}", primes::is_prime(1_000_003));
    println!("is_prime(2^61 - 1)    = {}", primes::is_prime((1u64 << 61) - 1));
    println!("next_prime(1_000_000) = {}", primes::next_prime(1_000_000));
    println!("nth_prime(10_000)     = {}", primes::nth_prime(10_000));

    // 2. Factorization.
    let n = 2_310_000u64;
    println!("\nprime_factorize({n}) = {:?}", primes::prime_factorize(n));
    println!("divisor_count({n}) = {}", properties::divisor_count(n));
    println!("divisor_sum({n})   = {}", properties::divisor_sum(n));

    // 3. Modular arithmetic.
    println!("\ngcd(252, 105)        = {}", arith::gcd(252, 105));
    println!("lcm(252, 105)        = {:?}", arith::lcm(252, 105));
    println!("mod_pow(2, 256, 97)  = {}", arith::mod_pow(2, 256, 97));
    println!("mod_inverse(17, 101) = {:?}", arith::mod_inverse(17, 101));
    println!("φ(1_000_000)         = {}", arith::euler_totient(1_000_000));

    // 4. CRT: the classical Sun Zi problem.
    let crt = arith::chinese_remainder(&[2, 3, 2], &[3, 5, 7]);
    println!("\nCRT x ≡ 2 (mod 3), 3 (mod 5), 2 (mod 7): {:?}", crt);

    // 5. Sequences.
    println!("\n20!  = {:?}", sequences::factorial(20));
    println!("F(50) = {:?}", sequences::fibonacci(50));

    // 6. Perfect numbers in [1, 10000].
    let perfects: Vec<u64> = (1..=10_000).filter(|&n| properties::is_perfect_number(n)).collect();
    println!("\nPerfect numbers ≤ 10_000: {:?}", perfects);
}
