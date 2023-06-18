//! equation.rs is a module that contains the Equation, EquationF, and ZeroEquation structs.
//! These structs are used to solve equations with numbers and an operator (+, -, *, /, %, ^)
//! or to solve equations that equal zero.

use crate::integers::base::*;
use crate::floats::base_float::*;

/// Equation struct that can be used to solve equations with two numbers and an operator (+, -, *, /, %, ^)
/// Sol is the solution to the solve and is calculated when get_sol() is called.
///
/// # Example
/// ```
/// use numbers_rus::solve::equation::Equation;
/// let mut solve = Equation::new(1, 2, '+');
/// assert_eq!(solve.get_sol(), 3);
/// ```
pub struct Equation {
        a: i128,
    b: i128,
    operator: char,
    sol: i128,
}
impl Equation {
        /// Creates a new equation
    pub fn new(a: i128, b: i128, operator: char) -> Equation {
            Equation {
                a: a,
                b: b,
                operator: operator,
                sol: 0,
            }
        }
    /// Returns the solution to the equation
    pub fn get_sol(&mut self) -> i128 {
            if self.sol == 0 {
                self.sol = match self.operator {
                    '+' => add(self.a, self.b),
                    '-' => subtract(self.a, self.b),
                    '*' => multiply(self.a, self.b),
                    '/' => divide(self.a, self.b),
                    '%' => modulo(self.a, self.b),
                    '^' => power(self.a, self.b),
                    _ => 0,
                };
            }
        self.sol
        }
    /// sets the a value (first number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_a(&mut self, a: i128) {
            self.a = a;
            self.sol = 0; // reset the solution
        }
    /// sets the b value (second number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_b(&mut self, b: i128) {
            self.b = b;
            self.sol = 0; // reset the solution
        }
    /// sets the operator in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_operator(&mut self, operator: char) {
            self.operator = operator;
            self.sol = 0; // reset the solution
        }
    /// returns the a value
    pub fn get_a(&self) -> i128 {
            self.a
        }
    /// returns the b value
    pub fn get_b(&self) -> i128 {
            self.b
        }
    /// returns the operator
    pub fn get_operator(&self) -> char {
            self.operator
        }
    }
#[cfg(test)]
mod test_equation {
        use super::*;

        #[test]
    fn it_works() {
            let mut equation = Equation::new(1, 2, '+');
            assert_eq!(equation.get_sol(), 3);
        }
    #[test]
    fn set_a() {
            let mut equation = Equation::new(1, 2, '+');
            equation.set_a(3);
            assert_eq!(equation.get_a(), 3);
            assert_eq!(equation.get_sol(), 5);
        }
    #[test]
    fn set_b() {
            let mut equation = Equation::new(1, 2, '+');
            equation.set_b(3);
            assert_eq!(equation.get_b(), 3);
            assert_eq!(equation.get_sol(), 4);
        }
    #[test]
    fn set_operator() {
            let mut equation = Equation::new(1, 2, '+');
            equation.set_operator('-');
            assert_eq!(equation.get_operator(), '-');
            assert_eq!(equation.get_sol(), -1);
        }
    #[test]
    fn change_equation(){
            let mut equation = Equation::new(1, 2, '+');
            assert_eq!(equation.get_sol(), 3);
            equation.set_a(3);
            println!("a: {}", equation.get_a());
            println!("b: {}", equation.get_b());
            println!("sol: {}", equation.get_sol());
            assert_eq!(equation.get_sol(), 5);
            equation.set_b(3);
            println!("a: {}", equation.get_a());
            println!("b: {}", equation.get_b());
            assert_eq!(equation.get_sol(), 6);
        }
    }

/// EquationF is an solve similar to Equation, but with f64 values.  Sol is calculated when get_sol() is called.
/// # Example
/// ```
/// use numbers_rus::solve::equation::EquationF;
/// let mut solve = EquationF::new(1.092, 2.435, '+');
/// assert_eq!(solve.get_sol(), 3.527);
/// ```

pub struct EquationF {
        a: f64,
    b: f64,
    operator: char,
    sol: f64,
}
impl EquationF {
        pub fn new(a: f64, b: f64, operator: char) -> EquationF {
            EquationF {
                a,
                b,
                operator,
                sol: 0.0,
            }
        }
    pub fn get_sol(&mut self) -> f64 {
            if self.sol == 0.0 {
                self.sol = match self.operator {
                    '+' => add_float(self.a, self.b),
                    '-' => subtract_float(self.a, self.b),
                    '*' => multiply_float(self.a, self.b),
                    '/' => divide_float(self.a, self.b),
                    '%' => modulo_float(self.a, self.b),
                    '^' => power_float(self.a, self.b),
                    _ => 0.0,
                };
            }
        self.sol
        }
    /// sets the a value (first number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_a(&mut self, a: f64) {
            self.a = a;
            self.sol = 0.0; // reset the solution
        }
    /// sets the b value (second number) in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_b(&mut self, b: f64) {
            self.b = b;
            self.sol = 0.0; // reset the solution
        }
    /// sets the operator in the equation struct.  This will reset the solution to 0 so that it will be recalculated when get_sol() is called.
    pub fn set_operator(&mut self, operator: char) {
            self.operator = operator;
            self.sol = 0.0; // reset the solution
        }
    pub fn get_a(&self) -> f64 {
            self.a
        }
    pub fn get_b(&self) -> f64 {
            self.b
        }
    pub fn get_operator(&self) -> char {
            self.operator
        }
    }
#[cfg(test)]
mod test_equation_f {
        use super::*;

        #[test]
    fn it_works() {
            let mut equation = EquationF::new(1.0, 2.4, '+');
            assert_eq!(equation.get_sol(), 3.4);
        }
    #[test]
    fn set_a() {
            let mut equation = EquationF::new(1.0, 2.0, '+');
            equation.set_a(3.5);
            assert_eq!(equation.get_a(), 3.5);
            assert_eq!(equation.get_sol(), 5.5);
        }
    #[test]
    fn set_b() {
            let mut equation = EquationF::new(1.0, 2.0, '+');
            equation.set_b(3.883);
            assert_eq!(equation.get_b(), 3.883);
            assert_eq!(equation.get_sol(), 4.883);
        }
    #[test]
    fn set_operator() {
            let mut equation = EquationF::new(1.0, 2.0, '+');
            equation.set_operator('-');
            assert_eq!(equation.get_operator(), '-');
            assert_eq!(equation.get_sol(), -1.0);
            equation.set_operator('*');
            assert_eq!(equation.get_operator(), '*');
            assert_eq!(equation.get_sol(), 2.0);
            equation.set_operator('/');
            assert_eq!(equation.get_operator(), '/');
            assert_eq!(equation.get_sol(), 0.5);
            equation.set_operator('%');
            assert_eq!(equation.get_operator(), '%');
            assert_eq!(equation.get_sol(), 1.0);
            equation.set_operator('^');
            assert_eq!(equation.get_operator(), '^');
            assert_eq!(equation.get_sol(), 1.0);
        }
    #[test]
    fn change_equation(){
            let mut equation = EquationF::new(1.0, 2.0, '+');
            assert_eq!(equation.get_sol(), 3.0);
            equation.set_a(3.7);
            println!("a: {}", equation.get_a());
            println!("b: {}", equation.get_b());
            println!("sol: {}", equation.get_sol());
            assert_eq!(equation.get_sol(), 5.7);
            equation.set_b(3.2);
            println!("a: {}", equation.get_a());
            println!("b: {}", equation.get_b());
            assert_eq!(equation.get_sol(), 6.9);
            equation.set_a(14.042134);
            println!("a: {}", equation.get_a());
            println!("b: {}", equation.get_b());
            assert_eq!(equation.get_sol(), 17.242134);
            equation.set_operator('-');
            println!("a: {}", equation.get_a());
            println!("b: {}", equation.get_b());
            assert_eq!(equation.get_sol(), 10.842134000000001);
        }
    }

/// ZeroEquation is an equation with a list of values on one side and a solution on the other.
/// Similar to Equation, but with a list of values (in a Vec) that will add up to the solution
/// (or be subtracted if the number is negative).  Sol is set to 0 and values can be moved to
/// the solution side of the solve with move_to_sol().
///
/// # Example
/// ```
/// use numbers_rus::solve::equation::ZeroEquation;
/// let mut solve = ZeroEquation::new(vec![1, 2, 3]);
/// assert_eq!(solve.get_sol(), 0);
/// ```
pub struct ZeroEquation {
        values: Vec<i128>,  // accepts lists of positive or negative numbers
    sol: i128,  // zero by default, but can be set to any number
    is_valid: bool,  // true if the values add up to the solution
    }

impl ZeroEquation {
        pub fn new(values: Vec<i128>) -> ZeroEquation {
            ZeroEquation {
                values,
                sol: 0,
                is_valid: false,
            }
        }
    /// returns all the values in the equation
    pub fn get_values(&self) -> Vec<i128> {
            self.values.clone()
        }
    /// returns the value at the given index
    pub fn get_value(&self, index: usize) -> i128 {
            self.values[index]
        }
    /// adds a single value to the list of values
    pub fn add_value(&mut self, value: i128) {
            self.values.push(value);
            // check equation after altering to see if it is valid
        self.check_sol();
        }
    /// removes a single value from the list of values
    pub fn remove_value(&mut self, index: usize) {
            self.values.remove(index);
            // check equation after altering to see if it is valid
        self.check_sol();
        }
    /// returns the solution
    pub fn get_sol(&self) -> i128 {
            self.sol
        }
    // moves a value to the solution side of the equation
    pub fn move_to_sol(&mut self, index: usize) -> i128 {
            self.sol += self.values[index];
            self.values.remove(index);
            self.sol
        }
    /// checks if the values add up to the equation's solution
    fn check_sol(&mut self) {
            let mut sum = 0;
            for value in &self.values {
                sum += value;
            }
        if sum == self.sol {
            self.is_valid = true;
        }
        }
    /// returns true if the values add up to the solution
    pub fn is_valid(&mut self) -> bool {
            self.check_sol();
            self.is_valid
        }
    /// returns the difference between the solution and the sum of the values if the values do not add up to the solution
    pub fn get_error(&mut self) -> i128 {
            self.check_sol();
            if self.is_valid {
                0
            } else {
                self.sol - self.values.iter().sum::<i128>()
            }
        }
    }
#[cfg(test)]
mod test_zero_equation {
        use super::*;

        #[test]
    fn it_works() {
            let mut equation = ZeroEquation::new(vec![1, 2, 3]);
            assert_eq!(equation.get_sol(), 0);
            assert_eq!(equation.get_values(), vec![1, 2, 3]);
            assert_eq!(equation.get_value(0), 1);
            assert_eq!(equation.get_value(1), 2);
            assert_eq!(equation.get_value(2), 3);
            assert_eq!(equation.move_to_sol(2), 3);
            assert_eq!(equation.get_sol(), 3);
            assert_eq!(equation.get_values(), vec![1, 2]);
            assert_eq!(equation.get_value(0), 1);
            assert_eq!(equation.get_value(1), 2);
            assert_eq!(equation.is_valid(), true);
            assert_eq!(equation.get_error(), 0);
        }
    #[test]
    fn add_value() {
            let mut equation = ZeroEquation::new(vec![1, 2, 3]);
            equation.add_value(4);
            assert_eq!(equation.get_values(), vec![1, 2, 3, 4]);
        }
    #[test]
    fn remove_value() {
            let mut equation = ZeroEquation::new(vec![1, 2, 3]);
            equation.remove_value(1);
            assert_eq!(equation.get_values(), vec![1, 3]);
        }
    #[test]
    fn get_sol() {
            let equation = ZeroEquation::new(vec![1, 2, 3]);
            assert_eq!(equation.get_sol(), 0);
        }
    #[test]
    fn move_to_sol() {
            let mut equation = ZeroEquation::new(vec![1, 2, 3]);
            assert_eq!(equation.move_to_sol(2), 3);
        }
    #[test]
    fn is_valid() {
            let mut equation = ZeroEquation::new(vec![1, 2, 3]);
            assert_eq!(equation.is_valid(), false);
        }
    #[test]
    fn get_error() {
            let mut equation = ZeroEquation::new(vec![1, 2, 3]);
            assert_eq!(equation.get_error(), -6);
            equation.move_to_sol(2);
            // solve is now 1 + 2 = 3 (valid) so error should be 0
        assert_eq!(equation.get_error(), 0);
            println!("solution: {}", equation.get_sol());
            println!("values: {:?}", equation.get_values());

            equation.add_value(4);
            println!("solution: {}", equation.get_sol());
            println!("values: {:?}", equation.get_values());
            println!("error: {}", equation.is_valid());
        }
    }
