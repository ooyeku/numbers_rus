use crate::integers::complex_integers::Complex;


pub struct Equation {
    pub left: Complex,
    pub right: Complex,
    pub operation: char,
    pub sol: Complex,
}

// Complex Equation implementation solves the equation during initialization
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

    pub fn get_sol(&mut self) -> &Complex {
        &self.sol
    }

    pub fn get_left(&mut self) -> &Complex {
        &self.left
    }

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
}
