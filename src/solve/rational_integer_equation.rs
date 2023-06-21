// complex rational int equations
use crate::rational::rational_integer::Rational;

#[derive(Debug, Clone)]
pub struct ComplexRationalIntegerEquation {
    pub left: Rational,
    pub right: Rational,
    pub operator: char,
    pub sol: Rational,
}

impl ComplexRationalIntegerEquation {
    pub fn new(left: Rational, right: Rational, operator: char) -> ComplexRationalIntegerEquation {
        let sol = match operator {
            '+' => left.add(&right),
            '-' => left.subtract(&right),
            '*' => left.multiply(&right),
            '/' => left.divide(&right),
            _ => panic!("Invalid operation"),
        };
        ComplexRationalIntegerEquation {
            left,
            right,
            operator,
            sol,
        }
    }

    pub fn get_sol(&mut self) -> &Rational {
        &self.sol
    }

    pub fn get_left(&mut self) -> &Rational {
        &self.left
    }

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
        let equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.left, Rational::new(1, 2));
        assert_eq!(equation.right, Rational::new(1, 2));
        assert_eq!(equation.sol, Rational::new(4, 4)); // 1/2 + 1/2 = 4/4 = 2/2 = 1
    }

    #[test]
    fn test_get_sol() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(4, 4));
    }

    #[test]
    fn test_get_left() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_left(), &Rational::new(1, 2));
    }

    #[test]
    fn test_get_right() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_right(), &Rational::new(1, 2));
    }

    #[test]
    fn test_add() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '+';
        let mut equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(4, 4));
    }

    #[test]
    fn test_subtract() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '-';
        let mut equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(0, 4));
    }

    #[test]
    fn test_multiply() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '*';
        let mut equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(1, 4));
    }

    #[test]
    fn test_divide() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '/';
        let mut equation = ComplexRationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(2, 2));
    }
}