//! Solve module contains structures that hold both values and operations to be automatically solved.
//! Module contains:
//! * Base Equation struct
//! * Equation struct for Complex numbers
//! * Equation struct for Complex integers
//! * Equation struct for Rational numbers
//! * Equation struct for Rational integers
//!
//! Structures in the solve module are used to solve equations automatically.  The equation structs
//! hold the left and right side of the equation, the operation, and the solution.  The solution is
//! calculated during initialization.  The left and right side of the equation are immutable.
//!
//! # Examples
//! ```
//! use numbers_rus::solve::*;
//! ```
//!
pub mod complex_float_equations;
pub mod complex_integer_equations;
pub mod equation;
pub mod rational_float_equation;
pub mod rational_integer_equation;
