use std::fmt;

/// `Complex` is a struct that represents a complex number.
/// It has two fields, `real` and `imag`, which are both `f64` values.
///
/// # Example
///
/// ```
/// use numbers_rus::integers::complex_integers::Complex;
///
/// let a = Complex::new(1, 2);
/// let b = Complex::new(3, 4);
/// println!("{},{}", a,b);
/// ```
///
/// # Methods
///
/// * `add` - adds two complex numbers and returns the result as a new `Complex` instance as the sum of the two inputs.
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
pub struct Complex {
    real: i128,
    imag: i128,
}

impl Default for Complex {
    fn default() -> Self {
        Self { real: 0, imag: 0 }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag < 0 {
            write!(f, "{} - {}i", self.real, -self.imag)
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

impl Complex {
    pub fn new(real: i128, imag: i128) -> Self {
        Self { real, imag }
    }

    /// add function adds two complex numbers and returns the result as a new `Complex` instance as the sum of the two inputs.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `Complex` instance to be added to the current instance.
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::integers::complex_integers::Complex;
    ///
    /// let a = Complex::new(1, 2);
    /// let b = Complex::new(3, 4);
    /// let c = a.add(&b);
    /// assert_eq!(c.get_real(), 4);
    /// assert_eq!(c.get_imag(), 6);
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        let real = self.real + other.real;
        let imag = self.imag + other.imag;
        Self::new(real, imag)
    }

    /// subtract function subtracts two complex numbers and returns the result as a new `Complex` instance as the difference of the two inputs.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `Complex` instance to be subtracted from the current instance.
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::integers::complex_integers::Complex;
    ///
    /// let a = Complex::new(1, 2);
    /// let b = Complex::new(3, 4);
    /// let c = a.subtract(&b);
    /// assert_eq!(c.get_real(), -2);
    /// assert_eq!(c.get_imag(), -2);
    pub fn subtract(&self, other: &Self) -> Self {
        let real = self.real - other.real;
        let imag = self.imag - other.imag;
        Self::new(real, imag)
    }

    /// multiply function multiplies two complex numbers and returns the result as a new `Complex` instance as the product of the two inputs.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `Complex` instance to be multiplied with the current instance.
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::integers::complex_integers::Complex;
    ///
    /// let a = Complex::new(1, 2);
    /// let b = Complex::new(3, 4);
    /// let c = a.multiply(&b);
    /// assert_eq!(c.get_real(), -5);
    /// assert_eq!(c.get_imag(), 10);
    /// ```
    pub fn multiply(&self, other: &Self) -> Self {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Self::new(real, imag)
    }

    /// The modulus function returns the modulus of the complex number.
    /// The modulus is the square root of the sum of the squares of the real and imaginary parts.
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::integers::complex_integers::Complex;
    ///
    /// let a = Complex::new(3, 4);
    /// assert_eq!(a.modulus(), 25.0);
    /// ```
    pub fn modulus(&self) -> f64 {
        (self.real.pow(2) + self.imag.pow(2)) as f64
    }

    /// The divide function divides two complex numbers and returns the result as a new `Complex` instance as the quotient of the two inputs.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `Complex` instance to be divided by the current instance.
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::integers::complex_integers::Complex;
    ///
    /// let a = Complex::new(1, 2);
    /// let b = Complex::new(3, 4);
    /// let c = a.divide(&b);
    /// assert_eq!(c.get_real(), 0);
    /// assert_eq!(c.get_imag(), 0);
    /// ```
    pub fn divide(&self, other: &Self) -> Self {
        let real = (self.real * other.real + self.imag * other.imag) / (other.real.pow(2) + other.imag.pow(2));
        let imag = (self.imag * other.real - self.real * other.imag) / (other.real.pow(2) + other.imag.pow(2));
        Self::new(real, imag)
    }

    /// The conjugate function returns the conjugate of the complex number.  The conjugate is the same as the original complex number except the sign of the imaginary part is changed.
    /// The conjugate of a complex number is the same as the original complex number except the sign of the imaginary part is changed.
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::integers::complex_integers::Complex;
    ///
    /// let a = Complex::new(1, 2);
    /// let b = a.conjugate();
    /// assert_eq!(b.get_real(), 1);
    /// assert_eq!(b.get_imag(), -2);
    /// ```
    pub fn conjugate(&self) -> Self {
        let real = self.real;
        let imag = -self.imag;
        Self::new(real, imag)
    }

    /// The inverse function returns the inverse of the complex number.  The inverse is the reciprocal of the complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use numbers_rus::integers::complex_integers::Complex;
    ///
    /// let a = Complex::new(1, 2);
    /// let b = a.inverse();
    /// assert_eq!(b.get_real(), 0);
    /// assert_eq!(b.get_imag(), 0);
    pub fn inverse(&self) -> Self {
        let real = self.real / (self.real.pow(2) + self.imag.pow(2));
        let imag = -self.imag / (self.real.pow(2) + self.imag.pow(2));
        Self::new(real, imag)
    }

    pub fn display(&self) {
        println!("{} + {}i", self.real, self.imag);
    }

    pub fn get_real(&self) -> i128 {
        self.real
    }

    pub fn get_imag(&self) -> i128 {
        self.imag
    }

    pub fn get(&self) -> (i128, i128) {
        (self.real, self.imag)
    }

    pub fn set_real(&mut self, real: i128) {
        self.real = real;
    }

    pub fn set_imag(&mut self, imag: i128) {
        self.imag = imag;
    }

    pub fn set(&mut self, real: i128, imag: i128) {
        self.real = real;
        self.imag = imag;
    }

    /// The zero function returns a new `Complex` instance with both the real and imaginary parts set to zero.
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    /// The one function returns a new `Complex` instance with the real part set to one and the imaginary part set to zero.
    pub fn one() -> Self {
        Self::new(1, 0)
    }

    pub fn from_real(real: i128) -> Self {
        Self::new(real, 0)
    }

    pub fn from_imag(imag: i128) -> Self {
        Self::new(0, imag)
    }

    /// The copy function returns a new `Complex` instance with the same real and imaginary parts as the current instance.
    pub fn copy(&self) -> Self {
        Self::new(self.real, self.imag)
    }
}
#[cfg(test)]
pub mod test_complex {
    use super::*;
    #[test]
    fn test_add() {
        let a = Complex::new(1, 2);
        let b = Complex::new(3, 4);
        let c = a.add(&b);
        assert_eq!(c.get(), (4, 6));
    }
    #[test]
    fn test_subtract() {
        let a = Complex::new(1, 2);
        let b = Complex::new(3, 4);
        let c = a.subtract(&b);
        assert_eq!(c.get(), (-2, -2));
    }
    #[test]
    fn test_multiply() {
        let a = Complex::new(1, 2);
        let b = Complex::new(3, 4);
        let c = a.multiply(&b);
        assert_eq!(c.get(), (-5, 10));
    }
    #[test]
    fn test_divide() {
        let a = Complex::new(1, 2);
        let b = Complex::new(3, 4);
        let c = a.divide(&b);
        assert_eq!(c.get(), (0, 0));
    }
    #[test]
    fn test_modulus() {
        let a = Complex::new(1, 2);
        let b = a.modulus();
        assert_eq!(b, 5.0);
    }
    #[test]
    fn test_conjugate() {
        let a = Complex::new(1, 2);
        let b = a.conjugate();
        assert_eq!(b.get(), (1, -2));
    }
    #[test]
    fn test_inverse() {
        let a = Complex::new(1, 2);
        let b = a.inverse();
        assert_eq!(b.get(), (0, 0));
    }
    #[test]
    fn test_display() {
        let a = Complex::new(1, 2);
        a.display();
    }
    #[test]
    fn test_get_real() {
        let a = Complex::new(1, 2);
        let b = a.get_real();
        assert_eq!(b, 1);
    }
    #[test]
    fn test_get_imag() {
        let a = Complex::new(1, 2);
        let b = a.get_imag();
        assert_eq!(b, 2);
    }
    #[test]
    fn test_get() {
        let a = Complex::new(1, 2);
        let b = a.get();
        assert_eq!(b, (1, 2));
    }
    #[test]
    fn test_set_real() {
        let mut a = Complex::new(1, 2);
        a.set_real(3);
        assert_eq!(a.get_real(), 3);
    }
    #[test]
    fn test_set_imag() {
        let mut a = Complex::new(1, 2);
        a.set_imag(3);
        assert_eq!(a.get_imag(), 3);
    }
    #[test]
    fn test_set() {
        let mut a = Complex::new(1, 2);
        a.set(3, 4);
        assert_eq!(a.get(), (3, 4));
    }
    #[test]
    fn test_zero() {
        let a = Complex::zero();
        assert_eq!(a.get(), (0, 0));
    }
    #[test]
    fn test_one() {
        let a = Complex::one();
        assert_eq!(a.get(), (1, 0));
    }
    #[test]
    fn test_from_real() {
        let a = Complex::from_real(1);
        assert_eq!(a.get(), (1, 0));
    }
    #[test]
    fn test_from_imag() {
        let a = Complex::from_imag(1);
        assert_eq!(a.get(), (0, 1));
    }
    #[test]
    fn test_copy() {
        let a = Complex::new(1, 2);
        let b = a.copy();
        assert_eq!(b.get(), (1, 2));
    }
}