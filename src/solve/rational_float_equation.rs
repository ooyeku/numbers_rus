// complex rational float equations
use crate::rational::rational_float::Rational;

#[derive(Debug, Clone)]
pub struct ComplexRationalFloatEquation {
    pub left: Rational,
    pub right: Rational,
    pub operator: char,
    pub sol: Rational,
}

impl ComplexRationalFloatEquation {
    pub fn new(left: Rational, right: Rational, operator: char) -> ComplexRationalFloatEquation {
        let sol = match operator {
            '+' => left.add(&right),
            '-' => left.subtract(&right),
            '*' => left.multiply(&right),
            '/' => left.divide(&right),
            _ => panic!("Invalid operation"),
        };
        ComplexRationalFloatEquation {
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
mod test_complex_rational_float_equation {
    use super::*;

    #[test]
    fn test_new() {
        let left = Rational::new(1.0, 2.0);
        let right = Rational::new(1.0, 2.0);
        let operator = '+';
        let equation = ComplexRationalFloatEquation::new(left, right, operator);
        assert_eq!(equation.left, Rational::new(1.0, 2.0));
        assert_eq!(equation.right, Rational::new(1.0, 2.0));
        assert_eq!(equation.sol, Rational::new(4.0, 4.0)); // 1/2 + 1/2 = 4/4 = 2/2 = 1
    }
}