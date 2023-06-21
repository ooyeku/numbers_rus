//! # Rational Integer Equation Module
use crate::rational::rational_integer::Rational;

/// A rational integer equation is a structure that holds two rational integers and an operator.
/// The solution is calculated during initialization.  The left and right side of the equation are immutable.
///
/// # Examples
/// ```
/// use numbers_rus::solve::rational_integer_equation::RationalIntegerEquation;
/// use numbers_rus::rational::rational_integer::Rational;
///
/// let left = Rational::new(1, 1);
/// let right = Rational::new(1, 1);
/// let operation = '*';
/// let equation = RationalIntegerEquation::new(left, right, operation);
/// assert_eq!(equation.left, Rational::new(1, 1));
/// assert_eq!(equation.right, Rational::new(1, 1));
/// assert_eq!(equation.sol, Rational::new(1, 1));
/// ```
#[derive(Debug, Clone)]
pub struct RationalIntegerEquation {
    pub left: Rational,
    pub right: Rational,
    pub operator: char,
    pub sol: Rational,
}

impl RationalIntegerEquation {
    pub fn new(left: Rational, right: Rational, operator: char) -> RationalIntegerEquation {
        let sol = match operator {
            '+' => left.add(&right),
            '-' => left.subtract(&right),
            '*' => left.multiply(&right),
            '/' => left.divide(&right),
            _ => panic!("Invalid operation"),
        };
        RationalIntegerEquation {
            left,
            right,
            operator,
            sol,
        }
    }
    /// Returns a reference to the solution of the equation
    pub fn get_sol(&mut self) -> &Rational {
        &self.sol
    }
    /// Returns a reference to the left side of the equation
    pub fn get_left(&mut self) -> &Rational {
        &self.left
    }
    /// Returns a reference to the right side of the equation
    pub fn get_right(&mut self) -> &Rational {
        &self.right
    }
}
#[cfg(test)]
mod test_complex_rational_integer_equation {
    use super::*;

    #[test]
    fn test_new() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.left, Rational::new(1, 2));
        assert_eq!(equation.right, Rational::new(1, 2));
        assert_eq!(equation.sol, Rational::new(4, 4)); // 1/2 + 1/2 = 4/4 = 2/2 = 1
    }

    #[test]
    fn test_get_sol() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(4, 4));
    }

    #[test]
    fn test_get_left() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_left(), &Rational::new(1, 2));
    }

    #[test]
    fn test_get_right() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_right(), &Rational::new(1, 2));
    }

    #[test]
    fn test_add() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(4, 4));
    }

    #[test]
    fn test_subtract() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '-';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(0, 4));
    }

    #[test]
    fn test_multiply() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '*';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(1, 4));
    }

    #[test]
    fn test_divide() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '/';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(2, 2));
    }
}