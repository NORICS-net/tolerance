use crate::{error::ToleranceError, Myth32, Myth64, Unit};
#[cfg(feature = "serde")]
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use std::str::FromStr;

///
/// # 16bit measurement type
///
/// A type to calculate lossless dimensions with a fixed 4 digit precision.
///
/// All sizes are defined in the tenth fraction of `μ`:
///
///  * `10` = 1 μ
///  * `10_000`  = 1 mm
///  * `30_000`  = 3 mm
///
/// The standard `Display::fmt`-method represents the value in `mm`. The *alternate* Display
/// shows the `i16` value.
///
/// As `10_000` = 1 mm
///
/// ### Warning
/// Casting an `i64` into a `Myth16` can cause an `IntegerOverflow`-error similar to casting
/// a big `i64`-value into an `i16`. It's up to the programmer to omit these situation.
///
/// If your sizes can exceed 3 mm, than this type is __not__ for you. Again:
///
/// ⚠ **Don't try to store more then +/- 3 millimeter in a** `Myth16`.
///
/// ### Example:
/// ```rust
///#    use tolerance::Myth16;
///     let myth = Myth16::from(1.5);
///
///     assert_eq!(format!("{myth}"), "1.5");
///     assert_eq!(format!("{myth:.2}"), "1.50");
///     assert_eq!(format!("{myth:.4}"),"1.5000");
///     assert_eq!(format!("{myth:#}"), "15000");
/// ```

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
#[must_use]
pub struct Myth16(pub(crate) i16);

impl Myth16 {
    #[must_use]
    pub const fn as_i16(&self) -> i16 {
        self.0
    }
}

super::standard_myths!(Myth16, i16, u64, u32, u16, u8, usize, i64, i32, i16, i8, isize);
super::from_number!(Myth16, u8, i16, i8);
super::try_from_number!(Myth16, u64, u32, u16, i64, isize, usize);
super::try_from_myths!(Myth16, Myth32, Myth64);
super::calc_with_myths!(Myth16, i16, Myth16);
#[cfg(feature = "serde")]
super::de_serde!(Myth16, i16);

/// A potentially dangerous function.
/// Use it for creating `Myth16` in tests or where you can control the danger.
impl From<i32> for Myth16 {
    fn from(value: i32) -> Self {
        Self(value as i16)
    }
}

#[cfg(test)]
mod should {
    use super::{Myth16, Unit};
    use pretty_assertions::assert_eq;

    #[test]
    fn try_from_str() {
        let d = Myth16::try_from("2.1234").unwrap();
        assert_eq!(d, Myth16(21_234));
        let d = Myth16::try_from("3.01").unwrap();
        assert_eq!(d, Myth16(30_100));

        let d = Myth16::try_from(" +2.07").unwrap();
        assert_eq!(d, Myth16(20_700));
        let d = Myth16::try_from("-3.01").unwrap();
        assert_eq!(d, Myth16(-30_100));
    }

    #[test]
    fn neg() {
        let m = -Myth16(2323);
        let n = Myth16(-2323);
        assert_eq!(n.0, m.0);
        assert_eq!(n, m);
    }

    #[test]
    fn round() {
        let m = Myth16(12345);
        assert_eq!(Myth16(12350), m.round(Unit::MY));
        assert_eq!(Myth16(10_000), m.round(Unit::MM));
        assert_eq!(Myth16(10_000), Myth16(9_000).round(Unit::MM));
        assert_eq!(Myth16(0), Myth16::from(-0.4993).round(Unit::MM));
        assert_eq!(Myth16(-4990), Myth16::from(-0.4993).round(Unit::MY));
        assert_eq!(Myth16(-10000), Myth16::from(-5000i16).round(Unit::MM));
        let m = Myth16::from(2.993);
        assert_eq!(10, *Unit::potency(1));
        assert_eq!(Myth16(29930), m.round(Unit::potency(1)));
        assert_eq!(100, *Unit::potency(2));
        assert_eq!(Myth16(29900), m.round(Unit::potency(2)));
        assert_eq!(1000, *Unit::potency(3));
        assert_eq!(Myth16(30000), m.round(Unit::potency(3)));
        assert_eq!(Myth16(20000), m.floor(Unit::potency(4)));
        assert_eq!(Myth16(-30000), Myth16::from(-2.293).floor(Unit::potency(4)));
    }

    #[test]
    fn display() {
        let m = Myth16(12455);
        assert_eq!("1.2455", format!("{m}").as_str());
        assert_eq!("1.246", format!("{m:.3}").as_str());
        assert_eq!("1.2", format!("{m:.1}").as_str());
        assert_eq!("1.2455", format!("{m:.7}").as_str());
        assert_eq!("1", format!("{m:.0}").as_str());
        assert_eq!("-1.2455", format!("{:.7}", -m).as_str());
        let m = Myth16(-455);
        assert_eq!("-0.0455", format!("{m}").as_str());
        assert_eq!("-0.345", format!("{}", Myth16(-3450)).as_str());
        assert_eq!("-0.3450", format!("{:.4}", Myth16(-3450)).as_str());
        assert_eq!("-455", format!("{m:#}").as_str());
        let m = Myth16::from(1.4689);
        assert_eq!(format!("{m:.3}"), "1.469");
        let m = Myth16::ZERO;
        assert_eq!(format!("{m:.2}"), "0.00");
    }

    #[test]
    fn min_max() {
        let max = Myth16::MAX;
        let min = Myth16::MIN;

        assert_eq!(max.0, 32767);
        assert_eq!(min.0, -32768);
        assert_eq!(format!("{max:.0}"), "3");
    }

    #[test]
    fn as_unit() {
        let m = Myth16::from(0.832);
        assert_eq!(m.as_unit(Unit::CM), 0.0832);
        assert_eq!(m.as_unit(Unit::MY), 832.0);
    }
}
