use super::{error::ToleranceError, Myth16, Myth32, Unit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

///
/// # Myth64
///
/// A type to calculate lossless dimensions with a fixed precision.
/// All sizes are defined in the tenth fraction of `μ`:
///
///  * `10` = 1 μ
///  * `10_000`  = 1 mm
///  * `10_000_000`  = 1 m
///
/// The standard `Display::fmt`-methode represents the value in `mm`. The *alternate* Display
/// shows the `i64` value.
///
/// As `10_000` = 1 mm
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
///

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
#[must_use]
pub struct Myth64(i64);

impl Myth64 {
    pub const MY: i64 = 10;
    pub const MM: Myth64 = Myth64(1_000 * Self::MY);
    pub const ZERO: Myth64 = Myth64(0);
    /// Holds at MAX 922 337 203 km
    pub const MAX: Myth64 = Myth64(i64::MAX);
    /// Holds at MIN -922 337 203 km
    pub const MIN: Myth64 = Myth64(i64::MIN);
}

super::standard_myths!(Myth64, i64, u64, u32, u16, u8, usize, i64, i32, i16, i8, isize);
super::from_number!(Myth64, u32, u16, u8, i64, i32, i16, i8);
super::try_from_number!(Myth64, u64, usize, isize);

impl TryFrom<&str> for Myth64 {
    type Error = ToleranceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        super::try_from_str(value.trim()).map(Self::from)
    }
}

impl TryFrom<String> for Myth64 {
    type Error = ToleranceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        super::try_from_str(value.trim()).map(Self::from)
    }
}

impl std::str::FromStr for Myth64 {
    type Err = ToleranceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        super::try_from_str(value.trim()).map(Self::from)
    }
}

#[cfg(test)]
mod should {
    use super::{Myth64, Unit};

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

        let d = Myth64::try_from("-12345.12343").unwrap();
        assert_eq!(d, -Myth64(123_451_234));
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
        assert_eq!(-340.000, -(340.993_f64.floor()));
        assert_eq!(
            Myth64(-3_400_000),
            Myth64::from(-340.993).floor(Unit::potency(4))
        );
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
        assert_eq!("-0.0455", format!("{m}").as_str());
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

    #[cfg(feature = "serde")]
    mod serde {
        use crate::Myth64;
        use serde_test::{assert_tokens, Token};

        #[test]
        fn serialize() {
            let m = Myth64::from(12456.832);
            assert_tokens(
                &m,
                &[
                    Token::NewtypeStruct { name: "Myth64" },
                    Token::I64(124_568_320),
                ],
            );
        }
    }
}
