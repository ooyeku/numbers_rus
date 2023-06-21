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
}