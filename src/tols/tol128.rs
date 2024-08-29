#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Not, Sub};

use crate::error::ToleranceError::ParseError;
use crate::{error, Myth32, Myth64};

/// # The 128bit tolerance-type
///
/// A 128bit wide type to hold values with a tolerance. Using [Myth64](./struct.Myth64.html) as
/// `value` and [Myth32](./struct.Myth32.html) as `plus` and `minus`. Comes with helper methods to
/// set and show values translated into mm.
///
/// The `Myth`-type stores all values internally in 0.1μ.
///
/// ```rust
/// # use tolerance::T128;
/// let width = T128::new(100.0, 0.05, -0.2);
///
/// assert_eq!(format!("{width}"), "100.00 +0.050/-0.200");
/// assert_eq!(format!("{width:?}"), "T128(100.0000 +0.0500 -0.2000)");
/// ```
///
/// The `plus` and `minus` tolerances are in the same scale unit as the `value`.
/// `plus` is signed positiv (`+`) and `minus` is signed negative (`-`).
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct T128 {
    pub value: Myth64,
    pub plus: Myth32,
    pub minus: Myth32,
}

super::tolerance_body!(T128, Myth64, Myth32);
super::multiply_tolerance!(T128, u64, u32, u16, u8, i64, i32);

#[cfg(test)]
mod should {
    use super::T128;
    use crate::error::ToleranceError;
    use std::convert::TryFrom;

    #[test]
    fn prove_tolerance_is_inside_of() {
        let o = T128::new(2_000, 5, -10);

        assert!(!o.is_inside_of(T128::with_sym(2_000, 5)));
        assert!(o.is_inside_of(T128::with_sym(2_000, 20)));
        assert!(o.is_inside_of(T128::with_sym(2_000, 10)));
        assert!(o.is_inside_of(T128::new(1_995, 10, -5)));
    }

    #[test]
    fn prove_tolerance_is_partial_ord() {
        let o = T128::new(2_000, 5, -10);

        assert!(o < T128::new(2_000, 5, -5));
        assert!(o < T128::new(2_000, 10, -10));
        assert!(o > T128::new(2_000, 2, -10));
        assert!(o > T128::new(2_000, 20, -11));
        assert!(o >= T128::new(2_000, 5, -10));
        assert!(o <= T128::new(2_000, 5, -10));

        let simple: T128 = 30.0.into();
        assert!(simple < 30.01.into());
        assert!(simple > 29.0565.into());
        assert!(simple <= 30.00.into());
        assert!(simple >= 30.0.into());
    }

    #[test]
    fn display_is_adjustible() {
        let o = T128::new(20_000, 50, -100);
        assert_eq!(format!("{o}"), String::from("2.00 +0.005/-0.010"));
        assert_eq!(format!("{o:.3}"), "2.000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.4}"), "2.0000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.0}"), String::from("2 +0.0/-0.0"));
        assert_eq!(format!("{o:.1}"), String::from("2.0 +0.01/-0.01"));

        let o = T128::with_sym(20_000, 50);
        assert_eq!(format!("{o}"), String::from("2.00 +/-0.005"));
        assert_eq!(format!("{o:.0}"), String::from("2 +/-0.0"));

        let o = T128::new(0.345, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("0.345 +0.0100/-0.0140"));
        let o = T128::new(-0.35, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("-0.350 +0.0100/-0.0140"));

        assert_eq!(format!("{o:#}"), String::from("-3500 +100/-140"));

        assert_eq!(
            format!("{o:.3?}"),
            String::from("T128(-0.3500 +0.0100 -0.0140)")
        );
    }

    #[test]
    fn construct_consistent() {
        let o = T128::from((2.0, 0.005, -0.01));
        assert_eq!(o.to_string(), "2.00 +0.005/-0.010".to_string())
    }

    #[test]
    fn substract() {
        let minuend = T128::from((1000.0, 0.0, 0.0));
        let subtrahend = T128::from((300.0, 20.0, -10.0));
        assert_eq!(minuend - subtrahend, (700.0, 10.0, -20.0).into());
        let minuend = T128::from((1000.0, 10.0, -30.0));
        assert_eq!(minuend - subtrahend, (700.0, 20.0, -50.0).into());
    }

    #[test]
    fn invert() {
        let basis = T128::new(20.0, 1.0, -0.5);
        let segment = T128::new(5.0, 0.75, -0.2);
        let res = basis + !segment;
        assert_eq!(res, T128::new(15.0, 1.2, -1.25));
        assert_eq!(basis + basis.invert(), T128::new(0.0, 1.5, -1.5));
    }

    #[test]
    fn error() {
        let tol = T128::try_from("nil");
        assert!(tol.is_err(), "T128 ");
        assert_eq!(
            tol,
            ToleranceError::parse_err("Found ascii #110 (a non-numerical literal) in input, can't parse input into a T128!")
        );

        let tol = T128::try_from("");
        assert!(tol.is_err(), "T128 ");
        assert_eq!(
            tol,
            ToleranceError::parse_err("Cannot parse an empty string into a T128!")
        );
    }

    #[cfg(feature = "serde")]
    mod serde {
        use crate::T128;
        use serde_test::{assert_tokens, Token};

        #[test]
        fn serialize() {
            let m = T128::from(12456.832);

            assert_tokens(
                &m,
                &[
                    Token::Struct {
                        name: "T128",
                        len: 3,
                    },
                    Token::Str("value"),
                    Token::NewtypeStruct { name: "Myth64" },
                    Token::I64(124568320),
                    Token::Str("plus"),
                    Token::NewtypeStruct { name: "Myth32" },
                    Token::I32(0),
                    Token::Str("minus"),
                    Token::NewtypeStruct { name: "Myth32" },
                    Token::I32(0),
                    Token::StructEnd,
                ],
            );
        }
    }
}
