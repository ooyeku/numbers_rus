//! # Rational Float Equation Module
use crate::rational::rational_float::Rational;

/// Equation struct for Rational floats.  Solves the equation during initialization, and stores the solution
/// as a Rational float.  The left and right sides of the equation are stored as Rational floats and the operation
/// is stored as a char.
///
/// # Examples
/// ```
/// use numbers_rus::rational::rational_float::Rational;
/// use numbers_rus::solve::rational_float_equation::RationalFloatEquation;
///
/// let left = Rational::new(1.0, 2.0);
/// let right = Rational::new(1.0, 2.0);
/// let operator = '+';
/// let equation = RationalFloatEquation::new(left, right, operator);
/// assert_eq!(equation.left, Rational::new(1.0, 2.0));
/// assert_eq!(equation.right, Rational::new(1.0, 2.0));
/// assert_eq!(equation.sol, Rational::new(4.0, 4.0)); // 1/2 + 1/2 = 4/4 = 2/2 = 1
/// ```
#[derive(Debug, Clone)]
pub struct RationalFloatEquation {
    pub left: Rational,
    pub right: Rational,
    pub operator: char,
    pub sol: Rational,
}

impl RationalFloatEquation {
    pub fn new(left: Rational, right: Rational, operator: char) -> RationalFloatEquation {
        let sol = match operator {
            '+' => left.add(&right),
            '-' => left.subtract(&right),
            '*' => left.multiply(&right),
            '/' => left.divide(&right),
            _ => panic!("Invalid operation"),
        };
        RationalFloatEquation {
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
mod test_complex_rational_float_equation {
    use super::*;

    #[test]
    fn test_new() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let equation = RationalFloatEquation::new(left, right, operator);
        assert_eq!(equation.left, Rational::new(1.0, 2.0));
        assert_eq!(equation.right, Rational::new(1.0, 2.0));
        assert_eq!(equation.sol, Rational::new(4.0, 4.0)); // 1/2 + 1/2 = 4/4 = 2/2 = 1
    }

    #[test]
    fn test_get_sol() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let mut equation = RationalFloatEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(4.0, 4.0));
    }

    #[test]
    fn test_get_left() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let mut equation = RationalFloatEquation::new(left, right, operator);
        assert_eq!(equation.get_left(), &Rational::new(1.0, 2.0));
    }

    #[test]
    fn test_get_right() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let mut equation = RationalFloatEquation::new(left, right, operator);
        assert_eq!(equation.get_right(), &Rational::new(1.0, 2.0));
    }

    #[test]
    fn test_get_operator() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let equation = RationalFloatEquation::new(left, right, operator);
        assert_eq!(equation.operator, '+');
    }

    #[test]
    fn test_set_sol() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let mut equation = RationalFloatEquation::new(left, right, operator);
        equation.sol = Rational::new(1.0, 2.0);
        assert_eq!(equation.sol, Rational::new(1.0, 2.0));
    }

    #[test]
    fn test_set_left() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let mut equation = RationalFloatEquation::new(left, right, operator);
        equation.left = Rational::new(1.0, 2.0);
        assert_eq!(equation.left, Rational::new(1.0, 2.0));
    }

    #[test]
    fn test_set_right() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let mut equation = RationalFloatEquation::new(left, right, operator);
        equation.right = Rational::new(1.0, 2.0);
        assert_eq!(equation.right, Rational::new(1.0, 2.0));
    }

    #[test]
    fn test_set_operator() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let mut equation = RationalFloatEquation::new(left, right, '+');
        equation.operator = '-';
        assert_eq!(equation.operator, '-');
    }

    #[test]
    #[should_panic]
    fn test_invalid_operator() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = 'a';
        RationalFloatEquation::new(left, right, operator);
    }
}