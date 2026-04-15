//! Exact rational arithmetic generic over any signed integer type.
//!
//! [`Rational<T>`] always stores its value in lowest terms with a
//! non-negative denominator. Arithmetic is infallible as long as the
//! intermediate products fit in `T`; if you need unbounded precision, use
//! `Rational<i128>` or bring your own big-integer type.
//!
//! # Examples
//!
//! ```
//! use numbers_rus::rational::Rational;
//!
//! let half = Rational::new(1i64, 2);
//! let third = Rational::new(1i64, 3);
//! assert_eq!(half + third, Rational::new(5, 6));
//! assert_eq!(half * third, Rational::new(1, 6));
//! assert_eq!(format!("{}", half - third), "1/6");
//!
//! // Auto-reduction and sign normalisation.
//! assert_eq!(Rational::new(4i64, 8), Rational::new(1, 2));
//! assert_eq!(Rational::new(1i64, -3), Rational::new(-1, 3));
//! ```

use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Sub};

use num_integer::Integer;
use num_traits::{Signed, ToPrimitive};

/// An exact rational number `numer / denom`, stored in lowest terms with
/// `denom > 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rational<T> {
    numer: T,
    denom: T,
}

impl<T> Rational<T>
where
    T: Integer + Signed + Copy,
{
    /// Construct a rational. Panics if `denom == 0`.
    pub fn new(numer: T, denom: T) -> Self {
        assert!(!denom.is_zero(), "Rational: denominator must be non-zero");
        Self::reduced(numer, denom)
    }

    /// Construct from an integer: `n / 1`.
    pub fn from_integer(n: T) -> Self {
        Self { numer: n, denom: T::one() }
    }

    fn reduced(mut numer: T, mut denom: T) -> Self {
        if denom.is_negative() {
            numer = -numer;
            denom = -denom;
        }
        let g = numer.abs().gcd(&denom.abs());
        if g.is_zero() {
            // Only reachable if numer == 0; denom is already non-zero and positive.
            return Self { numer: T::zero(), denom: T::one() };
        }
        Self {
            numer: numer / g,
            denom: denom / g,
        }
    }

    /// The numerator.
    pub fn numer(&self) -> T {
        self.numer
    }

    /// The denominator. Always positive.
    pub fn denom(&self) -> T {
        self.denom
    }

    /// Zero (`0/1`).
    pub fn zero() -> Self {
        Self {
            numer: T::zero(),
            denom: T::one(),
        }
    }

    /// One (`1/1`).
    pub fn one() -> Self {
        Self {
            numer: T::one(),
            denom: T::one(),
        }
    }

    /// True iff the rational equals an integer (`denom == 1`).
    pub fn is_integer(&self) -> bool {
        self.denom.is_one()
    }

    /// Multiplicative inverse. Panics if `self == 0`.
    pub fn recip(self) -> Self {
        assert!(!self.numer.is_zero(), "Rational::recip: cannot invert zero");
        Self::reduced(self.denom, self.numer)
    }

    /// Absolute value.
    pub fn abs(self) -> Self {
        Self {
            numer: self.numer.abs(),
            denom: self.denom,
        }
    }

    /// Raise to a non-negative integer power. Returns `None` on overflow.
    pub fn pow(self, exp: u32) -> Option<Self>
    where
        T: num_traits::CheckedMul,
    {
        let mut num = T::one();
        let mut den = T::one();
        for _ in 0..exp {
            num = num.checked_mul(&self.numer)?;
            den = den.checked_mul(&self.denom)?;
        }
        Some(Self::reduced(num, den))
    }
}

impl<T> Rational<T>
where
    T: Integer + Signed + Copy + ToPrimitive,
{
    /// Convert to `f64`. Returns `None` if either component cannot be cast.
    pub fn to_f64(&self) -> Option<f64> {
        Some(self.numer.to_f64()? / self.denom.to_f64()?)
    }
}

impl<T> From<T> for Rational<T>
where
    T: Integer + Signed + Copy,
{
    fn from(n: T) -> Self {
        Self::from_integer(n)
    }
}

impl<T> Default for Rational<T>
where
    T: Integer + Signed + Copy,
{
    fn default() -> Self {
        Self::zero()
    }
}

impl<T> fmt::Display for Rational<T>
where
    T: Integer + Signed + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denom.is_one() {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}

impl<T> PartialOrd for Rational<T>
where
    T: Integer + Signed + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Rational<T>
where
    T: Integer + Signed + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Both denominators are positive, so cross-multiplication preserves sign.
        (self.numer * other.denom).cmp(&(other.numer * self.denom))
    }
}

impl<T> Add for Rational<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::reduced(
            self.numer * rhs.denom + rhs.numer * self.denom,
            self.denom * rhs.denom,
        )
    }
}

impl<T> Sub for Rational<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::reduced(
            self.numer * rhs.denom - rhs.numer * self.denom,
            self.denom * rhs.denom,
        )
    }
}

impl<T> Mul for Rational<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::reduced(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl<T> Div for Rational<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(!rhs.numer.is_zero(), "Rational: division by zero");
        Self::reduced(self.numer * rhs.denom, self.denom * rhs.numer)
    }
}

impl<T> Neg for Rational<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            numer: -self.numer,
            denom: self.denom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type R = Rational<i64>;

    #[test]
    fn construction_reduces() {
        assert_eq!(R::new(4, 8), R::new(1, 2));
        assert_eq!(R::new(-4, 8), R::new(-1, 2));
        assert_eq!(R::new(4, -8), R::new(-1, 2));
        assert_eq!(R::new(-4, -8), R::new(1, 2));
        assert_eq!(R::new(0, 5), R::zero());
    }

    #[test]
    fn denom_always_positive() {
        for &(n, d) in &[(1i64, -2), (-3, -4), (5, -1)] {
            assert!(R::new(n, d).denom() > 0);
        }
    }

    #[test]
    fn arithmetic() {
        let half = R::new(1, 2);
        let third = R::new(1, 3);
        assert_eq!(half + third, R::new(5, 6));
        assert_eq!(half - third, R::new(1, 6));
        assert_eq!(half * third, R::new(1, 6));
        assert_eq!(half / third, R::new(3, 2));
        assert_eq!(-half, R::new(-1, 2));
        assert_eq!(half.abs(), half);
        assert_eq!((-half).abs(), half);
    }

    #[test]
    fn ordering() {
        assert!(R::new(1, 3) < R::new(1, 2));
        assert!(R::new(-1, 2) < R::new(1, 2));
        assert_eq!(R::new(2, 4).cmp(&R::new(1, 2)), Ordering::Equal);
    }

    #[test]
    fn display_and_conversion() {
        assert_eq!(format!("{}", R::new(3, 1)), "3");
        assert_eq!(format!("{}", R::new(3, 4)), "3/4");
        assert_eq!(format!("{}", R::new(-3, 4)), "-3/4");
        assert_eq!(R::new(1, 4).to_f64(), Some(0.25));
    }

    #[test]
    fn recip_and_pow() {
        assert_eq!(R::new(2, 3).recip(), R::new(3, 2));
        assert_eq!(R::new(2, 3).pow(3), Some(R::new(8, 27)));
        assert_eq!(R::new(2, 3).pow(0), Some(R::one()));
    }

    #[test]
    #[should_panic(expected = "denominator must be non-zero")]
    fn zero_denominator_panics() {
        let _ = R::new(1, 0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics() {
        let _ = R::new(1, 2) / R::zero();
    }

    #[test]
    fn hash_equality_after_reduction() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        fn h(r: R) -> u64 {
            let mut s = DefaultHasher::new();
            r.hash(&mut s);
            s.finish()
        }
        assert_eq!(h(R::new(2, 4)), h(R::new(1, 2)));
        assert_eq!(h(R::new(-3, 6)), h(R::new(-1, 2)));
        assert_eq!(h(R::new(0, 5)), h(R::zero()));
    }

    #[test]
    fn ordering_with_mixed_signs() {
        assert!(R::new(-3, 5) < R::new(-1, 5));
        assert!(R::new(-1, 1000) < R::zero());
        assert!(R::zero() < R::new(1, 1_000_000));
        // Sorting a small set.
        let mut xs = vec![R::new(1, 2), R::new(-1, 3), R::new(2, 5), R::zero(), R::new(-2, 7)];
        xs.sort();
        assert_eq!(
            xs,
            vec![R::new(-1, 3), R::new(-2, 7), R::zero(), R::new(2, 5), R::new(1, 2)]
        );
    }

    #[test]
    fn from_integer_and_default() {
        assert_eq!(R::from(7), R::from_integer(7));
        assert_eq!(R::default(), R::zero());
        assert_eq!(R::one(), R::new(1, 1));
        assert!(R::from_integer(42).is_integer());
    }

    #[test]
    fn arithmetic_identities() {
        let r = R::new(3, 7);
        assert_eq!(r + R::zero(), r);
        assert_eq!(r * R::one(), r);
        assert_eq!(r - r, R::zero());
        // a / b == a * b.recip()
        let s = R::new(5, 11);
        assert_eq!(r / s, r * s.recip());
        // Multiplication by zero.
        assert_eq!(r * R::zero(), R::zero());
    }

    #[test]
    fn pow_negative_base() {
        let r = R::new(-2, 3);
        assert_eq!(r.pow(2), Some(R::new(4, 9)));
        assert_eq!(r.pow(3), Some(R::new(-8, 27)));
    }

    #[test]
    #[should_panic(expected = "cannot invert zero")]
    fn recip_zero_panics() {
        let _ = R::zero().recip();
    }
}
