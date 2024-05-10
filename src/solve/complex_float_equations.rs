//! # Complex Float Equations Module
use crate::numbers::complex_floats::Complex;

/// Equation struct for Complex numbers.  Solves the equation during initialization, and stores the solution
/// as a Complex float.  The left and right sides of the equation are stored as Complex numbers and the operation
/// is stored as a char.
///
/// # Examples
/// ```
/// use numbers_rus::solve::complex_float_equations::Equation;
/// use numbers_rus::numbers::complex_floats::Complex;
///
/// let left = Complex::new(1.0, 1.0);
/// let right = Complex::new(1.0, 1.0);
/// let operation = '*';
/// let equation = Equation::new(left, right, operation);
/// println!("{:?}", equation);
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    left: Complex,
    right: Complex,
    operation: char,
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
    /// Returns a reference to the operator of the equation
    pub fn get_operation(&mut self) -> &char {
        &self.operation
    }
    /// Sets the left side of the equation
    /// # Examples
    /// ```
    /// use numbers_rus::solve::complex_float_equations::Equation;
    /// use numbers_rus::numbers::complex_floats::Complex;
    ///
    /// let left = Complex::new(1.0, 1.0);
    /// let right = Complex::new(1.0, 1.0);
    /// let operation = '*';
    /// let mut equation = Equation::new(left, right, operation);
    /// let new_left = Complex::new(2.0, 2.0);
    /// equation.set_left(new_left);
    /// ```
    pub fn set_left(&mut self, left: Complex) {
        self.left = left;
        // Recalculate solution
        self.sol = match self.operation {
            '+' => self.left.add(&self.right),
            '-' => self.left.subtract(&self.right),
            '*' => self.left.multiply(&self.right),
            '/' => self.left.divide(&self.right),
            _ => panic!("Invalid operation"),
        };
    }
    /// Sets the right side of the equation
    /// # Examples
    /// ```
    /// use numbers_rus::solve::complex_float_equations::Equation;
    /// use numbers_rus::numbers::complex_floats::Complex;
    ///
    /// let left = Complex::new(1.0, 1.0);
    /// let right = Complex::new(1.0, 1.0);
    /// let operation = '*';
    /// let mut equation = Equation::new(left, right, operation);
    /// let new_right = Complex::new(2.0, 2.0);
    /// equation.set_right(new_right);
    /// ```
    pub fn set_right(&mut self, right: Complex) {
        self.right = right;
        // Recalculate solution
        self.sol = match self.operation {
            '+' => self.left.add(&self.right),
            '-' => self.left.subtract(&self.right),
            '*' => self.left.multiply(&self.right),
            '/' => self.left.divide(&self.right),
            _ => panic!("Invalid operation"),
        };
    }
    /// Sets the operator of the equation
    /// # Examples
    /// ```
    /// use std::ptr::eq;
    /// use numbers_rus::solve::complex_float_equations::Equation;
    /// use numbers_rus::numbers::complex_floats::Complex;
    ///
    /// let left = Complex::new(1.0, 1.0);
    /// let right = Complex::new(1.0, 1.0);
    /// let operation = '*';
    /// let mut equation = Equation::new(left, right, operation);
    /// equation.set_operation('+');
    /// ```
    pub fn set_operation(&mut self, operation: char) {
        self.operation = operation;
        // Recalculate solution
        self.sol = match self.operation {
            '+' => self.left.add(&self.right),
            '-' => self.left.subtract(&self.right),
            '*' => self.left.multiply(&self.right),
            '/' => self.left.divide(&self.right),
            _ => panic!("Invalid operation"),
        };
    }
}
#[cfg(test)]
mod test_equation {
    use super::*;

    #[test]
    fn test_new() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let equation = Equation::new(left, right, operation);
        assert_eq!(equation.left, Complex::new(1.0, 1.0));
        assert_eq!(equation.right, Complex::new(1.0, 1.0));
        assert_eq!(equation.sol, Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_get_sol() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_get_left() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_left(), &Complex::new(1.0, 1.0));
    }

    #[test]
    fn test_get_right() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_right(), &Complex::new(1.0, 1.0));
    }

    #[test]
    #[should_panic]
    fn test_invalid_operation() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = 'a';
        Equation::new(left, right, operation);
    }

    #[test]
    fn test_add() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_subtract() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '-';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(1.0, 1.0));
    }

    #[test]
    fn test_multiply() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(2.0, 2.0);
        let operation = '*';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(0.0, 8.0));
    }

    #[test]
    fn test_divide() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(2.0, 2.0);
        let operation = '/';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(1.0, 0.0));
    }

    #[test]
    fn test_add_complex() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        assert_eq!(left.add(&right), Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_subtract_complex() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(1.0, 1.0);
        assert_eq!(left.subtract(&right), Complex::new(1.0, 1.0));
    }

    #[test]
    fn test_set_operation() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(2.0, 2.0);
        let mut equation = Equation::new(left, right, '+');
        // println!("{:?}", equation.get_sol());
        equation.set_operation('-');
        assert_eq!(equation.get_sol(), &Complex::new(0.0, 0.0));
        // println!("{:?}", equation.get_sol());
    }
}
