//! Symbolic equation objects.
//!
//! [`Equation<T>`] holds a pair of operands and a typed [`Op`], caches the
//! solution in an `Option<T>`, and returns it from [`Equation::solve`]. The
//! pre-1.0 API used `0` as a "not computed" sentinel, which silently hid the
//! case where zero was the real answer — that's gone.
//!
//! [`Polynomial<T>`] lives in the [`polynomial`] submodule and handles
//! single-variable polynomials with Horner evaluation and differentiation.
//!
//! # Examples
//!
//! ```
//! use numbers_rus::equation::{Equation, Op};
//!
//! let mut eq = Equation::new(12i64, 5, Op::Sub);
//! assert_eq!(eq.solve(), 7);
//!
//! // Mutating any part invalidates the cached solution.
//! eq.set_b(12);
//! assert_eq!(eq.solve(), 0); // zero is a real answer, not "not yet computed"
//! ```

pub mod polynomial;

pub use polynomial::Polynomial;

use core::fmt;
use num_traits::Num;

/// Binary operator for [`Equation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Op {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Remainder.
    Rem,
}

impl Op {
    /// The conventional single-character symbol for this operator.
    pub fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
            Op::Rem => '%',
        }
    }

    /// Parse an operator character. Accepts `+ - * / %`.
    pub fn from_symbol(c: char) -> Option<Self> {
        Some(match c {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            '%' => Op::Rem,
            _ => return None,
        })
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// A binary equation `a OP b`, with a lazily-cached solution.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Equation<T> {
    a: T,
    b: T,
    op: Op,
    sol: Option<T>,
}

impl<T: Num + Copy> Equation<T> {
    /// Create a new equation. The solution is not computed until [`solve`]
    /// is called.
    ///
    /// [`solve`]: Equation::solve
    pub fn new(a: T, b: T, op: Op) -> Self {
        Self { a, b, op, sol: None }
    }

    /// The left operand.
    pub fn a(&self) -> T {
        self.a
    }

    /// The right operand.
    pub fn b(&self) -> T {
        self.b
    }

    /// The operator.
    pub fn op(&self) -> Op {
        self.op
    }

    /// Set the left operand and invalidate the cached solution.
    pub fn set_a(&mut self, a: T) {
        self.a = a;
        self.sol = None;
    }

    /// Set the right operand and invalidate the cached solution.
    pub fn set_b(&mut self, b: T) {
        self.b = b;
        self.sol = None;
    }

    /// Set the operator and invalidate the cached solution.
    pub fn set_op(&mut self, op: Op) {
        self.op = op;
        self.sol = None;
    }

    /// Evaluate (or return the cached value of) `a OP b`.
    pub fn solve(&mut self) -> T {
        if let Some(s) = self.sol {
            return s;
        }
        let s = match self.op {
            Op::Add => self.a + self.b,
            Op::Sub => self.a - self.b,
            Op::Mul => self.a * self.b,
            Op::Div => self.a / self.b,
            Op::Rem => self.a % self.b,
        };
        self.sol = Some(s);
        s
    }

    /// The cached solution, without forcing computation.
    pub fn cached(&self) -> Option<T> {
        self.sol
    }
}

impl<T> fmt::Display for Equation<T>
where
    T: fmt::Display + Num + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.a, self.op, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_and_caches() {
        let mut e = Equation::new(2i64, 3, Op::Add);
        assert_eq!(e.cached(), None);
        assert_eq!(e.solve(), 5);
        assert_eq!(e.cached(), Some(5));
    }

    #[test]
    fn zero_is_a_real_answer() {
        // Regression for the pre-1.0 sentinel bug.
        let mut e = Equation::new(3i64, 3, Op::Sub);
        assert_eq!(e.solve(), 0);
        assert_eq!(e.cached(), Some(0));
        // Mutating invalidates the cache.
        e.set_b(2);
        assert_eq!(e.cached(), None);
        assert_eq!(e.solve(), 1);
    }

    #[test]
    fn every_operator() {
        assert_eq!(Equation::new(10i64, 3, Op::Add).solve(), 13);
        assert_eq!(Equation::new(10i64, 3, Op::Sub).solve(), 7);
        assert_eq!(Equation::new(10i64, 3, Op::Mul).solve(), 30);
        assert_eq!(Equation::new(10i64, 3, Op::Div).solve(), 3);
        assert_eq!(Equation::new(10i64, 3, Op::Rem).solve(), 1);
    }

    #[test]
    fn works_with_floats() {
        let mut e = Equation::new(1.5f64, 2.5, Op::Mul);
        assert!((e.solve() - 3.75).abs() < 1e-12);
    }

    #[test]
    fn op_symbol_roundtrip() {
        for op in [Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Rem] {
            assert_eq!(Op::from_symbol(op.symbol()), Some(op));
        }
        assert_eq!(Op::from_symbol('?'), None);
    }

    #[test]
    fn display_equation() {
        let mut e = Equation::new(4i64, 5, Op::Mul);
        let sol = e.solve();
        assert_eq!(format!("{} = {}", e, sol), "4 * 5 = 20");
    }
}
