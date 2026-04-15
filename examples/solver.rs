//! Random-equation demo.
//!
//! Generates random integer equations, solves them, and annotates each result
//! with its primality, parity, and (for composites) prime factorization.
//!
//! Run with: `cargo run --example solver --release`.

use numbers_rus::equation::{Equation, Op};
use numbers_rus::integers::{primes, properties};
use rand::RngExt;
use std::io::{self, Write};

fn main() {
    println!();
    println!("numbers_rus :: solver demo");
    println!("==========================");
    println!("Generates random integer equations, prints each solution, and");
    println!("annotates it with primality, parity, and factorization.\n");

    let reps = prompt_reps();
    let mut rng = rand::rng();
    let ops = [Op::Add, Op::Sub, Op::Mul];

    let start = std::time::Instant::now();
    for _ in 0..reps {
        let a: i64 = rng.random_range(1..=1_000_000);
        let b: i64 = rng.random_range(1..=1_000_000);
        let op = ops[rng.random_range(0..ops.len())];

        let mut eq = Equation::new(a, b, op);
        let sol = eq.solve();

        println!("{} = {}", eq, sol);

        if sol > 1 {
            let n = sol as u64;
            if primes::is_prime(n) {
                println!("  → prime");
            } else {
                let factors = primes::prime_factorize(n);
                let rendered: Vec<String> = factors
                    .iter()
                    .map(|&(p, e)| if e == 1 { p.to_string() } else { format!("{}^{}", p, e) })
                    .collect();
                println!("  → composite: {}", rendered.join(" · "));
            }
        }

        println!(
            "  → parity: {}",
            if properties::is_even(sol) { "even" } else { "odd" }
        );
    }
    println!("\nTotal time for {} reps: {:?}", reps, start.elapsed());
}

fn prompt_reps() -> u32 {
    loop {
        print!("How many reps? (blank to exit) › ");
        io::stdout().flush().ok();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("stdin read failed");
        if s.trim().is_empty() {
            std::process::exit(0);
        }
        match s.trim().parse::<u32>() {
            Ok(n) if n > 0 => return n,
            _ => println!("Please enter a positive integer."),
        }
    }
}
