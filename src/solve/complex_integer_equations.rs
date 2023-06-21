//! # Complex Integer Equation Module
use crate::integers::complex_integers::Complex;

/// Equation struct for complex integers that holds the left and right side of the equation, the operation, and the solution.
/// The solution is calculated during initialization.  The left and right side of the equation are immutable.
///
/// # Examples
/// ```
/// use numbers_rus::solve::complex_integer_equations::Equation;
/// use numbers_rus::integers::complex_integers::Complex;
///
/// let left = Complex::new(1, 1);
/// let right = Complex::new(1, 1);
/// let operation = '*';
/// let equation = Equation::new(left, right, operation);
/// assert_eq!(equation.left, Complex::new(1, 1));
/// assert_eq!(equation.right, Complex::new(1, 1));
/// assert_eq!(equation.sol, Complex::new(0, 2));
/// ```
pub struct Equation {
    pub left: Complex,
    pub right: Complex,
    pub operation: char,
    pub sol: Complex,
}

impl Equation {
    pub fn new(left: Complex, right: Complex, operation: char) -> Equation {
        let sol = match operation {
            '+' => left.add(&right),
            '-' => left.subtract(&right),
            '*' => left.multiply(&right),
            '/' => left.divide(&right),
            _ => panic!("Invalid operation"),
        };
        Equation {
            left,
            right,
            operation,
            sol,
        }
    }
    /// Returns a reference to the solution of the equation
    pub fn get_sol(&mut self) -> &Complex {
        &self.sol
    }
    /// Returns a reference to the left side of the equation
    pub fn get_left(&mut self) -> &Complex {
        &self.left
    }
    /// Returns a reference to the right side of the equation
    pub fn get_right(&mut self) -> &Complex {
        &self.right
    }
}
#[cfg(test)]
mod test_equation {
    use super::*;

    #[test]
    fn test_new() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '+';
        let equation = Equation::new(left, right, operation);
        assert_eq!(equation.left, Complex::new(1, 1));
        assert_eq!(equation.right, Complex::new(1, 1));
        assert_eq!(equation.sol, Complex::new(2, 2));
    }

    #[test]
    fn test_get_sol() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(2, 2));
    }

    #[test]
    fn test_get_left() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_left(), &Complex::new(1, 1));
    }

    #[test]
    fn test_get_right() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_right(), &Complex::new(1, 1));
    }

    #[test]
    fn test_full() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_left(), &Complex::new(1, 1));
        assert_eq!(equation.get_right(), &Complex::new(1, 1));
        assert_eq!(equation.get_sol(), &Complex::new(2, 2));
    }

    #[test]
    #[should_panic]
    fn test_invalid_operation() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = 'a';
        let equation = Equation::new(left, right, operation);
        assert_eq!(equation.left, Complex::new(1, 1));
    }

    #[test]
    fn test_add() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(2, 2));
    }

    #[test]
    fn test_subtract() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '-';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(0, 0));
    }

    #[test]
    fn test_multiply() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '*';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(0, 2));
    }

    #[test]
    fn test_divide() {
        let left = Complex::new(1, 1);
        let right = Complex::new(1, 1);
        let operation = '/';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(1, 0));
    }
}
