use crate::{Myth32, Myth64, Unit};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, TryFromIntError};
use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub, SubAssign};

///
/// # Myth16
///
/// A type to calculate lossless dimensions with a fixed precision.
/// All sizes are defined in the tenth fraction of `μ`:
///
///  * `10` = 1 μ
///  * `10_000`  = 1 mm
///  * `30_000`  = 3 mm
///
/// The standard `Display::fmt`-methode represents the value in `mm`. The *alternate* Display
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
/// **Don't try to store more then +/- 3 millimeter in a** `Myth16`.
///
/// ### Example:
/// ```rust
///#    use tolerance::Myth16;
///     let myth = Myth16::from(1.5);
///
///     assert_eq!(format!("{myth}"),"1.5000");
///     assert_eq!(format!("{myth:.2}"), "1.50");
///     assert_eq!(format!("{myth:#}"), "15000");
/// ```
///
///

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Myth16(i16);

impl Myth16 {
    pub const MY: i16 = 10;
    pub const MM: Myth16 = Myth16(1_000 * Self::MY);
    pub const ZERO: Myth16 = Myth16(0);
    /// Holds at maximum 3mm
    pub const MAX: Myth16 = Myth16(i16::MAX);
    /// Holds at minimum -3mm
    pub const MIN: Myth16 = Myth16(i16::MIN);

    pub fn as_i16(&self) -> i16 {
        self.0
    }

    #[inline]
    pub fn as_mm(&self) -> f64 {
        f64::from(self.0) / Unit::MM.multiply() as f64
    }

    /// Returns the value in the given `Unit`.
    pub fn as_unit(&self, unit: Unit) -> f64 {
        f64::from(self.0) / unit.multiply() as f64
    }

    /// Rounds to the given Unit.
    pub fn round(&self, unit: Unit) -> Self {
        if unit.multiply() == 0 {
            return *self;
        }
        let m = unit.multiply() as i32;
        let clip = i32::from(self.0 % m as i16);
        match m / 2 {
            _ if clip == 0 => *self, // don't round
            x if clip <= -x => Myth16::from(i32::from(self.0) - clip - m),
            x if clip >= x => Myth16::from(i32::from(self.0) - clip + m),
            _ => Myth16(self.0 - clip as i16),
        }
    }

    pub fn floor(&self, unit: Unit) -> Self {
        let val = self.0;
        let clip = val % unit.multiply() as i16;
        Myth16(val - clip)
    }
}

macro_rules! myth16_from_number {
    ($($typ:ident),+) => {
        $(
            impl From<$typ> for Myth16 {
                fn from(a: $typ) -> Self {
                    assert!(a < i16::MAX as $typ && a > i16::MIN as $typ);
                    Self(a as i16)
                }
            }

            impl From<Myth16> for $typ {
                fn from(a: Myth16) -> Self {
                    a.0 as $typ
                }
            }

            impl Add<$typ> for Myth16 {
                type Output = Myth16;

                fn add(self, rhs: $typ) -> Self::Output {
                    Self(self.0 + (rhs as i16))
                }
            }

            impl AddAssign<$typ> for Myth16 {
                fn add_assign(&mut self, rhs: $typ) {
                    self.0 += (rhs as i16);
                }
            }

            impl Sub<$typ> for Myth16 {
                type Output = Myth16;

                fn sub(self, rhs: $typ) -> Self::Output {
                    Self(self.0 - (rhs as i16))
                }
            }

            impl Mul<$typ> for Myth16 {
                type Output = Myth16;

                fn mul(self, rhs: $typ) -> Self::Output {
                    Self(self.0 * (rhs as i16))
                }
            }

            impl Div<$typ> for Myth16 {
                type Output = Myth16;

                fn div(self, rhs: $typ) -> Self::Output {
                    Self(self.0 / (rhs as i16))
                }
            }
        )+
    }
}

myth16_from_number!(u64, u32, u16, u8, usize, i64, i32, i16, i8);

impl From<Unit> for Myth16 {
    fn from(unit: Unit) -> Self {
        Myth16::from(unit.multiply())
    }
}

impl From<f64> for Myth16 {
    fn from(f: f64) -> Self {
        assert!(
            f < f64::from(i16::MAX) && f > f64::from(i16::MIN),
            "i16 overflow, the f64 is beyond the limits of this type (Myth16)."
        );
        Self((f * f64::from(Myth16::MM.as_i16())) as i16)
    }
}

impl From<Myth16> for f64 {
    fn from(f: Myth16) -> Self {
        f64::from(f.0) / f64::from(Myth16::MM.as_i16())
    }
}

// Upcasting is no problem!
impl From<Myth16> for Myth64 {
    fn from(m: Myth16) -> Self {
        Myth64::from(m.0)
    }
}

// Upcasting is no problem!
impl From<Myth16> for Myth32 {
    fn from(m: Myth16) -> Self {
        Myth32::from(m.0)
    }
}

impl TryFrom<&str> for Myth16 {
    type Error = ParseFloatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Myth16::from(value.parse::<f64>()?))
    }
}

impl TryFrom<String> for Myth16 {
    type Error = ParseFloatError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Myth16::from(value.parse::<f64>()?))
    }
}

impl TryFrom<Myth64> for Myth16 {
    type Error = TryFromIntError;

    fn try_from(value: Myth64) -> Result<Self, Self::Error> {
        let v: i16 = value.as_i64().try_into()?;
        Ok(Myth16(v))
    }
}

impl TryFrom<Myth32> for Myth16 {
    type Error = TryFromIntError;

    fn try_from(value: Myth32) -> Result<Self, Self::Error> {
        let v: i16 = value.as_i32().try_into()?;
        Ok(Myth16(v))
    }
}

impl Display for Myth16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let p = f.precision().map_or(4, |p| p.min(4));
        assert!(p <= 4, "Myth64 has a limited precision of 4!");
        if f.alternate() {
            Display::fmt(&self.0, f)
        } else {
            let val = self.round(Unit::DYN(4 - p)).0;
            let n = if val.is_negative() { 6 } else { 5 };
            let mut s = format!("{val:0n$}");
            if p > 0 {
                s.insert(s.len() - 4, '.');
            }
            s.truncate(s.len() - (4 - p));
            write!(f, "{s}")
        }
    }
}

impl Debug for Myth16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = self.0;
        let n = if val.is_negative() { 6 } else { 5 };
        let mut m = format!("{val:0n$}");
        m.insert(m.len() - 4, '.');
        write!(f, "Myth16({m})")
    }
}

impl Neg for Myth16 {
    type Output = Myth16;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Myth16 {
    type Output = Myth16;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Myth64> for Myth16 {
    type Output = Myth64;

    fn add(self, rhs: Myth64) -> Self::Output {
        Myth64::from(rhs.as_i64() + i64::from(self.as_i16()))
    }
}

impl Add<Myth32> for Myth16 {
    type Output = Myth32;

    fn add(self, rhs: Myth32) -> Self::Output {
        Myth32::from(rhs.as_i32() + i32::from(self.as_i16()))
    }
}

impl Add<Myth16> for Myth32 {
    type Output = Myth32;

    fn add(self, rhs: Myth16) -> Self::Output {
        Myth32::from(i32::from(rhs.as_i16()) + self.as_i32())
    }
}

impl AddAssign for Myth16 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Myth16 {
    type Output = Myth16;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<Myth32> for Myth16 {
    type Output = Myth32;

    fn sub(self, rhs: Myth32) -> Self::Output {
        Myth32::from(i32::from(self.0) - rhs.as_i32())
    }
}

impl Sub<Myth64> for Myth16 {
    type Output = Myth64;

    fn sub(self, rhs: Myth64) -> Self::Output {
        Myth64::from(i64::from(self.0) - rhs.as_i64())
    }
}

impl SubAssign for Myth16 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Myth16 {
    type Output = Myth16;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Myth16 {
    type Output = Myth16;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Deref for Myth16 {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for Myth16 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Myth16 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
mod should {
    use super::{Myth16, Ordering, Unit};

    #[test]
    fn cmp() {
        let s1 = Myth16(20_000);
        let i1 = Myth16(19_000);
        let s2 = Myth16::from(2.0);
        let i2 = Myth16::from(1.9);

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
        assert_eq!(Myth16(-10000), Myth16::from(-5000).round(Unit::MM));
        let m = Myth16::from(2.993);
        assert_eq!(10, Unit::DYN(1).multiply());
        assert_eq!(Myth16(29930), m.round(Unit::DYN(1)));
        assert_eq!(100, Unit::DYN(2).multiply());
        assert_eq!(Myth16(29900), m.round(Unit::DYN(2)));
        assert_eq!(1000, Unit::DYN(3).multiply());
        assert_eq!(Myth16(30000), m.round(Unit::DYN(3)));
        assert_eq!(Myth16(20000), m.floor(Unit::DYN(4)));
        assert_eq!(Myth16(-20000), Myth16::from(-2.293).floor(Unit::DYN(4)));
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
        assert_eq!("-0.3450", format!("{}", Myth16(-3450)).as_str());
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
