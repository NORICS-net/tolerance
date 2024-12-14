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
/// assert_eq!(format!("{width}"), "100.0 +0.05/-0.2");
/// assert_eq!(format!("{width:.3}"), "100.000 +0.0500/-0.2000");
/// assert_eq!(format!("{width:?}"), "T128(100.0 +0.05 -0.2)");
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
#[cfg_attr(
    feature = "serde",
    doc = include_str!("serde.md")
)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct T128 {
    #[cfg_attr(feature = "serde", doc = "In deserialization `value` or `v` is used.")]
    pub value: Myth64,
    #[cfg_attr(feature = "serde", doc = "In deserialization `plus` or `p` is used.")]
    pub plus: Myth32,
    #[cfg_attr(feature = "serde", doc = "In deserialization `minus` or `m` is used.")]
    pub minus: Myth32,
}

super::tolerance_body!(T128, Myth64, Myth32);
super::multiply_tolerance!(T128, u64, u32, u16, u8, i64, i32);
#[cfg(feature = "serde")]
super::de_serde_tol!(T128, Myth64, Myth32);

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

        let a = T128::new(363_000, 10_000, 0);
        assert_eq!(a, T128::try_from(a.to_string()).unwrap());
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
    fn display_compact() {
        let o = T128::new(20_000, 50, -100);
        assert_eq!(format!("{o}"), "2.0 +0.005/-0.01");
        let o = T128::new(20_000, 50, -50);
        assert_eq!(format!("{o}"), "2.0 +/-0.005");
        let o = T128::new(20_000, 0, 0);
        assert_eq!(format!("{o}"), "2.0 +/-0.0");
        let o = T128::new(20_000, 50, 0);
        assert_eq!(format!("{o}"), "2.0 +0.005/-0.0");
        let o = T128::new(20_000, 0, -400);
        assert_eq!(format!("{o}"), "2.0 +0.0/-0.04");
        let o = T128::new(20_000, 800, 400);
        assert_eq!(format!("{o}"), "2.0 +0.08/+0.04");
        let o = T128::new(20_000, -400, -800);
        assert_eq!(format!("{o}"), "2.0 -0.04/-0.08");
    }

    #[test]
    fn display_is_adjustable() {
        let o = T128::new(20_000, 50, -100);
        assert_eq!("2.0 +0.005/-0.01", format!("{o}"));
        assert_eq!("2.000 +0.0050/-0.0100", format!("{o:.3}"));
        assert_eq!("2.0000 +0.0050/-0.0100", format!("{o:.4}"));
        assert_eq!("2 +/-0.0", format!("{o:.0}"));
        assert_eq!("2.0 +/-0.01", format!("{o:.1}"));

        let o = T128::with_sym(20_000, 50);
        assert_eq!("2.0 +/-0.005", format!("{o}"));
        assert_eq!("2 +/-0.0", format!("{o:.0}"));

        let o = T128::new(0.345, 0.010, -0.014);
        assert_eq!("0.345 +0.0100/-0.0140", format!("{o:.3}"));
        let o = T128::new(-0.35, 0.010, -0.014);
        assert_eq!("-0.350 +0.0100/-0.0140", format!("{o:.3}"));
        assert_eq!("      -0.35 +0.010/-0.014", format!("{o:>25.2}"),);
        assert_eq!("   -0.35 +0.010/-0.014   ", format!("{o:^25.2}"),);

        assert_eq!(format!("{o:#}"), "-3500 +100/-140");
        assert_eq!("T128(-0.350 +0.010 -0.014)", format!("{o:.3?}"));
    }

    #[test]
    fn construct_consistent() {
        let o = T128::from((2.0, 0.005, -0.01));
        assert_eq!("2.0 +0.005/-0.01", o.to_string())
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
        use crate::*;
        use pretty_assertions::assert_eq;
        use serde::{Deserialize, Serialize};
        use serde_test::{assert_de_tokens, assert_tokens, Token};

        #[test]
        fn serialize_std() {
            #[derive(Serialize)]
            struct T1 {
                width: T128,
            }
            let t = T1 {
                width: T128::from(123455),
            };
            assert_eq!(
                r#"{"width":{"value":123455,"plus":0,"minus":0}}"#,
                serde_json::to_string(&t).unwrap()
            );
        }

        #[test]
        fn serialize_to_tol_string() {
            #[derive(Serialize)]
            struct T2 {
                #[serde(serialize_with = "into_string")]
                width: T128,

                #[serde(serialize_with = "into_string")]
                length: Option<T128>,
            }
            let t = T2 {
                width: T128::from(123455),
                length: Some(T128::new(1230.0, 40.0, 0)),
            };
            assert_eq!(
                r#"{"width":"12.3455 +/-0.0","length":"1230.0 +40.0/-0.0"}"#,
                serde_json::to_string(&t).unwrap()
            );
            let t = T2 {
                width: T128::new(123.4, 0.5, -0.5),
                length: None,
            };
            assert_eq!(
                r#"{"width":"123.4 +/-0.5","length":null}"#,
                serde_json::to_string(&t).unwrap()
            );
        }

        #[test]
        fn serialize_to_float_struct() {
            #[derive(Serialize)]
            struct T1 {
                #[serde(serialize_with = "T128::into_float_struct")]
                width: T128,
            }
            let t = T1 {
                width: T128::from(123455),
            };
            assert_eq!(
                r#"{"width":{"value":12.3455,"plus":0.0,"minus":0.0}}"#,
                serde_json::to_string(&t).unwrap()
            );
        }

        #[test]
        fn serialize_to_float_seq() {
            #[derive(Serialize)]
            struct T1 {
                #[serde(serialize_with = "T128::into_float_seq")]
                width: T128,
            }
            let t = T1 {
                width: T128::from(123455),
            };
            assert_eq!(
                r#"{"width":[12.3455,0.0,0.0]}"#,
                serde_json::to_string(&t).unwrap()
            );

            #[derive(Serialize, Deserialize, Debug, PartialEq)]
            struct T2 {
                #[serde(serialize_with = "T128::option_into_float_seq")]
                width: Option<T128>,
            }
            let t = T2 {
                width: Some(T128::from(123455)),
            };
            assert_eq!(
                r#"{"width":[12.3455,0.0,0.0]}"#,
                serde_json::to_string(&t).unwrap()
            );
            assert_tokens(
                &t,
                &[
                    Token::Struct { name: "T2", len: 1 },
                    Token::Str("width"),
                    Token::Some,
                    Token::Seq { len: Some(3) },
                    Token::F64(12.3455),
                    Token::F64(0.0),
                    Token::F64(0.0),
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );
            let t = T2 { width: None };
            assert_eq!(r#"{"width":null}"#, serde_json::to_string(&t).unwrap());
        }

        #[test]
        fn serialize_newtype_struct() {
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

        #[test]
        fn deserialize_struct() {
            let tol = T128::from(1230000);
            // Full
            assert_tokens(
                &tol,
                &[
                    Token::Struct {
                        name: "T128",
                        len: 3,
                    },
                    Token::Str("value"),
                    Token::NewtypeStruct { name: "Myth64" },
                    Token::I64(1230000),
                    Token::Str("plus"),
                    Token::NewtypeStruct { name: "Myth32" },
                    Token::I32(0),
                    Token::Str("minus"),
                    Token::NewtypeStruct { name: "Myth32" },
                    Token::I32(0),
                    Token::StructEnd,
                ],
            );
            // aliasse
            assert_de_tokens(
                &tol,
                &[
                    Token::Struct {
                        name: "T128",
                        len: 3,
                    },
                    Token::Str("v"),
                    Token::NewtypeStruct { name: "Myth64" },
                    Token::I64(1230000),
                    Token::Str("p"),
                    Token::NewtypeStruct { name: "Myth32" },
                    Token::I32(0),
                    Token::Str("m"),
                    Token::NewtypeStruct { name: "Myth32" },
                    Token::I32(0),
                    Token::StructEnd,
                ],
            );
            // defaults
            assert_de_tokens(
                &Some(tol),
                &[
                    Token::Some,
                    Token::Struct {
                        name: "T128",
                        len: 1,
                    },
                    Token::Str("v"),
                    Token::NewtypeStruct { name: "Myth64" },
                    Token::I64(1230000),
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn deserialize_json() {
            let t: T128 = serde_json::from_slice(b"{\"v\": 1245.67}").unwrap();
            assert_eq!(t, T128::new(1245_6700, 0, 0));

            let t: T128 =
                serde_json::from_slice(b"{\"v\": 1245.67, \"plus\": 0.3, \"minus\": -0.5 }")
                    .unwrap();
            assert_eq!(t, T128::new(1245_6700, 0.3, -0.5));

            let t: T128 = serde_json::from_slice(b"[ 1245.67, 0.3, -0.5 ]").unwrap();
            assert_eq!(t, T128::new(1245_6700, 0.3, -0.5));

            let t: T128 = serde_json::from_slice(b"[ 1245.67, 0.3 ]").unwrap();
            assert_eq!(t, T128::new(1245_6700, 0.3, -0.3));

            let t: Result<T128, serde_json::Error> =
                serde_json::from_slice(b"[ 1245.67, 0.3, -0.5, 234.0 ]");
            assert!(t.is_err());

            let t: T128 = serde_json::from_slice(b"1245.67").unwrap();
            assert_eq!(t, T128::new(1245_6700, 0.0, 0.0));

            let t: T128 = serde_json::from_slice(b"\"1245.67 +/- 0.45\"").unwrap();
            assert_eq!(t, T128::new(1245_6700, 0.45, -0.45));

            let t: T128 = serde_json::from_slice(b"\"1245.6700 +0.45 -0.2\"").unwrap();
            assert_eq!(t, T128::new(1245_6700, 0.45, -0.2));
        }

        #[test]
        fn serialize_from_option_t128_default() {
            use crate::*;

            #[derive(Serialize, Deserialize, PartialEq, Debug)]
            struct T3 {
                #[serde(deserialize_with = "T128::empty_to_zero")]
                width: Option<T128>,
            }
            let t = T3 { width: None };
            assert_eq!(r#"{"width":null}"#, serde_json::to_string(&t).unwrap());
            assert_eq!(serde_json::from_str::<T3>(r#"{"width":null}"#).unwrap(), t);
            assert_eq!(
                serde_json::from_str::<T3>(r#"{"width": "123.34"}"#).unwrap(),
                T3 {
                    width: Some(T128::from(123.34))
                }
            );
            assert_eq!(
                serde_json::from_str::<T3>(r#"{"width": ""}"#).unwrap(),
                T3 {
                    width: Some(T128::from(0))
                }
            );
        }
    }
}
