use std::fmt;

pub struct Complex {
    real: f64,
    imag: f64,
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

    pub fn add(&self, other: &Self) -> Self {
        let real = self.real + other.real;
        let imag = self.imag + other.imag;
        Self::new(real, imag)
    }

    pub fn subtract(&self, other: &Self) -> Self {
        let real = self.real - other.real;
        let imag = self.imag - other.imag;
        Self::new(real, imag)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Self::new(real, imag)
    }

    pub fn modulus(&self) -> f64 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }

    pub fn divide(&self, other: &Self) -> Self {
        let real = (self.real * other.real + self.imag * other.imag) / (other.real.powi(2) + other.imag.powi(2));
        let imag = (self.imag * other.real - self.real * other.imag) / (other.real.powi(2) + other.imag.powi(2));
        Self::new(real, imag)
    }

    pub fn conjugate(&self) -> Self {
        let real = self.real;
        let imag = -self.imag;
        Self::new(real, imag)
    }

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

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn from_real(real: f64) -> Self {
        Self::new(real, 0.0)
    }

    pub fn from_imag(imag: f64) -> Self {
        Self::new(0.0, imag)
    }

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
