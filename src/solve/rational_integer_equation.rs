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
/// println!("{:?}", equation);
/// ```
#[derive(Debug, Clone)]
pub struct RationalIntegerEquation {
    left: Rational,
    right: Rational,
    operator: char,
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
    /// Returns a reference to the operator of the equation
    pub fn get_operator(&mut self) -> &char {
        &self.operator
    }
    /// Sets the operator of the equation and recalculates the solution
    /// # Examples
    /// ```
    /// use numbers_rus::solve::rational_integer_equation::RationalIntegerEquation;
    /// use numbers_rus::rational::rational_integer::Rational;
    ///
    /// let left = Rational::new(1, 1);
    /// let right = Rational::new(1, 1);
    /// let operation = '*';
    /// let mut equation = RationalIntegerEquation::new(left, right, operation);
    /// equation.set_operator('+');
    pub fn set_operator(&mut self, operator: char) {
        self.operator = operator;
        self.sol = match operator {
            '+' => self.left.add(&self.right),
            '-' => self.left.subtract(&self.right),
            '*' => self.left.multiply(&self.right),
            '/' => self.left.divide(&self.right),
            _ => panic!("Invalid operation"),
        };
    }
    /// Sets the left side of the equation and recalculates the solution
    /// # Examples
    /// ```
    /// use numbers_rus::solve::rational_integer_equation::RationalIntegerEquation;
    /// use numbers_rus::rational::rational_integer::Rational;
    /// let left = Rational::new(1, 1);
    /// let right = Rational::new(1, 1);
    /// let operation = '*';
    /// let mut equation = RationalIntegerEquation::new(left, right, operation);
    /// equation.set_left(Rational::new(2, 1));
    /// ```
    pub fn set_left(&mut self, left: Rational) {
        self.left = left;
        self.sol = match self.operator {
            '+' => self.left.add(&self.right),
            '-' => self.left.subtract(&self.right),
            '*' => self.left.multiply(&self.right),
            '/' => self.left.divide(&self.right),
            _ => panic!("Invalid operation"),
        };
    }
    /// Sets the right side of the equation and recalculates the solution
    /// # Examples
    /// ```
    /// use numbers_rus::solve::rational_integer_equation::RationalIntegerEquation;
    /// use numbers_rus::rational::rational_integer::Rational;
    /// let left = Rational::new(1, 1);
    /// let right = Rational::new(1, 1);
    /// let operation = '*';
    /// let mut equation = RationalIntegerEquation::new(left, right, operation);
    /// equation.set_right(Rational::new(2, 1));
    /// ```
    pub fn set_right(&mut self, right: Rational) {
        self.right = right;
        self.sol = match self.operator {
            '+' => self.left.add(&self.right),
            '-' => self.left.subtract(&self.right),
            '*' => self.left.multiply(&self.right),
            '/' => self.left.divide(&self.right),
            _ => panic!("Invalid operation"),
        };
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

    #[test]
    #[should_panic]
    fn test_invalid_operation() {
        let left = Rational::new(1, 2);
        let right = Rational::new(1, 2);
        let operator = '%';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(2, 2));
    }

    #[test]
    fn test_add_neg() {
        let left = Rational::new(1, 2);
        let right = Rational::new(-1, 2);
        let operator = '+';
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(0, 4));
    }

    #[test]
    fn test_subtract_neg() {
        let left = Rational::new(1, 2);
        let right = Rational::new(-1, 2);
        let operator = '-'; // subtracting a negative is the same as adding a positive
        let mut equation = RationalIntegerEquation::new(left, right, operator);
        assert_eq!(equation.get_sol(), &Rational::new(4, 4));
    }

    #[test]
    fn test_set_operator() {
        let left = Rational::new(1, 2);
        let right = Rational::new(-1, 2);
        let mut equation = RationalIntegerEquation::new(left, right, '+');
        assert_eq!(equation.get_sol(), &Rational::new(0, 4));
        // println!("{:?}", equation.get_sol());
        equation.set_operator('-'); // operator changed to subtract
        assert_eq!(equation.get_sol(), &Rational::new(4, 4)); // solution doesn't change - should
                                                              // println!("{:?}", equation.get_sol());
    }
}
