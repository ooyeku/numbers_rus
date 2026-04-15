//! Generic complex numbers `a + bi`.
//!
//! [`Complex<T>`] works over any type implementing [`num_traits::Num`] and
//! `Copy`. For floating-point components you also get `norm`, `arg`, and
//! `to_polar`.
//!
//! # Examples
//!
//! ```
//! use numbers_rus::complex::Complex;
//!
//! let a = Complex::new(1.0, 2.0);
//! let b = Complex::new(3.0, -4.0);
//! assert_eq!(a + b, Complex::new(4.0, -2.0));
//! assert_eq!(a * b, Complex::new(11.0, 2.0));
//! assert_eq!(Complex::new(0.0, 1.0) * Complex::new(0.0, 1.0), Complex::new(-1.0, 0.0));
//! assert!((a.norm() - (5.0f64).sqrt()).abs() < 1e-12);
//! ```

use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::{Float, Num, One, Zero};

/// A complex number `re + im·i`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Complex<T> {
    /// Real part.
    pub re: T,
    /// Imaginary part.
    pub im: T,
}

impl<T> Complex<T> {
    /// Construct from real and imaginary parts.
    pub const fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

impl<T: Num + Copy> Complex<T> {
    /// `0 + 0i`.
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    /// `1 + 0i`.
    pub fn one() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// The imaginary unit `i`.
    pub fn i() -> Self {
        Self::new(T::zero(), T::one())
    }

    /// From a real value: `r + 0i`.
    pub fn from_real(r: T) -> Self {
        Self::new(r, T::zero())
    }

    /// The squared norm `re² + im²`. No square root required, so this stays
    /// in `T` and never overflows into the float domain.
    pub fn norm_sqr(self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T: Num + Copy + Neg<Output = T>> Complex<T> {
    /// The complex conjugate `re − im·i`.
    pub fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }
}

impl<T: Float> Complex<T> {
    /// The modulus `|z| = √(re² + im²)`.
    pub fn norm(self) -> T {
        self.norm_sqr().sqrt()
    }

    /// The principal argument `atan2(im, re)` in `(-π, π]`.
    pub fn arg(self) -> T {
        self.im.atan2(self.re)
    }

    /// Polar form `(r, θ)` with `self = r·(cos θ + i sin θ)`.
    pub fn to_polar(self) -> (T, T) {
        (self.norm(), self.arg())
    }

    /// Construct from polar form.
    pub fn from_polar(r: T, theta: T) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }
}

impl<T: Num + Copy> From<T> for Complex<T> {
    fn from(r: T) -> Self {
        Self::from_real(r)
    }
}

impl<T: Num + Copy> Default for Complex<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Num + Copy> Zero for Complex<T> {
    fn zero() -> Self {
        Complex::zero()
    }
    fn is_zero(&self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
}

impl<T: Num + Copy> One for Complex<T> {
    fn one() -> Self {
        Complex::one()
    }
}

impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display + Zero + PartialOrd + Neg<Output = T> + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im.partial_cmp(&T::zero()) == Some(Ordering::Less) {
            write!(f, "{} - {}i", self.re, -self.im)
        } else {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }
}

impl<T: Num + Copy> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T: Num + Copy> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T: Num + Copy> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T: Num + Copy> Div for Complex<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self::new(
            (self.re * rhs.re + self.im * rhs.im) / denom,
            (self.im * rhs.re - self.re * rhs.im) / denom,
        )
    }
}

impl<T: Num + Copy + Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type C = Complex<f64>;

    #[test]
    fn arithmetic() {
        let a = C::new(1.0, 2.0);
        let b = C::new(3.0, -4.0);
        assert_eq!(a + b, C::new(4.0, -2.0));
        assert_eq!(a - b, C::new(-2.0, 6.0));
        assert_eq!(a * b, C::new(11.0, 2.0));
        let q = a / b;
        assert!((q.re - (-0.2)).abs() < 1e-12);
        assert!((q.im - 0.4).abs() < 1e-12);
    }

    #[test]
    fn conjugate_and_norm() {
        let z = C::new(3.0, -4.0);
        assert_eq!(z.conj(), C::new(3.0, 4.0));
        assert_eq!(z.norm_sqr(), 25.0);
        assert!((z.norm() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn i_squared_is_minus_one() {
        let i = C::i();
        assert_eq!(i * i, C::new(-1.0, 0.0));
    }

    #[test]
    fn polar_roundtrip() {
        let z = C::new(1.0, 1.0);
        let (r, theta) = z.to_polar();
        let back = C::from_polar(r, theta);
        assert!((back.re - z.re).abs() < 1e-12);
        assert!((back.im - z.im).abs() < 1e-12);
    }

    #[test]
    fn integer_complex() {
        let a: Complex<i64> = Complex::new(2, 3);
        let b: Complex<i64> = Complex::new(1, -1);
        assert_eq!(a + b, Complex::new(3, 2));
        assert_eq!(a * b, Complex::new(5, 1));
        assert_eq!(a.norm_sqr(), 13);
    }

    #[test]
    fn display_handles_negative_imaginary() {
        assert_eq!(format!("{}", C::new(1.0, -2.0)), "1 - 2i");
        assert_eq!(format!("{}", C::new(1.0, 2.0)), "1 + 2i");
    }
}
