pub(crate) mod myth16;
pub(crate) mod myth32;
pub(crate) mod myth64;

macro_rules! from_number {
    ($Self:ident, $($Target:ident),+) => {
        $(
            impl From<$Target> for $Self {
                fn from(a: $Target) -> Self {
                    Self(a.into())
                }
            }

            impl From<$Self> for $Target {
                fn from(a: $Self) -> Self {
                    a.0 as $Target
                }
            }

            impl <'a> From<&'a $Self> for $Target {
                fn from(a: &$Self) -> Self {
                    a.0 as $Target
                }
            }
        )+
    }
}

macro_rules! from_myths {
    ($Self:ident, $($Target:ident),+) => {
        $(
            impl From<$Target> for $Self {
                fn from(m: $Target) -> Self {
                    Self::from(m.0)
                }
            }
        )+
    }
}

macro_rules! try_from_number {
    ($Self:ident, $($Target:ident),+) => {
        $(
            impl TryFrom<$Target> for $Self {
                type Error = ToleranceError;

                fn try_from(value: $Target) -> Result<Self, Self::Error> {
                    Ok(Self(value.try_into()?))
                }
            }
        )+
    }
}

macro_rules! try_from_myths {
    ($Self:ident, $($Target:ident),+) => {
        $(
            impl TryFrom<$Target> for $Self {
                type Error = ToleranceError;

                #[allow(clippy::needless_question_mark)]
                fn try_from(value: $Target) -> Result<Self, Self::Error> {
                    Ok(Self::try_from(value.0)?)
                }
            }
        )+
    }
}

macro_rules! standard_myths {
    ($Self:ident, $typ:ident, $($Target:ident),+) => {
        $(
            impl Mul<$Target> for $Self {
                type Output = $Self;

                fn mul(self, other: $Target) -> Self::Output {
                    Self(self.0 * (other as $typ))
                }
            }

            impl <'a> Mul<$Target> for &'a $Self {
                type Output = $Self;

                fn mul(self, other: $Target) -> Self::Output {
                    $Self(self.0 * (other as $typ))
                }
            }

            impl Mul<$Self> for $Target {
                type Output = $Self;

                fn mul(self, other: $Self) -> Self::Output {
                    $Self(self as $typ * other.0)
                }
            }

            impl Div<$Target> for $Self {
                type Output = $Self;

                fn div(self, other: $Target) -> Self::Output {
                    Self(self.0 / (other as $typ))
                }
            }

            impl <'a> Div<$Target> for &'a $Self {
                type Output = $Self;

                fn div(self, other: $Target) -> Self::Output {
                    $Self(self.0 / (other as $typ))
                }
            }

            impl Add<$Target> for $Self {
                type Output = $Self;

                fn add(self, other: $Target) -> Self::Output {
                    $Self::from(self.0 + $typ::try_from(other).expect("Addend out of scope"))
                }
            }

            impl Sub<$Target> for $Self {
                type Output = $Self;

                fn sub(self, other: $Target) -> Self::Output {
                    $Self::from(self.0 - $typ::try_from(other).expect("Addend out of scope"))
                }
            }

        )+

        impl $Self {

            /// The neutral element in relation to multiplication and division.
            pub const ONE: $Self = $Self(10_000);
            /// The neutral element in relation to addition and subtraction.
            pub const ZERO: $Self = $Self(0);

            pub const MIN: $Self = $Self($typ::MIN);
            pub const MAX: $Self = $Self($typ::MAX);

            // --- deprecated
            #[deprecated(since="1.0.3", note="please use [`ONE`](#associatedconstant.ONE) instead.")]
            pub const MM: $Self = $Self(10_000);

            /// Returns the value as a `i64` in "1/10 μ".
            #[must_use]
            pub const fn as_i64(&self) -> i64 {
                self.0 as i64
            }

            /// Returns the value as a `f64` in "mm".
            #[inline]
            #[must_use]
            #[deprecated(since="1.0.3", note="please use [`as_f64`](#method.as_f64) instead.")]
            pub fn as_mm(&self) -> f64 {
                self.as_f64()
            }

            /// Returns the value as a `f64` in "mm".
            #[inline]
            #[must_use]
            pub fn as_f64(&self) -> f64 {
                self.0 as f64 / $Self::ONE.0 as f64
            }

            /// Returns the value in the given `Unit`.
            #[must_use]
            pub fn as_unit(&self, unit: Unit) -> f64 {
                self.0 as f64 / *unit as f64
            }

            /// Rounds to the given Unit.
            pub fn round(&self, unit: Unit) -> Self {
                if *unit == 0 {
                    return *self;
                }
                let m = $typ::try_from(unit).expect("Unit.multiply to big.");
                let clip = self.0 % m;
                match m / 2 {
                    _ if clip == 0 => *self, // don't round
                    x if clip <= -x => Self(self.0 - clip - m),
                    x if clip >= x => Self(self.0 - clip + m),
                    _ => Self(self.0 - clip),
                }
            }

            /// Finds the nearest value less than or equal to an integer multiple of the given `Unit`.
            pub fn floor(&self, unit: Unit) -> Self {
                let val = self.0;
                let m = $typ::try_from(*unit).expect("Unit.multiply to big.");
                let clip = val % m;
                if val < 0 {
                    Self(val - clip - m)
                } else {
                    Self(val - clip)}
            }

            /// Computes the absolute value of self.
            pub const fn abs(&self) -> Self {
                if self.0 < 0 {
                    Self(-self.0)
                } else {
                    *self
                }
            }

            /// Computes the absolute difference between `self` and `other`.
            pub const fn abs_diff(self, other: $Self) -> Self {
                Self(self.0 - other.0).abs()
            }

            #[doc = concat!("Returns a ", stringify!($Self) ," representing the sign of self.")]
            ///
            ///   *  0 if the number is zero
            ///   *  1 if the number is positive
            ///   *  -1 if the number is negative
            pub const fn signum(self) -> Self {
                if self.is_negative() {
                    Self(-1)
                } else if self.is_positive() {
                    Self(1)
                } else {
                    Self::ZERO
                }
            }

            /// Returns `true` if `self` is negative and `false` if zero or positive.
            #[must_use]
            pub const fn is_negative(&self) -> bool {
                self.0 < 0
            }

            /// Returns `true` if `self` is positive and `false` if zero or negative.
            #[must_use]
            pub const fn is_positive(&self) -> bool {
                self.0 > 0
            }

            /// Returns `true` if `self` is zero.
            #[must_use]
            pub const fn is_zero(&self) -> bool {
                self.0 == 0
            }

            #[doc = concat!("Returns the memory representation of this ", stringify!($Self), " as a byte array in")]
            /// big-endian (network) byte order.
            #[must_use]
            pub fn to_be_bytes(&self) -> [u8; std::mem::size_of::<$typ>()] {
                $typ::to_be_bytes(self.0)
            }

            #[doc = concat!("Returns the memory representation of this ", stringify!($Self), " as a byte array in")]
            /// little-endian byte order.
            #[must_use]
            pub fn to_le_bytes(&self) -> [u8; std::mem::size_of::<$typ>()] {
                $typ::to_le_bytes(self.0)
            }

            #[doc = concat!("Returns the memory representation of this ", stringify!($Self), " as a byte array in")]
            /// native byte order.
            #[must_use]
            pub fn to_ne_bytes(&self) -> [u8; std::mem::size_of::<$typ>()] {
                $typ::to_ne_bytes(self.0)
            }

            #[doc = concat!("Creates a ", stringify!($Self), " value from its representation")]
            /// as a byte array in big-endian.
            pub fn from_be_bytes(bytes: [u8; std::mem::size_of::<$typ>()]) -> Self {
                Self($typ::from_be_bytes(bytes))
            }

            #[doc = concat!("Creates a ", stringify!($Self), " value from its representation")]
            /// as a byte array in little endian.
            pub fn from_le_bytes(bytes: [u8; std::mem::size_of::<$typ>()]) -> Self {
                Self($typ::from_le_bytes(bytes))
            }

            #[doc = concat!("Creates a ", stringify!($Self), " value from its representation")]
            /// as a byte array in native byte order.
            pub fn from_ne_bytes(bytes: [u8; std::mem::size_of::<$typ>()]) -> Self {
                Self($typ::from_ne_bytes(bytes))
            }

        }

        impl Debug for $Self {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let val = self.0;
                let n = if val.is_negative() { 6 } else { 5 };
                let mut m = format!("{val:0n$}");
                m.insert(m.len() - 4, '.');
                write!(f, "{}({m})", stringify!($Self))
            }
        }

        impl Display for $Self {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let v = self.0;
                let p = f.precision().map_or(if v % 1000 == 0 { 1 } else
                    if v % 100 == 0 { 2 } else
                    if v % 10 == 0 { 3 } else
                    { 4 }, |p| p.min(4));
                assert!(p <= 4, "{} has a limited precision of 4!", stringify!($Self));
                if f.alternate() {
                    Display::fmt(&self.0, f)
                } else {
                    let val = self.round(Unit::potency(4 - p)).0;
                    let l = if val.is_negative() || f.sign_plus() { 6 } else { 5 };
                    let mut s = if f.sign_plus() { format!("{val:+0l$}") } else { format!("{val:0l$}") };
                    if p > 0 {
                        s.insert(s.len() - 4, '.');
                    }
                    s.truncate(s.len() - (4 - p));
                    write!(f, "{s}")
                }
            }
        }

        impl TryFrom<&str> for $Self {
            type Error = ToleranceError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                $Self::from_str(value)
            }
        }

        impl TryFrom<String> for $Self {
            type Error = ToleranceError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                $Self::from_str(value.as_str())
            }
        }

        impl std::str::FromStr for $Self {
            type Err = ToleranceError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                crate::try_from_str(value.trim(), &stringify!($Self))
                .and_then(|i| Self::try_from(i).
                    map_err(|_| ToleranceError::Overflow(format!("{value} is to big for {}", stringify!($Self))))
                )
            }
        }

        #[allow(clippy::cast_possible_truncation)]
        impl From<f64> for $Self {
            fn from(f: f64) -> Self {
                assert!(
                    f < $typ::MAX as f64 && f > $typ::MIN as f64,
                    "{} overflow, the f64 '{f:?}' is beyond the limits of this type ({}).",
                    stringify!($typ),
                    stringify!($Self),
                );
                Self((f * 10_000.0) as $typ)
            }
        }

        impl From<$Self> for f64 {
            fn from(f: $Self) -> Self {
                f.0 as f64 / 10_000.0
            }
        }

        impl From<Unit> for $Self {
            fn from(unit: Unit) -> Self {
                $Self::try_from(*unit).expect("Unit out of scope")
            }
        }

        impl Neg for $Self {
            type Output = $Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl <'a> Neg for &'a $Self {
            type Output = $Self;

            fn neg(self) -> Self::Output {
                $Self(-self.0)
            }
        }
    }
}

macro_rules! calc_with_myths {
    ($Self:ident, $typ:ident, $($Target:ident),+) => {
        $(
            impl Add<$Target> for $Self {
                type Output = $Self;

                fn add(self, other: $Target) -> Self::Output {
                    $Self::from(self.0 + $typ::try_from(other.0).expect("Addend out of scope"))
                }
            }

            impl <'a> Add<&'a $Target> for $Self {
                type Output = $Self;

                fn add(self, other: &'a $Target) -> Self::Output {
                    $Self::from(self.0 + $typ::try_from(other.0).expect("Addend out of scope"))
                }
            }

            impl Sub<$Target> for $Self {
                type Output = $Self;

                fn sub(self, other: $Target) -> Self::Output {
                    $Self::from(self.0 - $typ::try_from(other.0).expect("Minuend out of scope"))
                }
            }

            impl <'a> Sub<&'a $Target> for $Self {
                type Output = $Self;

                fn sub(self, other: &'a $Target) -> Self::Output {
                    $Self::from(self.0 - $typ::try_from(other.0).expect("Minuend out of scope"))
                }
            }
        )+

        impl AddAssign for $Self {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl SubAssign for $Self {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }

        impl std::iter::Sum for $Self {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Add::add)
            }
        }

        impl<'a> std::iter::Sum<&'a $Self> for $Self {
            fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    Self::ZERO,
                    |a, b| a + b,
                )
            }
        }
    }
}

#[cfg(feature = "serde")]
macro_rules! de_serde {
    ($Self:ident, $typ:ident) => {
        impl<'de> Deserialize<'de> for $Self {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct MythVisitor;

                impl<'de> Visitor<'de> for MythVisitor {
                    type Value = $Self;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a float, string or integer!")
                    }

                    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        $Self::try_from(v).map_err(|_| {
                            serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &"1.0")
                        })
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        self.visit_borrowed_str(s)
                    }

                    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        self.visit_borrowed_str(s.as_str())
                    }

                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        $Self::try_from(v).map_err(|_| {
                            serde::de::Error::invalid_value(serde::de::Unexpected::Float(v), &"1.0")
                        })
                    }

                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($Self(v as $typ))
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($Self(v as $typ))
                    }

                    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($Self(v as $typ))
                    }

                    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($Self(v as $typ))
                    }

                    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        deserializer.deserialize_any(MythVisitor)
                    }

                    fn visit_newtype_struct<D>(
                        self,
                        deserializer: D,
                    ) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        deserializer.deserialize_any(MythVisitor)
                    }
                }
                deserializer.deserialize_any(MythVisitor)
            }
        }
    };
}

pub(crate) use calc_with_myths;
#[cfg(feature = "serde")]
pub(crate) use de_serde;
pub(crate) use from_myths;
pub(crate) use from_number;
pub(crate) use standard_myths;
pub(crate) use try_from_myths;
pub(crate) use try_from_number;
