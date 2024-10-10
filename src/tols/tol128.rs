#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Not, Sub, SubAssign};
use std::str::FromStr;

use crate::error::ToleranceError::ParseError;
use crate::{error, Myth32, Myth64};

/// # 128bit tolerance-type
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
/// `plus` is signed positive (`+`) and `minus` is signed negative (`-`).
///
/// ### Ways to create a T128
///
/// ```rust
/// # use tolerance::T128;
/// # use std::str::FromStr;
///
/// // 12.6 +0.40/-1.00
/// assert_eq!("12 .4 -1".parse(), Ok(T128::new(12.0, 0.4, -1.0)));
/// assert_eq!("12/.4/-1".parse(), Ok(T128::new(12.0, 0.4, -1.0)));
/// assert_eq!("12;0.4; -1".parse(), Ok(T128::new(12.0, 0.4, -1.0)));
/// // 12 +/-0.4
/// assert_eq!(T128::try_from("12.0 0.4"), Ok(T128::with_sym(12.0, 0.4)));
/// assert_eq!(T128::from_str("12.0 +-0.4"), Ok(T128::new(12.0, 0.4, -0.4)));
/// // 12.0 +/- 0
/// assert_eq!(T128::try_from("12.0"), Ok(T128::from(12.0)));
/// ```
///
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
    use crate::{error::ToleranceError, Myth32, Myth64};
    use pretty_assertions::assert_eq;
    use std::convert::TryFrom;

    #[test]
    fn convert_from_string() {
        assert_eq!(T128::try_from("14.0").unwrap(), T128::new(140_000, 0, 0));
        assert_eq!(T128::try_from("14").unwrap(), T128::new(140_000, 0, 0));
        assert_eq!(T128::try_from("14 0 0").unwrap(), T128::new(140_000, 0, 0));
        assert_eq!(
            T128::try_from("14.0 +1 -2").unwrap(),
            T128::new(140_000, 10_000, -20_000)
        );
        assert_eq!(
            T128::try_from("14.0 2 ").unwrap(),
            T128::new(140_000, 20_000, -20_000)
        );
        assert_eq!(
            T128::try_from("14.0 +-2 ").unwrap(),
            T128::new(140_000, 20_000, -20_000)
        );
        assert_eq!(
            T128::try_from("14.0 +/-2 ").unwrap(),
            T128::new(140_000, 20_000, -20_000)
        );

        assert_eq!(
            T128::try_from("14.0 2 1").unwrap(),
            T128::new(140_000, 20_000, 10_000)
        );
        assert_eq!(
            T128::try_from("14.0 -1 -2").unwrap(),
            T128::new(140_000, -10_000, -20_000)
        );

        assert_eq!(
            T128::try_from("141213 -1/-2").unwrap(),
            T128::new(1_412_130_000, -10_000, -20_000)
        );

        assert_eq!(
            T128::try_from("700 .1/-.25").unwrap(),
            T128::new(7_000_000, 1_000, -2_500)
        );
        // eat your own output.
        let t1 = T128::from((653.0, 3.0, -2.5));
        assert_eq!("653.0 +3.00/-2.50", format!("{t1:.1}"));
        assert_eq!(Ok(t1), T128::try_from(format!("{t1:.1}")));

        let t1 = T128::from((-53.0, 3.0, -3.0));
        assert_eq!("-53.0 +/-3.00", format!("{t1:.1}"));
        assert_eq!(Ok(t1), T128::try_from(format!("{t1:.1}")));
    }

    #[test]
    fn serialize_to_u8_array() {
        let test = T128::from((1234567890, 123455, -124555));
        let max = T128 {
            value: Myth64::MAX,
            plus: Myth32::MAX,
            minus: Myth32::MIN,
        };
        assert_eq!(
            format!("{:?}", test.to_be_bytes()),
            "[0, 0, 0, 0, 73, 150, 2, 210, 0, 1, 226, 63, 255, 254, 25, 117]"
        );
        assert_eq!(test, T128::from_be_bytes(test.to_be_bytes()));
        assert_eq!(max, T128::from_be_bytes(max.to_be_bytes()));

        assert_eq!(
            format!("{:?}", test.to_le_bytes()),
            "[210, 2, 150, 73, 0, 0, 0, 0, 63, 226, 1, 0, 117, 25, 254, 255]"
        );
        assert_eq!(test, T128::from_le_bytes(test.to_le_bytes()));
        assert_eq!(max, T128::from_le_bytes(max.to_le_bytes()));
    }

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
    fn display_is_adjustable() {
        let o = T128::new(20_000, 50, -100);
        assert_eq!(format!("{o}"), String::from("2.00 +0.005/-0.010"));
        assert_eq!(format!("{o:.3}"), "2.000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.4}"), "2.0000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.0}"), String::from("2 +/-0.0"));
        assert_eq!(format!("{o:.1}"), String::from("2.0 +/-0.01"));

        let o = T128::with_sym(20_000, 50);
        assert_eq!(format!("{o}"), String::from("2.00 +/-0.005"));
        assert_eq!(format!("{o:.0}"), String::from("2 +/-0.0"));

        let o = T128::new(0.345, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("0.345 +0.0100/-0.0140"));
        let o = T128::new(-0.35, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("-0.350 +0.0100/-0.0140"));

        assert_eq!(format!("{o:#}"), String::from("-3500 +100/-140"));

        assert_eq!("T128(-0.3500 +0.0100 -0.0140)", format!("{o:.3?}"));
    }

    #[test]
    fn construct_consistent() {
        let o = T128::from((2.0, 0.005, -0.01));
        assert_eq!(o.to_string(), "2.00 +0.005/-0.010".to_string())
    }

    #[test]
    fn subtract() {
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
            ToleranceError::parse_err("T128 not parsable from 'nil'!")
        );

        let tol = T128::try_from("");
        assert!(tol.is_err(), "T128 ");
        assert_eq!(
            tol,
            ToleranceError::parse_err("Can not parse an empty string into a T128!")
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
