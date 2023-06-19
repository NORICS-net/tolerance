use super::Myth32;
use super::Unit;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseFloatError;
use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub, SubAssign};

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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Myth64(i64);

impl Myth64 {
    pub const MY: i64 = 10;
    pub const MM: Myth64 = Myth64(1_000 * Self::MY);
    pub const ZERO: Myth64 = Myth64(0);
    /// Holds at MAX 922 337 203 km
    pub const MAX: Myth64 = Myth64(i64::MAX);
    /// Holds at MIN -922 337 203 km
    pub const MIN: Myth64 = Myth64(i64::MIN);

    pub fn as_i64(&self) -> i64 {
        self.0
    }

    #[inline]
    pub fn as_mm(&self) -> f64 {
        f64::from(self.0) / Unit::MM.multiply() as f64
    }

    /// Returns the value in the given `Unit`.
    pub fn as_unit(&self, unit: Unit) -> f64 {
        f64::from(self.0 ) / unit.multiply() as f64
    }

    /// Rounds to the given Unit.
    pub fn round(&self, unit: Unit) -> Self {
        if unit.multiply() == 0 {
            return *self;
        }
        let m = unit.multiply();
        let clip = self.0 % m;
        match m / 2 {
            _ if clip == 0 => *self, // don't round
            x if clip <= -x => Myth64(self.0 - clip - m),
            x if clip >= x => Myth64(self.0 - clip + m),
            _ => Myth64(self.0 - clip),
        }
    }

    pub fn floor(&self, unit: Unit) -> Self {
        let val = self.0;
        let clip = val % unit.multiply();
        Myth64(val - clip)
    }
}

macro_rules! measure_from_number {
    ($($typ:ident),+) => {
        $(
            impl From<$typ> for Myth64 {
                fn from(a: $typ) -> Self {
                    Self(i64::from(a))
                }
            }

            impl From<Myth64> for $typ {
                fn from(a: Myth64) -> Self {
                    a.0 as $typ
                }
            }

            impl Add<$typ> for Myth64 {
                type Output = Myth64;

                fn add(self, rhs: $typ) -> Self::Output {
                    Self(self.0 + (rhs as i64))
                }
            }

            impl AddAssign<$typ> for Myth64 {
                fn add_assign(&mut self, rhs: $typ) {
                    self.0 += (rhs as i64);
                }
            }

            impl Sub<$typ> for Myth64 {
                type Output = Myth64;

                fn sub(self, rhs: $typ) -> Self::Output {
                    Self(self.0 - (rhs as i64))
                }
            }

            impl Mul<$typ> for Myth64 {
                type Output = Myth64;

                fn mul(self, rhs: $typ) -> Self::Output {
                    Self(self.0 * (rhs as i64))
                }
            }

            impl Div<$typ> for Myth64 {
                type Output = Myth64;

                fn div(self, rhs: $typ) -> Self::Output {
                    Self(self.0 / (rhs as i64))
                }
            }
        )+
    }
}

measure_from_number!(u64, u32, u16, u8, usize, i64, i32, i16, i8, isize);

impl From<Unit> for Myth64 {
    fn from(unit: Unit) -> Self {
        Myth64::from(unit.multiply())
    }
}

impl From<f64> for Myth64 {
    fn from(f: f64) -> Self {
        assert!(
            f < i64::MAX as f64 && f > i64::MIN as f64,
            "i64 overflow, the f64 is beyond the limits of this type (Myth64)."
        );
        Self((f * Myth64::MM.as_i64() as f64) as i64)
    }
}

impl From<Myth64> for f64 {
    fn from(f: Myth64) -> Self {
        f.0 as f64 / Myth64::MM.as_i64() as f64
    }
}

impl TryFrom<&str> for Myth64 {
    type Error = ParseFloatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Myth64::from(value.parse::<f64>()?))
    }
}

impl TryFrom<String> for Myth64 {
    type Error = ParseFloatError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Myth64::from(value.parse::<f64>()?))
    }
}

impl Display for Myth64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let p = f.precision().map_or(4, |p| p.min(4));
        assert!(p <= 4, "Myth64 has a limited precision of 4!");
        if f.alternate() {
            Display::fmt(&self.0, f)
        } else {
            let val = self.round(Unit::DYN(4 - p)).0;
            let l = if val.is_negative() { 6 } else { 5 };
            let mut s = format!("{val:0l$}");
            if p > 0 {
                s.insert(s.len() - 4, '.');
            }
            s.truncate(s.len() - (4 - p));
            write!(f, "{s}")
        }
    }
}

impl Debug for Myth64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = self.0;
        let n = if val.is_negative() { 6 } else { 5 };
        let mut m = format!("{val:0n$}");
        m.insert(m.len() - 4, '.');
        write!(f, "Myth64({m})")
    }
}

impl Neg for Myth64 {
    type Output = Myth64;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Myth64 {
    type Output = Myth64;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Myth32> for Myth64 {
    type Output = Myth64;

    fn add(self, rhs: Myth32) -> Self::Output {
        Self(self.0 + i64::from(rhs.as_i32()))
    }
}

impl AddAssign for Myth64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl AddAssign<Myth32> for Myth64 {
    fn add_assign(&mut self, rhs: Myth32) {
        self.0 += i64::from(rhs.as_i32());
    }
}

impl Sub for Myth64 {
    type Output = Myth64;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<Myth32> for Myth64 {
    type Output = Myth64;

    fn sub(self, rhs: Myth32) -> Self::Output {
        Self(self.0 - i64::from(rhs.as_i32()))
    }
}

impl SubAssign for Myth64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl SubAssign<Myth32> for Myth64 {
    fn sub_assign(&mut self, rhs: Myth32) {
        self.0 -= i64::from(rhs.as_i32());
    }
}

impl Mul for Myth64 {
    type Output = Myth64;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Myth64 {
    type Output = Myth64;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Deref for Myth64 {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for Myth64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Myth64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
mod should {
    use super::{Myth64, Ordering, Unit};

    #[test]
    fn cmp() {
        let s1 = Myth64(200_000);
        let i1 = Myth64(190_000);
        let s2 = Myth64::from(20.0);
        let i2 = Myth64::from(19.0);

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
    fn round() {
        let m = Myth64(1_234_567);
        assert_eq!(Myth64(1_234_570), m.round(Unit::MY));
        assert_eq!(Myth64(1_200_000), m.round(Unit::CM));
        assert_eq!(Myth64(10_000_000), Myth64(9_999_000).round(Unit::MM));
        assert_eq!(Myth64(0), Myth64::from(-0.4993).round(Unit::MM));
        assert_eq!(Myth64(-4990), Myth64::from(-0.4993).round(Unit::MY));
        assert_eq!(Myth64(-10000), Myth64::from(-5000).round(Unit::MM));
        let m = Myth64::from(340.993);
        assert_eq!(10, Unit::DYN(1).multiply());
        assert_eq!(Myth64(3_409_930), m.round(Unit::DYN(1)));
        assert_eq!(100, Unit::DYN(2).multiply());
        assert_eq!(Myth64(3_409_900), m.round(Unit::DYN(2)));
        assert_eq!(1000, Unit::DYN(3).multiply());
        assert_eq!(Myth64(3_410_000), m.round(Unit::DYN(3)));
        assert_eq!(Myth64(3_400_000), m.floor(Unit::DYN(4)));
        assert_eq!(-340.000, -(340.993_f64.floor()));
        assert_eq!(
            Myth64(-3_400_000),
            Myth64::from(-340.993).floor(Unit::DYN(4))
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
