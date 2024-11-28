use crate::{error::ToleranceError, Myth16, Myth32, Unit};
#[cfg(feature = "serde")]
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use std::str::FromStr;

///
/// # 64bit measurement type
///
/// A type to calculate lossless dimensions with a fixed 4 digit precision.
///
/// If you define `Myth64` as the tenth fraction of `μ`:
///
///  * `10` = 1 μ
///  * `10_000`  = 1 mm
///  * `10_000_000`  = 1 m
///
/// The standard `Display::fmt`-method represents the value in `mm`. The *alternate* Display
/// shows the `i64` value.
///
/// ### Example:
/// ```rust
///#    use tolerance::Myth64;
///     let myth = Myth64::from(12.5);
///
///     assert_eq!(format!("{myth}"),"12.5000");
///     assert_eq!(format!("{myth:.2}"), "12.50");
///     assert_eq!(format!("{myth:#}"), "125000");
/// ```
///
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
#[must_use]
pub struct Myth64(pub(crate) i64);

super::calc_with_myths!(Myth64, i64, Myth64, Myth32, Myth16);
super::from_myths!(Myth64, Myth32, Myth16);
super::from_number!(Myth64, u32, u16, u8, i64, i32, i16, i8);
super::standard_myths!(Myth64, i64, u64, u32, u16, u8, usize, i64, i32, i16, i8, isize);
super::try_from_number!(Myth64, u64, usize, isize);
#[cfg(feature = "serde")]
super::de_serde!(Myth64, i64);

#[cfg(test)]
mod should {
    use super::{Myth64, Unit};
    use pretty_assertions::assert_eq;

    #[test]
    fn multiply() {
        let d = Myth64(2_500_000);
        let p1 = Myth64(100_000);
        let p2 = Myth64(250_000);
        assert_eq!(d, p1 * 25);
        assert_eq!(d, 10 * p2);
    }

    #[test]
    fn subtract() {
        let s = Myth64(350_000);
        let s1 = Myth64(100_000);
        let s2 = Myth64(250_000);

        assert_eq!(s1, s - s2);
        assert_eq!(s2, s - s1);
    }

    #[test]
    fn try_from_str() {
        let d = Myth64::try_from("12345.12343").unwrap();
        assert_eq!(d, Myth64(123_451_234));
        let d = Myth64::try_from("6.02").unwrap();
        assert_eq!(d, Myth64(60_200));
        let d = Myth64::try_from("18").unwrap();
        assert_eq!(d, Myth64(180_000));
        let d = Myth64::try_from("0").unwrap();
        assert_eq!(d, Myth64(0));
        let d = Myth64::try_from("14.9300").unwrap();
        assert_eq!(d, Myth64(149_300));

        let d = Myth64::try_from(" +2.07").unwrap();
        assert_eq!(d, Myth64(20_700));
        let d = Myth64::try_from("-3.01").unwrap();
        assert_eq!(d, Myth64(-30_100));

        let d = Myth64::try_from(".01").unwrap();
        assert_eq!(d, Myth64(100));

        let d = Myth64::try_from(".01").unwrap();
        assert_eq!(d, Myth64(100));

        let d = Myth64::try_from("-.044").unwrap();
        assert_eq!(d, Myth64(-440));

        let d = Myth64::try_from("+.01").unwrap();
        assert_eq!(d, Myth64(100));

        let d = Myth64::try_from("-12345.12343").unwrap();
        assert_eq!(d, -Myth64(123_451_234));
        let d = Myth64::try_from("-12345.12346345").unwrap();
        assert_eq!(d, -Myth64(123_451_234));

        // not parsable
        let d = Myth64::try_from("12345*12343");
        assert!(d.is_err());

        let d = Myth64::try_from("   ");
        assert!(d.is_err());

        let d = Myth64::try_from(" -  ");
        assert!(d.is_err());

        let d = Myth64::try_from("+");
        assert!(d.is_err());

        let m = Myth64::from(5445.234);
        let m_s = m.to_string();
        assert_eq!("5445.2340", m_s);
        assert_eq!(Ok(m), Myth64::try_from(m_s));
    }

    #[test]
    fn round() {
        let m = Myth64(1_234_567);
        assert_eq!(Myth64(1_234_570), m.round(Unit::MY));
        assert_eq!(Myth64(1_200_000), m.round(Unit::CM));
        assert_eq!(Myth64(10_000_000), Myth64(9_999_000).round(Unit::MM));
        assert_eq!(Myth64(0), Myth64::from(-0.4993).round(Unit::MM));
        assert_eq!(Myth64(-4990), Myth64::from(-0.4993).round(Unit::MY));
        assert_eq!(Myth64(-10000), Myth64::from(-5000).round(Unit::MM));
        let m = Myth64::from(340.993);
        assert_eq!(10, *Unit::potency(1));
        assert_eq!(Myth64(3_409_930), m.round(Unit::potency(1)));
        assert_eq!(100, *Unit::potency(2));
        assert_eq!(Myth64(3_409_900), m.round(Unit::potency(2)));
        assert_eq!(1000, *Unit::potency(3));
        assert_eq!(Myth64(3_410_000), m.round(Unit::potency(3)));
        assert_eq!(Myth64(3_400_000), m.floor(Unit::potency(4)));
        assert_eq!(-341.000, (-340.993_f64).floor());
        assert_eq!(
            Myth64(-3_410_000),
            Myth64::from(-340.993).floor(Unit::potency(4))
        );
        assert_eq!(Myth64(0), Myth64(4_567).floor(Unit::potency(4)));
        assert_eq!(Myth64(10_000), Myth64(5_567).round(Unit::potency(4)));
        let m = Myth64(-67);
        assert_eq!(Myth64(0), m.round(Unit::potency(3)));
        let m = Myth64(-67);
        assert_eq!(Myth64(-1_000), m.floor(Unit::potency(3)));
        let m = Myth64(-67);
        assert_eq!(Myth64(-100), m.floor(Unit::potency(2)));
    }

    #[test]
    fn display() {
        let m = Myth64(12455);
        assert_eq!("1.2455", format!("{m}").as_str());
        assert_eq!("1.246", format!("{m:.3}").as_str());
        assert_eq!("1.2", format!("{m:.1}").as_str());
        assert_eq!("1.2455", format!("{m:.7}").as_str());
        assert_eq!("1", format!("{m:.0}").as_str());
        assert_eq!("-1.2455", format!("{:.7}", -m).as_str());
        let m = Myth64(-455);
        assert_eq!("-0.046", format!("{m:.3}").as_str());
        assert_eq!("-0.3450", format!("{}", Myth64(-3450)).as_str());
        assert_eq!("-455", format!("{m:#}").as_str());
        let m = Myth64::from(4566.4689);
        assert_eq!(format!("{m:.3}"), "4566.469");
        let m = Myth64::ZERO;
        assert_eq!(format!("{m:.2}"), "0.00");
    }

    #[test]
    fn min_max() {
        let max = Myth64::MAX;
        let min = Myth64::MIN;
        assert_eq!(max.0, 9_223_372_036_854_775_807);
        assert_eq!(min.0, -9_223_372_036_854_775_808);
    }

    #[test]
    fn as_unit() {
        let m = Myth64::from(12456.832);
        assert_eq!(m.as_unit(Unit::CM), 1245.6832);
        assert_eq!(m.as_unit(Unit::METER), 12.456_832);
        let m = Myth64::MAX;
        assert_eq!(m.as_unit(Unit::KM), 922_337_203.685_477_6);
    }

    #[test]
    fn sum() {
        let m64s = (0..10).map(|d| Myth64::from(d * 10_000));
        assert_eq!(Myth64::from(450_000), m64s.sum());
    }

    #[cfg(feature = "serde")]
    mod serde {
        use crate::Myth64;
        use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};

        #[test]
        fn serialize_i64() {
            let m = Myth64::from(12456.832);
            assert_tokens(
                &m,
                &[
                    Token::NewtypeStruct { name: "Myth64" },
                    Token::I64(124_568_320),
                ],
            );
        }

        #[test]
        fn deserialize_string() {
            assert_de_tokens_error::<Myth64>(
                &[Token::String("nonumber")],
                "invalid value: string \"nonumber\", expected 1.0",
            );
            assert_de_tokens(&Myth64::from(23.004), &[Token::String("23.004")]);
            assert_de_tokens(&Myth64::from(0.04), &[Token::String(".04")]);
            assert_de_tokens(&Myth64::from(0.04), &[Token::Str(".04")]);
        }

        #[test]
        fn deserialize_f64() {
            assert_de_tokens(&Myth64::from(23.004), &[Token::F64(23.004)]);
            assert_de_tokens(&Myth64::from(0.0043), &[Token::F64(0.0043)]);
        }

        #[test]
        fn deserialize_i64() {
            assert_de_tokens(&Myth64::from(23.004), &[Token::I64(23_0040)]);
            assert_de_tokens(&Myth64::from(0.0043), &[Token::I64(0_0043)]);
        }

        #[test]
        fn deserialize_option() {
            assert_de_tokens(&Myth64::from(23.004), &[Token::Some, Token::I64(23_0040)]);
            assert_de_tokens(
                &Myth64::from(0.0043),
                &[Token::Some, Token::String(".0043")],
            );
        }

        #[test]
        fn deserialize_i32() {
            assert_de_tokens(&Myth64::from(23.004), &[Token::I32(23_0040)]);
            assert_de_tokens(&Myth64::from(0.0043), &[Token::I32(0_0043)]);
        }

        #[test]
        fn deserialize_json() {
            let m = serde_json::from_slice(b"23.004").unwrap();
            assert_eq!(Myth64::from(23.004), m);

            let m = serde_json::from_slice(b"\".004\"").unwrap();
            assert_eq!(Myth64::from(0.004), m);

            let m = serde_json::from_slice(b"4000").unwrap();
            assert_eq!(Myth64::from(0.4), m);
        }
    }
}
