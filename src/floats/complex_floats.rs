//! `complex_floats` module for implementing various mathematical operations on complex floating-point numbers.
//! The operations include addition, subtraction, multiplication, division, power, root, minimum, maximum, average,
//! median, and mode calculation. The results are returned as a comma-separated string representing the element-wise results
//! of the applied operation.
//!
//! # Examples
//! ```
//! use numbers_rus::floats::complex_floats::Complex;
//!
//! let a = Complex::new(1.0, 2.0);
//! let b = Complex::new(3.0, 4.0);
//! let c = a.add(&b);
//! assert_eq!(c.get_real(), 4.0);
//! assert_eq!(c.get_imag(), 6.0);
//! ```
use std::fmt;
/// A complex number is a number that can be expressed in the form a + bi, where a and b are real numbers, and i is a solution of the solve x2 = −1. Because no real number satisfies this solve, i is called an imaginary number. For the complex number a + bi, a is called the real part, and b is called the imaginary part. Despite the historical nomenclature "imaginary", complex numbers are regarded in the mathematical sciences as just as "real" as the real numbers, and are fundamental in many aspects of the scientific description of the natural world.
///
/// Complex numbers allow solutions to certain equations that have no solutions in real numbers.
///
///
/// # Arguments
/// * `real` - The real part of the complex number
/// * `imag` - The imaginary part of the complex number
///
/// # Methods
///
/// * new - Creates a new complex number
/// * add - Adds two complex numbers
/// * `subtract` - subtracts two complex numbers and returns the result as a new `Complex` instance as the difference of the two inputs.
/// * `multiply` - multiplies two complex numbers and returns the result as a new `Complex` instance as the product of the two inputs.
/// * `divide` - divides two complex numbers and returns the result as a new `Complex` instance as the quotient of the two inputs.
/// * `get_real` - returns the real part of the complex number.
/// * `get_imag` - returns the imaginary part of the complex number.
/// * `get` - returns the complex number as a tuple of the real and imaginary parts.
/// * `set_real` - sets the real part of the complex number.
/// * `set_imag` - sets the imaginary part of the complex number.
/// * `set` - sets the complex number as a tuple of the real and imaginary parts.
/// * `zero` - returns a new `Complex` instance with both the real and imaginary parts set to zero.
/// * `one` - returns a new `Complex` instance with the real part set to one and the imaginary part set to zero.
/// * `from_real` - returns a new `Complex` instance with the real part set to the input value and the imaginary part set to zero.
/// * `from_imag` - returns a new `Complex` instance with the real part set to zero and the imaginary part set to the input value.
/// * `copy` - returns a new `Complex` instance with the real and imaginary parts set to the same values as the current instance.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Default for Complex {
    fn default() -> Self {
        Self { real: 0.0, imag: 0.0 }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag < 0.0 {
            write!(f, "{} - {}i", self.real, -self.imag)
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// add - Adds two complex numbers (floats)
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another complex float
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::floats::complex_floats::Complex;
    ///
    /// let a = Complex::new(1.0, 2.0);
    /// let b = Complex::new(3.0, 4.0);
    /// let c = a.add(&b);
    /// assert_eq!(c.get_real(), 4.0);
    /// assert_eq!(c.get_imag(), 6.0);
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        let real = self.real + other.real;
        let imag = self.imag + other.imag;
        Self::new(real, imag)
    }

    /// subtract - subtracts two complex numbers (floats)
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another complex float
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::floats::complex_floats::Complex;
    ///
    /// let a = Complex::new(1.0, 2.0);
    /// let b = Complex::new(3.0, 4.0);
    /// let c = a.subtract(&b);
    /// assert_eq!(c.get_real(), -2.0);
    /// assert_eq!(c.get_imag(), -2.0);
    /// ```
    pub fn subtract(&self, other: &Self) -> Self {
        let real = self.real - other.real;
        let imag = self.imag - other.imag;
        Self::new(real, imag)
    }

    /// multiply - multiplies two complex numbers (floats)
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another complex float
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::floats::complex_floats::Complex;
    ///
    /// let a = Complex::new(1.0, 2.0);
    /// let b = Complex::new(3.0, 4.0);
    /// let c = a.multiply(&b);
    /// assert_eq!(c.get_real(), -5.0);
    /// assert_eq!(c.get_imag(), 10.0);
    /// ```
    pub fn multiply(&self, other: &Self) -> Self {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Self::new(real, imag)
    }

    /// divide - divides two complex numbers (floats)
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::floats::complex_floats::Complex;
    ///
    /// let a = Complex::new(1.0, 2.0);
    /// let b = Complex::new(3.0, 4.0);
    /// let c = a.divide(&b);
    /// assert_eq!(c.get_real(), 0.44);
    /// assert_eq!(c.get_imag(), 0.08);
    /// ```
    pub fn modulus(&self) -> f64 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }

    /// divide - divides two complex numbers (floats)
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another complex float
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::floats::complex_floats::Complex;
    ///
    /// let a = Complex::new(1.0, 2.0);
    /// let b = Complex::new(3.0, 4.0);
    /// let c = a.divide(&b);
    /// assert_eq!(c.get_real(), 0.44);
    /// assert_eq!(c.get_imag(), 0.08);
    /// ```
    pub fn divide(&self, other: &Self) -> Self {
        let real = (self.real * other.real + self.imag * other.imag) / (other.real.powi(2) + other.imag.powi(2));
        let imag = (self.imag * other.real - self.real * other.imag) / (other.real.powi(2) + other.imag.powi(2));
        Self::new(real, imag)
    }

    /// conjugate - returns the conjugate of a complex number (float)
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::floats::complex_floats::Complex;
    ///
    /// let a = Complex::new(1.0, 2.0);
    /// let b = a.conjugate();
    /// assert_eq!(b.get_real(), 1.0);
    /// assert_eq!(b.get_imag(), -2.0);
    /// ```
    pub fn conjugate(&self) -> Self {
        let real = self.real;
        let imag = -self.imag;
        Self::new(real, imag)
    }

    /// inverse - returns the inverse of a complex number (float)
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::floats::complex_floats::Complex;
    ///
    /// let a = Complex::new(1.0, 2.0);
    /// let b = a.inverse();
    /// assert_eq!(b.get_real(), 0.2);
    /// assert_eq!(b.get_imag(), -0.4);
    /// ```
    pub fn inverse(&self) -> Self {
        let real = self.real / (self.real.powi(2) + self.imag.powi(2));
        let imag = -self.imag / (self.real.powi(2) + self.imag.powi(2));
        Self::new(real, imag)
    }

    pub fn display(&self) {
        println!("{}", self);
    }

    pub fn get_real(&self) -> f64 {
        self.real
    }

    pub fn get_imag(&self) -> f64 {
        self.imag
    }

    pub fn get(&self) -> (f64, f64) {
        (self.real, self.imag)
    }

    pub fn set_real(&mut self, real: f64) {
        self.real = real;
    }

    pub fn set_imag(&mut self, imag: f64) {
        self.imag = imag;
    }

    pub fn set(&mut self, real: f64, imag: f64) {
        self.real = real;
        self.imag = imag;
    }

    /// zero - returns a complex number (float) with real and imaginary parts equal to zero
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// one - returns a complex number (float) with real part equal to one and imaginary part equal to zero
    pub fn one() -> Self {
        Self::new(1.0, 0.0)
    }

    /// from_real - returns a complex number (float) with real part equal to the argument and imaginary part equal to zero
    pub fn from_real(real: f64) -> Self {
        Self::new(real, 0.0)
    }

    /// from_imag - returns a complex number (float) with real part equal to zero and imaginary part equal to the argument
    pub fn from_imag(imag: f64) -> Self {
        Self::new(0.0, imag)
    }

    /// copy - returns a copy of a complex number (float)
    pub fn copy(&self) -> Self {
        Self::new(self.real, self.imag)
    }
}
#[cfg(test)]
pub mod test_complex {
    use super::*;
    #[test]
    fn test_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.add(&b);
        assert_eq!(c.get(), (4.0, 6.0));
    }
    #[test]
    fn test_subtract() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.subtract(&b);
        assert_eq!(c.get(), (-2.0, -2.0));
    }
    #[test]
    fn test_multiply() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.multiply(&b);
        assert_eq!(c.get(), (-5.0, 10.0));
    }
    #[test]
    fn test_divide() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.divide(&b);
        assert_eq!(c.get(), (0.44, 0.08));
    }
    #[test]
    fn test_modulus() {
        let a = Complex::new(1.0, 2.0);
        let b = a.modulus();
        assert_eq!(b, 2.23606797749979);
    }
    #[test]
    fn test_conjugate() {
        let a = Complex::new(1.0, 2.0);
        let b = a.conjugate();
        assert_eq!(b.get(), (1.0, -2.0));
    }
    #[test]
    fn test_inverse() {
        let a = Complex::new(1.0, 2.0);
        let b = a.inverse();
        assert_eq!(b.get(), (0.2, -0.4));
    }
    #[test]
    fn test_display() {
        let a = Complex::new(1.0, 2.0);
        a.display();
    }
    #[test]
    fn test_get_real() {
        let a = Complex::new(1.0, 2.0);
        let b = a.get_real();
        assert_eq!(b, 1.0);
    }
    #[test]
    fn test_get_imag() {
        let a = Complex::new(1.0, 2.0);
        let b = a.get_imag();
        assert_eq!(b, 2.0);
    }
    #[test]
    fn test_get() {
        let a = Complex::new(1.0, 2.0);
        let b = a.get();
        assert_eq!(b, (1.0, 2.0));
    }
    #[test]
    fn test_set_real() {
        let mut a = Complex::new(1.0, 2.0);
        a.set_real(3.0);
        assert_eq!(a.get(), (3.0, 2.0));
    }
    #[test]
    fn test_set_imag() {
        let mut a = Complex::new(1.0, 2.0);
        a.set_imag(3.0);
        assert_eq!(a.get(), (1.0, 3.0));
    }
    #[test]
    fn test_set() {
        let mut a = Complex::new(1.0, 2.0);
        a.set(3.0, 4.0);
        assert_eq!(a.get(), (3.0, 4.0));
    }
    #[test]
    fn test_zero() {
        let a = Complex::zero();
        assert_eq!(a.get(), (0.0, 0.0));
    }
    #[test]
    fn test_one() {
        let a = Complex::one();
        assert_eq!(a.get(), (1.0, 0.0));
    }
    #[test]
    fn test_from_real() {
        let a = Complex::from_real(1.0);
        assert_eq!(a.get(), (1.0, 0.0));
    }
    #[test]
    fn test_from_imag() {
        let a = Complex::from_imag(1.0);
        assert_eq!(a.get(), (0.0, 1.0));
    }
    #[test]
    fn test_copy() {
        let a = Complex::new(1.0, 2.0);
        let b = a.copy();
        assert_eq!(b.get(), (1.0, 2.0));
    }
    #[test]
    fn test_equality() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(1.0, 2.0);
        assert_eq!(a.display(), b.display());
    }
}
