//! This module contains the rational number implementation.
//! The rational number implementation is based on the 'Rational' struct with a numerator and a denominator.
//!
//! # Examples
//! ```
//! use numbers_rus::rational::rational::Rational;
//!
//! let a = Rational::new(1, 2);
//! let numerator = a.get_numerator();
//! let denominator = a.get_denominator();
//! assert_eq!(numerator, 1);
//! assert_eq!(denominator, 2);
//! ```
pub mod rational;