use std::thread::sleep;
/// To run this example, use `cargo run --example solver`

use numbers_rus::solve::equation::*;
use rand::Rng;
fn main(){
    println!();
    println!();
    println!("Demonstrating the Equation struct from the numbers_rus::solve module.");
    sleep(std::time::Duration::from_secs(1));
    println!("The Example below creates an equation with two random numbers between 1 and 1,000,000 and");
    sleep(std::time::Duration::from_secs(1));
    println!("then prints the equation and the solution.  It then checks to see if the solution is prime");
    sleep(std::time::Duration::from_secs(1));
    println!("and if it is odd.  It then changes the operator to subtraction and repeats the process.");
    sleep(std::time::Duration::from_secs(1));
    println!();
    let mut reps = 0;
    assert_eq!(reps, 0);
    let mut rng = rand::thread_rng();
    loop {
        println!("How many reps would you like to run? (Please enter a number greater than 0)");
        println!("Leave blank and press enter to exit.");
        println!("Enter reps: ");
        let mut reps_str = String::new();
        std::io::stdin().read_line(&mut reps_str).expect("Failed to read line");
        // check for exit
        if reps_str.trim() == "" {
            println!("Exiting...");
            std::process::exit(0);
        }
        reps = reps_str.trim().parse().expect("Please type a number!");
        if reps > 0 {
            break;
        }
    }
    let start = std::time::Instant::now();
    for _ in 0..reps {
        let a = rng.gen_range(1..=1_000_000);
        println!("a = {}", a);
        let b = rng.gen_range(1..=1_000_000);
        println!("b = {}", b);
       // generate random operator
        let operators = ['+', '-', '*', '/'];
        let operator = operators[rng.gen_range(0..operators.len())];
        println!("operator = {}", operator);
        let mut c = Equation::new(a, b, operator);
        println!("{} {} {} = {}", a, operator, b, c.get_sol());
        if numbers_rus::integers::base::is_prime(c.get_sol()) {
            println!("{} is prime", c.get_sol());
        } else {
            println!("{} is not prime", c.get_sol());
        }
        if numbers_rus::integers::base::is_odd(c.get_sol()) {
            println!("{} is odd", c.get_sol());
        } else {
            println!("{} is even", c.get_sol());
        }






















































    }
    // end timer
    let duration = start.elapsed();
    println!("Time to run {} reps: {:?}",reps, duration);
}