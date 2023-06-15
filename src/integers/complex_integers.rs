use std::fmt;

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
        (self.real.pow(2) + self.imag.pow(2)) as f64
    }

    pub fn divide(&self, other: &Self) -> Self {
        let real = (self.real * other.real + self.imag * other.imag) / (other.real.pow(2) + other.imag.pow(2));
        let imag = (self.imag * other.real - self.real * other.imag) / (other.real.pow(2) + other.imag.pow(2));
        Self::new(real, imag)
    }

    pub fn conjugate(&self) -> Self {
        let real = self.real;
        let imag = -self.imag;
        Self::new(real, imag)
    }

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

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn one() -> Self {
        Self::new(1, 0)
    }

    pub fn from_real(real: i128) -> Self {
        Self::new(real, 0)
    }

    pub fn from_imag(imag: i128) -> Self {
        Self::new(0, imag)
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