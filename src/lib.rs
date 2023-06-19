//!
//! # Tolerance
//!
//! Math representation of the physically needed permissible deviation of measures in rust
//! avoiding floating point inaccuracy. Allows to calc with allowances aka tolerances in a
//! consistent way.
//!
//! Based on `Myth`-types with a accuracy of 1/10th my-meter (= 0.1μ) as the name suggests.
//!
//! ### Exaxmple
//! ```rust
//! use tolerance::T128;
//!
//! let width1 = T128::new(100.0, 0.05, -0.2);
//! let width2 = T128::with_sym(50.0, 0.05);
//!
//! // Adding two `T128`s is strait-forth.
//! assert_eq!(width1 + width2, T128::new(150.0, 0.1, -0.25));
//!
//! // `!` inverts the direction of tolerance to /subtract/ measures.
//! assert_eq!(!width1, T128::new(-100.0, 0.2, -0.05));
//!
//! // Adding an inverted `T128` wides the tolerance.
//! assert_eq!(width1 + !width1, T128::new(0.0, 0.25, -0.25));
//! ```
extern crate core;

pub mod error;
mod myth16;
mod myth32;
mod myth64;
mod tol128;
mod tol64;
mod unit;

pub use self::myth16::*;
pub use self::myth32::*;
pub use self::myth64::*;
pub use self::tol128::*;
pub use self::tol64::*;
pub use self::unit::*;
