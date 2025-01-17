use std::ops::{Deref, Mul};

/// # Unit-conversation helper.
///
/// This `Unit` is used to translate the [Myth64](./struct.Myth64.html),
/// and [Myth32](./struct.Myth32.html) and [Myth16](./struct.Myth16.html)-types into other
/// length-units.
///
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Unit(i64);

impl Unit {
    /// My-meter `μ` the equivalent to `potency(1)`.
    pub const MY: Self = Self(10);

    /// Millimeter `1 mm = 1000 μ` the equivalent to `potency(4)`.
    pub const MM: Self = Self(1_000 * Self::MY.0);

    /// Centimeter `1 cm = 10 mm = 10_000 μ` the equivalent to `potency(5)`.
    pub const CM: Self = Self(10 * Self::MM.0);

    /// Inch `1 in = 25.4 mm = 25_400 μ`.
    pub const INCH: Self = Self(25_400 * Self::MY.0);

    /// Foot `1 ft = 12 in = 304.8 mm = 304_800 μ`.
    pub const FT: Self = Self(12 * Self::INCH.0);

    /// Yard `1 yd = 3 ft = 914.4 mm = 914_400 μ`.
    pub const YD: Self = Self(3 * Self::FT.0);

    /// Meter `100 cm = 1_000 mm = 1_000_000 μ` the equivalent to `potency(7)`.
    pub const METER: Self = Self(1_000 * Self::MM.0);

    /// Kilometer `1 km = 1_000 m` the equivalent to `potency(10)`.
    pub const KM: Self = Self(1_000 * Self::METER.0);

    /// Mile `1 mi = 1760 yd = 1609.344 m = 1_609_344_000 μ`.
    pub const MILE: Self = Self(1760 * Self::YD.0);
}

impl Unit {
    #[inline]
    #[must_use]
    pub const fn multiply(&self) -> i64 {
        self.0
    }

    /// ten to the power of `p`. `10^p`
    #[must_use]
    pub fn potency(p: usize) -> Self {
        Self((0..p).fold(1i64, |acc, _| acc * 10))
    }
}

impl Deref for Unit {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! unit_from_number {
    ($($typ:ident),+) => {
        $(
            impl Mul<$typ> for Unit {
                type Output = Unit;

                fn mul(self, rhs: $typ) -> Self::Output {
                    Unit(self.0 * rhs as i64)
                }
            }

            impl Mul<Unit> for $typ {
                type Output = $typ;

                fn mul(self, rhs: Unit) -> Self::Output {
                    self * rhs.0 as $typ
                }
            }

            impl From<Unit> for $typ {
                fn from(value: Unit) -> Self {
                    value.0 as $typ
                }
            }
        )+
    }
}

unit_from_number!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[cfg(test)]
mod should {
    use super::Unit;

    #[test]
    fn multiply_with_number() {
        assert_eq!(30_000, 3 * Unit::MM);
        assert_eq!(550_000_000, 55 * Unit::METER);
    }

    #[test]
    fn be_equal_dyn() {
        assert_eq!(Unit::MY, Unit::potency(1));
        assert_eq!(Unit::MM, Unit::potency(4));
        assert_eq!(Unit::potency(7), Unit::METER);
    }

    #[test]
    fn be_const() {
        assert_eq!(3_048_000, Unit::FT.0);
        assert_eq!(9_144_000, Unit::YD.0);
        assert_eq!(16_093_440_000, Unit::MILE.0);
    }
}
