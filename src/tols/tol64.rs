use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Not, Sub, SubAssign};
use std::str::FromStr;

use crate::error::ToleranceError::ParseError;
use crate::{error, Myth16, Myth32};

/// # 64bit tolerance-type
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
/// assert_eq!(format!("{width}"), "100.0 +0.05/-0.2");
/// assert_eq!(format!("{width:?}"), "T64(100.0 +0.05 -0.2)");
/// ```
///
/// The `plus` and `minus` tolerances are in the same scale unit as the `value`.
/// `plus` is signed positive (`+`) and `minus` is signed negative (`-`).
#[cfg_attr(
    feature = "serde",
    doc = include_str!("serde.md")
)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct T64 {
    #[cfg_attr(feature = "serde", doc = "In deserialization `value` or `v` is used.")]
    pub value: Myth32,
    #[cfg_attr(feature = "serde", doc = "In deserialization `plus` or `p` is used.")]
    pub plus: Myth16,
    #[cfg_attr(feature = "serde", doc = "In deserialization `minus` or `m` is used.")]
    pub minus: Myth16,
}

super::tolerance_body!(T64, Myth32, Myth16);
super::multiply_tolerance!(T64, u64, u32, u16, u8, i64, i32);
#[cfg(feature = "serde")]
super::de_serde_tol!(T64, Myth32, Myth16);

#[cfg(test)]
mod should {
    use super::T64;
    use crate::error::ToleranceError;
    use pretty_assertions::assert_eq;
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
    fn display_is_adjustable() {
        let o = T64::new(20_000, 50, -100);
        assert_eq!(format!("{o}"), String::from("2.0 +0.005/-0.01"));
        assert_eq!(format!("{o:.3}"), "2.000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.4}"), "2.0000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.0}"), String::from("2 +/-0.0"));
        assert_eq!(format!("{o:.1}"), String::from("2.0 +/-0.01"));

        let o = T64::with_sym(20_000, 50);
        assert_eq!(format!("{o}"), String::from("2.0 +/-0.005"));
        assert_eq!(format!("{o:.0}"), String::from("2 +/-0.0"));

        let o = T64::new(0.345, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("0.345 +0.0100/-0.0140"));
        let o = T64::new(-0.35, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("-0.350 +0.0100/-0.0140"));

        assert_eq!(format!("{o:#}"), String::from("-3500 +100/-140"));

        assert_eq!(
            format!("{o:.3?}"),
            String::from("T64(-0.350 +0.010 -0.014)")
        );
    }

    #[test]
    fn construct_consistent() {
        let o = T64::from((2.0, 0.005, -0.01));
        assert_eq!(o.to_string(), "2.0 +0.005/-0.01".to_string())
    }

    #[test]
    fn subtract() {
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
            ParseError(String::from("T64 not parsable from 'nil'!"))
        );

        let a = T64::try_from("");
        assert!(a.is_err(), "T64 ");
        assert_eq!(
            a.unwrap_err(),
            ParseError(String::from("Can not parse an empty string into a T64!"))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_string() {
        use crate::{into_string, T64};
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct T1 {
            width: T64,
        }
        let t = T1 {
            width: T64::from(123455),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(r#"{"width":{"value":123455,"plus":0,"minus":0}}"#, json);
        let t2: T1 = serde_json::from_str(&json).unwrap();
        assert_eq!(t2, t);

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct T2 {
            #[serde(serialize_with = "into_string")]
            width: T64,
        }
        let t = T2 {
            width: T64::from(123460),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(r#"{"width":"12.346 +/-0.0"}"#, json);
        let t2: T2 = serde_json::from_str(&json).unwrap();
        assert_eq!(t2, t);

        let t = T2 {
            width: T64::new(123.4, 0.5, -0.5),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(r#"{"width":"123.4 +/-0.5"}"#, json);
        let t2: T2 = serde_json::from_str(&json).unwrap();
        assert_eq!(t2, t);

        let t = T2 {
            width: T64::new(123.4, 0.5, -0.3),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(r#"{"width":"123.4 +0.5/-0.3"}"#, json);
        let t2: T2 = serde_json::from_str(&json).unwrap();
        assert_eq!(t2, t);

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct T3 {
            #[serde(serialize_with = "into_string")]
            width: Option<T64>,
        }
        let t = T3 {
            width: Some(T64::from(123.460)),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(r#"{"width":"123.46 +/-0.0"}"#, json);
        let t = T3 { width: None };
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(r#"{"width":null}"#, json);
    }
}
