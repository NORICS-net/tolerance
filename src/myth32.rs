use crate::{error::ToleranceError, Myth16, Myth64, Unit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub, SubAssign};

///
/// # Myth32
///
/// A type to calculate lossless dimensions with a fixed precision.
/// All sizes are defined in the tenth fraction of `μ`:
///
///  * `10` = 1 μ
///  * `10_000`  = 1 mm
///  * `10_000_000`  = 1 m
///
/// The standard `Display::fmt`-methode represents the value in `mm`. The *alternate* Display
/// shows the `i32` value.
///
/// As `10_000` = 1 mm
///
/// ### Warning
/// Casting an `i64` into a `Myth32` can cause an `IntegerOverflow`-error similar to casting
/// a big `i64`-value into an `i32`. It's up to the programmer to omit these situation. Don't
/// try to store more then `+/- 214 meter` in a `Myth32`.
///
/// ### Example:
/// ```rust
///#    use tolerance::Myth32;
///     let myth = Myth32::from(12.5);
///
///     assert_eq!(format!("{myth}"),"12.5000");
///     assert_eq!(format!("{myth:.2}"), "12.50");
///     assert_eq!(format!("{myth:#}"), "125000");
/// ```
///
///

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[must_use]
pub struct Myth32(i32);

impl Myth32 {
    pub const MY: i32 = 10;
    pub const MM: Myth32 = Myth32(1_000 * Self::MY);
    pub const ZERO: Myth32 = Myth32(0);
    /// Holds at maximum 214m
    pub const MAX: Myth32 = Myth32(i32::MAX);
    /// Holds at minimum -214m
    pub const MIN: Myth32 = Myth32(i32::MIN);

    #[must_use]
    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

super::math_number!(Myth32, i32, u64, u32, u16, u8, usize, i64, i32, i16, i8, isize);
super::from_number!(Myth32, u16, u8, i32, i16, i8);
super::try_from_number!(Myth32, u64, u32, i64, isize, usize);

impl From<Myth32> for Myth64 {
    fn from(m: Myth32) -> Self {
        Myth64::from(m.0)
    }
}

impl TryFrom<&str> for Myth32 {
    type Error = ToleranceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(super::try_from_str(value.trim())?)
    }
}

impl TryFrom<String> for Myth32 {
    type Error = ToleranceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(super::try_from_str(value.trim())?)
    }
}

impl TryFrom<Myth64> for Myth32 {
    type Error = ToleranceError;

    fn try_from(value: Myth64) -> Result<Self, Self::Error> {
        Self::try_from(value.as_i64())
    }
}

#[cfg(test)]
mod should {
    use super::{Myth32, Ordering, Unit};

    #[test]
    fn try_from_str() {
        let d = Myth32::try_from("12345.12343").unwrap();
        assert_eq!(d, Myth32(1_234_512_34));
        let d = Myth32::try_from("6.02").unwrap();
        assert_eq!(d, Myth32(60_200));

        let d = Myth32::try_from(" +2.07").unwrap();
        assert_eq!(d, Myth32(20_700));
        let d = Myth32::try_from("-3.01").unwrap();
        assert_eq!(d, Myth32(-30_100));
    }

    #[test]
    fn cmp() {
        let s1 = Myth32(200_000);
        let i1 = Myth32(190_000);
        let s2 = Myth32::from(20.0);
        let i2 = Myth32::from(19.0);

        assert!(s1 > i1);
        assert_eq!(s1.partial_cmp(&i1).unwrap(), Ordering::Greater);
        assert_eq!(s1, s1);
        assert_eq!(s1.partial_cmp(&s1).unwrap(), Ordering::Equal);

        assert!(s2 > i2);
        assert_eq!(s2.partial_cmp(&i2).unwrap(), Ordering::Greater);
        assert_eq!(s2, s1);
        assert_eq!(s2.partial_cmp(&s1).unwrap(), Ordering::Equal);

        assert_eq!(i1.cmp(&s1), Ordering::Less);
        assert_eq!(i1.cmp(&i1), Ordering::Equal);
    }

    #[test]
    fn neg() {
        let m = -Myth32(232_332);
        let n = Myth32(-232_332);
        assert_eq!(n.0, m.0);
        assert_eq!(n, m);
    }

    #[test]
    fn round() {
        let m = Myth32(1_234_567);
        assert_eq!(Myth32(1_234_570), m.round(Unit::MY));
        assert_eq!(Myth32(1_200_000), m.round(Unit::CM));
        assert_eq!(Myth32(10_000_000), Myth32(9_999_000).round(Unit::MM));
        assert_eq!(Myth32(0), Myth32::from(-0.4993).round(Unit::MM));
        assert_eq!(Myth32(-4990), Myth32::from(-0.4993).round(Unit::MY));
        assert_eq!(Myth32(-10000), Myth32::from(-5000).round(Unit::MM));
        let m = Myth32::from(340.993);
        assert_eq!(10, Unit::DYN(1).multiply());
        assert_eq!(Myth32(3_409_930), m.round(Unit::DYN(1)));
        assert_eq!(100, Unit::DYN(2).multiply());
        assert_eq!(Myth32(3_409_900), m.round(Unit::DYN(2)));
        assert_eq!(1000, Unit::DYN(3).multiply());
        assert_eq!(Myth32(3_410_000), m.round(Unit::DYN(3)));
        assert_eq!(Myth32(3_400_000), m.floor(Unit::DYN(4)));
        assert_eq!(-340.000, -(340.993_f64.floor()));
        assert_eq!(
            Myth32(-3_400_000),
            Myth32::from(-340.993).floor(Unit::DYN(4))
        );
    }

    #[test]
    fn display() {
        let m = Myth32(12455);
        assert_eq!("1.2455", format!("{m}").as_str());
        assert_eq!("1.246", format!("{m:.3}").as_str());
        assert_eq!("1.2", format!("{m:.1}").as_str());
        assert_eq!("1.2455", format!("{m:.7}").as_str());
        assert_eq!("1", format!("{m:.0}").as_str());
        assert_eq!("-1.2455", format!("{:.7}", -m).as_str());
        let m = Myth32(-455);
        assert_eq!("-0.0455", format!("{m}").as_str());
        assert_eq!("-0.3450", format!("{}", Myth32(-3450)).as_str());
        assert_eq!("-455", format!("{m:#}").as_str());
        let m = Myth32::from(4566.4689);
        assert_eq!(format!("{m:.3}"), "4566.469");
        let m = Myth32::ZERO;
        assert_eq!(format!("{m:.2}"), "0.00");
    }

    #[test]
    fn min_max() {
        let max = Myth32::MAX;
        let min = Myth32::MIN;

        assert_eq!(max.0, 2_147_483_647);
        assert_eq!(min.0, -2_147_483_648);
        assert_eq!(format!("{max:.0}"), "214748");
    }

    #[test]
    fn as_unit() {
        let m = Myth32::from(12456.832);
        assert_eq!(m.as_unit(Unit::CM), 1245.6832);
        assert_eq!(m.as_unit(Unit::METER), 12.456_832);
    }
}
