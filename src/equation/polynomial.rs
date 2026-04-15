//! Single-variable polynomials `Σ cᵢ · xⁱ`.
//!
//! Coefficients are stored **low-to-high**: `coefficients[0]` is the constant
//! term and `coefficients[degree()]` is the leading coefficient. The
//! representation is trimmed on construction, so the leading coefficient is
//! never an unintentional zero.
//!
//! # Examples
//!
//! ```
//! use numbers_rus::equation::Polynomial;
//!
//! // 1 - x² (coefficients in ascending degree order).
//! let p = Polynomial::new(vec![1i64, 0, -1]);
//! assert_eq!(p.degree(), 2);
//! assert_eq!(p.eval(3), -8);
//!
//! // Addition.
//! let q = Polynomial::new(vec![0i64, 1]); // x
//! assert_eq!((p + q).coefficients(), &[1, 1, -1]);
//! ```

use core::fmt;
use core::ops::{Add, Mul, Neg, Sub};

use num_traits::{FromPrimitive, Num};

/// A polynomial in one variable over `T`, stored low-to-high.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polynomial<T> {
    coeffs: Vec<T>,
}

impl<T: Num + Copy> Polynomial<T> {
    /// Construct from a vector of coefficients, low-degree first.
    ///
    /// Trailing zero coefficients are trimmed. The zero polynomial is
    /// represented as `Polynomial::new(vec![])` or `Polynomial::zero()`.
    pub fn new(coeffs: Vec<T>) -> Self {
        let mut p = Self { coeffs };
        p.trim();
        p
    }

    /// Construct a constant polynomial.
    pub fn constant(c: T) -> Self {
        Self::new(vec![c])
    }

    /// The zero polynomial.
    pub fn zero() -> Self {
        Self { coeffs: Vec::new() }
    }

    /// True iff this is the zero polynomial.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    fn trim(&mut self) {
        while self.coeffs.last().is_some_and(|c| c.is_zero()) {
            self.coeffs.pop();
        }
    }

    /// Coefficients in ascending degree order.
    pub fn coefficients(&self) -> &[T] {
        &self.coeffs
    }

    /// Degree. The zero polynomial returns `0` by convention.
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }

    /// Leading coefficient, or `T::zero()` for the zero polynomial.
    pub fn leading(&self) -> T {
        self.coeffs.last().copied().unwrap_or_else(T::zero)
    }

    /// Evaluate at `x` using Horner's method.
    pub fn eval(&self, x: T) -> T {
        let mut acc = T::zero();
        for &c in self.coeffs.iter().rev() {
            acc = acc * x + c;
        }
        acc
    }

    /// Formal derivative.
    pub fn derivative(&self) -> Self
    where
        T: FromPrimitive,
    {
        if self.coeffs.len() <= 1 {
            return Self::zero();
        }
        let derived = self
            .coeffs
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| c * T::from_usize(i).expect("degree fits in T"))
            .collect();
        Self::new(derived)
    }
}

impl<T: Num + Copy> Add for Polynomial<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let n = self.coeffs.len().max(rhs.coeffs.len());
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let a = self.coeffs.get(i).copied().unwrap_or_else(T::zero);
            let b = rhs.coeffs.get(i).copied().unwrap_or_else(T::zero);
            out.push(a + b);
        }
        Self::new(out)
    }
}

impl<T: Num + Copy> Sub for Polynomial<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let n = self.coeffs.len().max(rhs.coeffs.len());
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let a = self.coeffs.get(i).copied().unwrap_or_else(T::zero);
            let b = rhs.coeffs.get(i).copied().unwrap_or_else(T::zero);
            out.push(a - b);
        }
        Self::new(out)
    }
}

impl<T: Num + Copy> Mul for Polynomial<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let n = self.coeffs.len() + rhs.coeffs.len() - 1;
        let mut out = vec![T::zero(); n];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in rhs.coeffs.iter().enumerate() {
                out[i + j] = out[i + j] + a * b;
            }
        }
        Self::new(out)
    }
}

impl<T: Num + Copy + Neg<Output = T>> Neg for Polynomial<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(self.coeffs.into_iter().map(|c| -c).collect())
    }
}

impl<T> fmt::Display for Polynomial<T>
where
    T: fmt::Display + Num + Copy + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let mut first = true;
        for (i, &c) in self.coeffs.iter().enumerate().rev() {
            if c.is_zero() {
                continue;
            }
            if first {
                write!(f, "{}", format_term(c, i))?;
                first = false;
            } else if c >= T::zero() {
                write!(f, " + {}", format_term(c, i))?;
            } else {
                write!(f, " - {}", format_term(T::zero() - c, i))?;
            }
        }
        Ok(())
    }
}

fn format_term<T: fmt::Display + Num + Copy>(c: T, degree: usize) -> String {
    match degree {
        0 => format!("{}", c),
        1 if c.is_one() => "x".to_string(),
        1 => format!("{}x", c),
        _ if c.is_one() => format!("x^{}", degree),
        _ => format!("{}x^{}", c, degree),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type P = Polynomial<i64>;

    #[test]
    fn construction_trims() {
        let p = P::new(vec![1, 2, 0, 0]);
        assert_eq!(p.coefficients(), &[1, 2]);
        assert_eq!(p.degree(), 1);
    }

    #[test]
    fn zero_polynomial() {
        let p = P::new(vec![0, 0, 0]);
        assert!(p.is_zero());
        assert_eq!(p.degree(), 0);
        assert_eq!(p.leading(), 0);
    }

    #[test]
    fn horner_evaluation() {
        // p(x) = 1 - x^2
        let p = P::new(vec![1, 0, -1]);
        assert_eq!(p.eval(0), 1);
        assert_eq!(p.eval(1), 0);
        assert_eq!(p.eval(2), -3);
        assert_eq!(p.eval(-3), -8);
    }

    #[test]
    fn arithmetic() {
        // p = 1 + x, q = 1 - x
        let p = P::new(vec![1, 1]);
        let q = P::new(vec![1, -1]);
        assert_eq!((p.clone() + q.clone()).coefficients(), &[2]);
        assert_eq!((p.clone() - q.clone()).coefficients(), &[0, 2]);
        // (1 + x)(1 - x) = 1 - x^2
        assert_eq!((p.clone() * q.clone()).coefficients(), &[1, 0, -1]);
        assert_eq!((-p).coefficients(), &[-1, -1]);
    }

    #[test]
    fn derivative() {
        // p(x) = 3x^3 - 2x + 5 → p'(x) = 9x^2 - 2
        let p = P::new(vec![5, -2, 0, 3]);
        assert_eq!(p.derivative().coefficients(), &[-2, 0, 9]);
        let constant = P::constant(42);
        assert!(constant.derivative().is_zero());
    }

    #[test]
    fn display_polynomial() {
        let p = P::new(vec![5, -2, 0, 3]); // 3x^3 - 2x + 5
        assert_eq!(format!("{}", p), "3x^3 - 2x + 5");
        assert_eq!(format!("{}", P::zero()), "0");
        assert_eq!(format!("{}", P::new(vec![0, 1])), "x");
        assert_eq!(format!("{}", P::new(vec![0, 0, 1])), "x^2");
    }
}
