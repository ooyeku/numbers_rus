//! Rational numbers are numbers that can be in the form p/q where p and q are integers and q is not equal to zero.
//!
//! # Examples
//!
//! ```
//! use numbers_rus::rational::rational_integer::Rational;
//!
//! let a = Rational::new(1, 2);
//! ```

use crate::integers::complex_integers;
use crate::numbers::complex_floats;

/// A rational number is a number that can be in the form p/q where p and q are integers and q is not equal to zero.
///
/// # Examples
///
/// ```
/// use numbers_rus::rational::rational_integer::Rational;
///
/// let a = Rational::new(1, 2);
/// ```
///
/// # Methods
///
/// * `new(numerator: i32, denominator: i32) -> Self`
/// * `add(&self, other: &Self) -> Self`
/// * `subtract(&self, other: &Self) -> Self`
/// * `multiply(&self, other: &Self) -> Self`
/// * `divide(&self, other: &Self) -> Self`
/// * `display(&self)`
/// * `get_numerator(&self) -> i32`
/// * `get_denominator(&self) -> i32`
/// * `simplify(&self) -> Self`
/// * `to_float(&self) -> f64`
/// * `to_complex_float(&self) -> complex_floats::Complex`
/// * `to_complex_int(&self) -> complex_integers::Complex`
/// * `to_string(&self) -> String`
/// * `to_string_complex(&self) -> String`
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rational {
    numerator: f64,
    denominator: f64,
}

impl Rational {
    pub fn new(numerator: f64, denominator: f64) -> Self {
        assert_ne!(denominator, 0.0, "Denominator must not be zero!");
        Self {
            numerator,
            denominator,
        }
    }

    /// add two rational numbers
    ///
    /// # Arguments
    ///
    /// * `other` - another rational number
    ///
    /// # Examples
    ///
    /// ```
    /// use numbers_rus::rational::rational_integer::Rational;
    ///
    /// let a = Rational::new(1, 2);
    /// let b = Rational::new(1, 3);
    /// let c = a.add(&b);
    /// assert_eq!(c.get_numerator(), 5);
    /// assert_eq!(c.get_denominator(), 6);
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Self::new(numerator, denominator)
    }

    /// subtract two rational numbers
    ///
    /// # Arguments
    ///
    /// * `other` - another rational number
    ///
    /// # Examples
    ///
    /// ```
    /// use numbers_rus::rational::rational_integer::Rational;
    ///
    /// let a = Rational::new(1, 2);
    /// let b = Rational::new(1, 3);
    /// let c = a.subtract(&b);
    /// assert_eq!(c.get_numerator(), 1);
    /// assert_eq!(c.get_denominator(), 6);
    /// ```
    pub fn subtract(&self, other: &Self) -> Self {
        let numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Self::new(numerator, denominator)
    }

    /// multiply two rational numbers
    ///
    /// # Arguments
    ///
    /// * `other` - another rational number
    ///
    /// # Examples
    ///
    /// ```
    /// use numbers_rus::rational::rational_integer::Rational;
    ///
    /// let a = Rational::new(1, 2);
    /// let b = Rational::new(1, 3);
    /// let c = a.multiply(&b);
    /// assert_eq!(c.get_numerator(), 1);
    /// assert_eq!(c.get_denominator(), 6);
    /// ```
    pub fn multiply(&self, other: &Self) -> Self {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        Self::new(numerator, denominator)
    }

    /// divide two rational numbers
    ///
    /// # Arguments
    ///
    /// * `other` - another rational number
    ///
    /// # Examples
    ///
    /// ```
    /// use numbers_rus::rational::rational_integer::Rational;
    ///
    /// let a = Rational::new(1, 2);
    /// let b = Rational::new(1, 3);
    /// let c = a.divide(&b);
    /// assert_eq!(c.get_numerator(), 3);
    /// assert_eq!(c.get_denominator(), 2);
    /// ```
    pub fn divide(&self, other: &Self) -> Self {
        let numerator = self.numerator * other.denominator;
        let denominator = self.denominator * other.numerator;
        Self::new(numerator, denominator)
    }

    pub fn display(&self) {
        println!("{}/{}", self.numerator, self.denominator);
    }

    pub fn get_numerator(&self) -> f64 {
        self.numerator
    }

    pub fn get_denominator(&self) -> f64 {
        self.denominator
    }

    /// simplify by dividing numerator and denominator by their greatest common divisor.  If the gcd is 1, then the rational number is already in simplest form.
    ///
    /// # Examples
    ///
    /// ```
    /// use numbers_rus::rational::rational_integer::Rational;
    ///
    /// let a = Rational::new(2, 4);
    /// let b = a.simplify();
    /// assert_eq!(b.get_numerator(), 1);
    /// assert_eq!(b.get_denominator(), 2);
    /// ```
    pub fn simplify(&self) -> Self {
        let mut numerator = self.numerator;
        let mut denominator = self.denominator;
        let mut gcd = 1.0;
        for i in 1..=numerator.abs().min(denominator.abs()) as i32 {
            if numerator % i as f64 == 0.0 && denominator % i as f64 == 0.0 {
                gcd = i as f64;
            }
        }
        numerator /= gcd;
        denominator /= gcd;
        Self::new(numerator, denominator)
    }

    pub fn to_float(&self) -> i128 {
        self.numerator as i128 / self.denominator as i128
    }

    /// convert to complex number with real and imaginary parts as numbers
    pub fn to_complex_float(&self) -> complex_floats::Complex {
        complex_floats::Complex::new(self.numerator as f64, self.denominator as f64)
    }

    /// convert to complex number with real and imaginary parts as integers
    pub fn to_complex_int(&self) -> complex_integers::Complex {
        complex_integers::Complex::new(self.numerator as i128, self.denominator as i128)
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }

    pub fn to_string_complex(&self) -> String {
        format!("{}/{}i", self.numerator, self.denominator)
    }
}
#[cfg(test)]
pub mod test_rational {

    use super::*;
    #[test]
    fn test_add() {
        let a = Rational::new(1.0, 2.0);
        let b = Rational::new(1.0, 3.0);
        let c = a.add(&b);
        assert_eq!(c.get_numerator(), 5.0);
        assert_eq!(c.get_denominator(), 6.0);
    }
    #[test]
    fn test_subtract() {
        let a = Rational::new(1.0, 2.0);
        let b = Rational::new(1.0, 3.0);
        let c = a.subtract(&b);
        assert_eq!(c.get_numerator(), 1.0);
        assert_eq!(c.get_denominator(), 6.0);
    }
    #[test]
    fn test_multiply() {
        let a = Rational::new(1.0, 2.0);
        let b = Rational::new(1.0, 3.0);
        let c = a.multiply(&b);
        assert_eq!(c.get_numerator(), 1.0);
        assert_eq!(c.get_denominator(), 6.0);
    }
    #[test]
    fn test_divide() {
        let a = Rational::new(1.0, 2.0);
        let b = Rational::new(1.0, 3.0);
        let c = a.divide(&b);
        assert_eq!(c.get_numerator(), 3.0);
        assert_eq!(c.get_denominator(), 2.0);
    }
    #[test]
    fn test_simplify() {
        let a = Rational::new(2.0, 4.0);
        let b = a.simplify();
        assert_eq!(b.get_numerator(), 1.0);
        assert_eq!(b.get_denominator(), 2.0);
    }
    #[test]
    fn test_to_float() {
        let a = Rational::new(1.0, 2.0);
        let b = a.to_float();
        assert_eq!(b, 0);
    }
    #[test]
    fn test_to_complex_float() {
        let a = Rational::new(1.0, 2.0);
        let b = a.to_complex_float();
        b.display();
        println!("{}, {}", b.get_real(), b.get_imag());
        println!("{}, {}", b.get_real(), b.get_imag());
        assert_eq!(b.get_real(), 1.0);
        assert_eq!(b.get_imag(), 2.0);
    }
    #[test]
    fn test_to_complex_int() {
        let a = Rational::new(1.0, 2.0);
        let b = a.to_complex_int();
        b.display();
        println!("{}, {}", b.get_real(), b.get_imag());
        println!("{}, {}", b.get_real(), b.get_imag());
        assert_eq!(b.get_real(), 1 as i128);
        assert_eq!(b.get_imag(), 2 as i128);
    }
    #[test]
    fn test_to_string() {
        let a = Rational::new(1.0, 2.0);
        let b = a.to_string();
        a.display();
        assert_eq!(b, "1/2");
    }
    #[test]
    fn test_to_string_complex() {
        let a = Rational::new(1.0, 2.0);
        let b = a.to_string_complex();
        a.display();
        assert_eq!(b, "1/2i");
    }
    #[test]
    fn test_new() {
        let a = Rational::new(1.0, 2.0);
        assert_eq!(a.get_numerator(), 1.0);
        assert_eq!(a.get_denominator(), 2.0);
    }
}
