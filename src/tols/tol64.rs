#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Not, Sub};

use crate::error::ToleranceError::ParseError;
use crate::{error, Myth16, Myth32};

/// # The 64bit tolerance-type
///
/// A 64bit wide type to hold values with a tolerance. Using [Myth32](./struct.Myth32.html) as
/// `value` and [Myth16](./struct.Myth16.html) as `plus` and `minus`. Comes with helper methods to
/// set and show values translated into mm.
///
/// The `Myth`-type stores all values internally in 0.1μ.
///
/// ```rust
/// # use tolerance::T64;
/// let width = T64::new(100.0, 0.05, -0.2);
///
/// assert_eq!(format!("{width}"), "100.00 +0.050/-0.200");
/// assert_eq!(format!("{width:?}"), "T64(100.0000 +0.0500 -0.2000)");
/// ```
///
/// The `plus` and `minus` tolerances are in the same scale unit as the `value`.
/// `plus` is signed positiv (`+`) and `minus` is signed negative (`-`).
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct T64 {
    pub value: Myth32,
    pub plus: Myth16,
    pub minus: Myth16,
}

super::tolerance_body!(T64, Myth32, Myth16);
super::multiply_tolerance!(T64, u64, u32, u16, u8, i64, i32);

#[cfg(test)]
mod should {
    use super::T64;
    use crate::error::ToleranceError;
    use std::convert::TryFrom;

    #[test]
    fn try_from_tuples() {
        let t64 = T64::try_from((Some(4.0), Some(32.0), Some(23.0))).unwrap();
        assert_eq!(T64::new(4.0, 32.0, 23.0), t64);

        let t64 = T64::try_from((Some(&400000), Some(&320), None)).unwrap();
        assert_eq!(T64::new(40.0, 320, -320), t64);
    }

    #[test]
    fn prove_tolerance_is_inside_of() {
        let o = T64::new(2_000, 5, -10);

        assert!(!o.is_inside_of(T64::with_sym(2_000, 5)));
        assert!(o.is_inside_of(T64::with_sym(2_000, 20)));
        assert!(o.is_inside_of(T64::with_sym(2_000, 10)));
        assert!(o.is_inside_of(T64::new(1_995, 10, -5)));
    }

    #[test]
    fn prove_tolerance_is_partial_ord() {
        let o = T64::new(2_000, 5, -10);

        assert!(o < T64::new(2_000, 5, -5));
        assert!(o < T64::new(2_000, 10, -10));
        assert!(o > T64::new(2_000, 2, -10));
        assert!(o > T64::new(2_000, 20, -11));
        assert!(o >= T64::new(2_000, 5, -10));
        assert!(o <= T64::new(2_000, 5, -10));

        let simple: T64 = 30.0.into();
        assert!(simple < 30.01.into());
        assert!(simple > 29.0565.into());
        assert!(simple <= 30.00.into());
        assert!(simple >= 30.0.into());
    }

    #[test]
    fn display_is_adjustible() {
        let o = T64::new(20_000, 50, -100);
        assert_eq!(format!("{o}"), String::from("2.00 +0.005/-0.010"));
        assert_eq!(format!("{o:.3}"), "2.000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.4}"), "2.0000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.0}"), String::from("2 +0.0/-0.0"));
        assert_eq!(format!("{o:.1}"), String::from("2.0 +0.01/-0.01"));

        let o = T64::with_sym(20_000, 50);
        assert_eq!(format!("{o}"), String::from("2.00 +/-0.005"));
        assert_eq!(format!("{o:.0}"), String::from("2 +/-0.0"));

        let o = T64::new(0.345, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("0.345 +0.0100/-0.0140"));
        let o = T64::new(-0.35, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("-0.350 +0.0100/-0.0140"));

        assert_eq!(format!("{o:#}"), String::from("-3500 +100/-140"));

        assert_eq!(
            format!("{o:.3?}"),
            String::from("T64(-0.3500 +0.0100 -0.0140)")
        );
    }

    #[test]
    fn construct_consistent() {
        let o = T64::from((2.0, 0.005, -0.01));
        assert_eq!(o.to_string(), "2.00 +0.005/-0.010".to_string())
    }

    #[test]
    fn substract() {
        let minuend = T64::from((1000.0, 0.0, 0.0));
        let subtrahend = T64::from((300.0, 0.2, -0.1));
        assert_eq!(minuend - subtrahend, (700.0, 0.1, -0.2).into());
        let minuend = T64::from((1000.0, 0.1, -0.3));
        assert_eq!(minuend - subtrahend, (700.0, 0.20, -0.50).into());
    }

    #[test]
    fn invert() {
        let basis = T64::new(20.0, 1.0, -0.5);
        let segment = T64::new(5.0, 0.75, -0.2);
        let res = basis + !segment;
        assert_eq!(res, T64::new(15.0, 1.2, -1.25));
        assert_eq!(basis + basis.invert(), T64::new(0.0, 1.5, -1.5));
    }

    #[test]
    fn error() {
        use ToleranceError::ParseError;
        let a = T64::try_from("nil");
        assert!(a.is_err(), "T64 ");
        assert_eq!(
            a.unwrap_err(),
            ParseError(String::from(
                "Found ascii #110 (a non-numerical literal) in input, can't parse input into a T64!"
            ))
        );

        let a = T64::try_from("");
        assert!(a.is_err(), "T64 ");
        assert_eq!(
            a.unwrap_err(),
            ParseError(String::from("Cannot parse an empty string into a T64!"))
        );
    }
}
