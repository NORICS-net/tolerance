//!
//! # Tolerance
//!
//! Math representation of the physically needed permissible deviation of measures in Rust
//! avoiding floating point inaccuracy. Allows to calculate with tolerance ranges in a
//! consistent way.
//!
//! Based on `Myth`-types with a accuracy of 1/10th my-meter (= 0.1μ) as the name suggests.
//!
//! ### Example
//!
//! ```rust
//! use tolerance::T128;
//!
//! let width1 = T128::new(100.0, 0.05, -0.2);
//! let width2 = T128::with_sym(50.0, 0.05);
//!
//! // Adding two `T128`s is straightforward.
//! assert_eq!(width1 + width2, T128::new(150.0, 0.1, -0.25));
//!
//! // `!` inverts the direction of tolerance to /subtract/ measures.
//! assert_eq!(!width1, T128::new(-100.0, 0.2, -0.05));
//!
//! // Adding an inverted `T128` wides the tolerance.
//! assert_eq!(width1 + !width1, T128::new(0.0, 0.25, -0.25));
//! ```
extern crate core;

pub mod error;
mod myth16;
mod myth32;
mod myth64;
mod tol128;
mod tol64;
mod unit;

pub use self::myth16::*;
pub use self::myth32::*;
pub use self::myth64::*;
pub use self::tol128::*;
pub use self::tol64::*;
pub use self::unit::*;

use crate::error::ToleranceError;

macro_rules! from_number {
    ($class:ident, $($target:ident),+) => {
        $(
            impl From<$target> for $class {
                fn from(a: $target) -> Self {
                    Self(a.into())
                }
            }

            impl From<$class> for $target {
                fn from(a: $class) -> Self {
                    a.0 as $target
                }
            }
        )+
    }
}

macro_rules! try_from_number {
    ($class:ident, $($target:ident),+) => {
        $(
            impl TryFrom<$target> for $class {
                type Error = ToleranceError;

                fn try_from(value: $target) -> Result<Self, Self::Error> {
                    Ok(Self(value.try_into()?))
                }
            }
        )+
    }
}

macro_rules! standard_myths {
    ($class:ident, $typ:ident, $($target:ident),+) => {
        $(
            impl Add<$target> for $class {
                type Output = $class;

                fn add(self, rhs: $target) -> Self::Output {
                    Self(self.0 + (rhs as $typ))
                }
            }

            impl AddAssign<$target> for $class {
                fn add_assign(&mut self, rhs: $target) {
                    self.0 += (rhs as $typ);
                }
            }

            impl Sub<$target> for $class {
                type Output = $class;

                fn sub(self, rhs: $target) -> Self::Output {
                    Self(self.0 - (rhs as $typ))
                }
            }

            impl Mul<$target> for $class {
                type Output = $class;

                fn mul(self, rhs: $target) -> Self::Output {
                    Self(self.0 * (rhs as $typ))
                }
            }

            impl Div<$target> for $class {
                type Output = $class;

                fn div(self, rhs: $target) -> Self::Output {
                    Self(self.0 / (rhs as $typ))
                }
            }
        )+

        impl $class {
            #[must_use]
            pub const fn as_i64(&self) -> i64 {
                self.0 as i64
            }

            #[inline]
            #[must_use]
            pub fn as_mm(&self) -> f64 {
                self.0 as f64 / *Unit::MM as f64
            }

            /// Returns the value in the given `Unit`.
            #[must_use]
            pub fn as_unit(&self, unit: Unit) -> f64 {
                self.0 as f64 / *unit as f64
            }

            /// Rounds to the given Unit.
            pub fn round(&self, unit: Unit) -> $class {
                let unit = *unit;
                if unit == 0 {
                    return *self;
                }
                let m = $typ::try_from(unit).expect("Unit.multiply to big.");
                let clip = self.0 % m;
                match m / 2 {
                    _ if clip == 0 => *self, // don't round
                    x if clip <= -x => Self($typ::from(self.0) - clip - m),
                    x if clip >= x => Self($typ::from(self.0) - clip + m),
                    _ => Self(self.0 - clip as $typ),
                }
            }

            /// Finds the nearest integer less than or equal to x at the given `Unit`.
            pub fn floor(&self, unit: Unit) -> $class {
                let val = self.0;
                let m = $typ::try_from(*unit).expect("Unit.multiply to big.");
                let clip = val % m;
                Self(val - clip)
            }

            /// Computes the absolute value of self.
            pub const fn abs(&self) -> $class {
                if self.0 < 0 {
                    Self(-self.0)
                } else {
                    *self
                }
            }

            /// Computes the absolute difference between `self` and `other`.
            pub const fn abs_diff(self, other: $class) -> $class {
                Self(self.0 - other.0).abs()
            }

            /// Returns a number representing sign of self.
            ///
            ///   *  0 if the number is zero
            ///   *  1 if the number is positive
            ///   *  -1 if the number is negative
            pub const fn signum(self) -> $class {
                if self.is_negative() {
                    Self(-1)
                } else if self.is_positive() {
                    Self(1)
                } else {
                    Self::ZERO
                }
            }

            /// Returns `true` if `self` is negative and `false` if the number is zero or positive.
            #[must_use]
            pub const fn is_negative(&self) -> bool {
                self.0 < 0
            }

            /// Returns `true` if `self` is positive and `false` if the number is zero or negative.
            #[must_use]
            pub const fn is_positive(&self) -> bool {
                self.0 > 0
            }

        }

        impl Debug for $class {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let val = self.0;
                let n = if val.is_negative() { 6 } else { 5 };
                let mut m = format!("{val:0n$}");
                m.insert(m.len() - 4, '.');
                write!(f, "{}({m})", stringify!($class))
            }
        }

        impl Display for $class {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let p = f.precision().map_or(4, |p| p.min(4));
                assert!(p <= 4, "{} has a limited precision of 4!", stringify!($class));
                if f.alternate() {
                    Display::fmt(&self.0, f)
                } else {
                    let val = self.round(Unit::potency(4 - p)).0;
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

        impl From<f64> for $class {
            fn from(f: f64) -> Self {
                assert!(
                    f < $typ::MAX as f64 && f > $typ::MIN as f64,
                    "{} overflow, the f64 is beyond the limits of this type ({}).",
                    stringify!($typ),
                    stringify!($class),
                );
                Self((f * 10_000.0) as $typ)
            }
        }

        impl From<$class> for f64 {
            fn from(f: $class) -> Self {
                f.0 as f64 / 10_000.0
            }
        }

        impl From<Unit> for $class {
            fn from(unit: Unit) -> Self {
                $class::try_from(*unit).expect("Unit out of scope")
            }
        }

        impl Neg for $class {
            type Output = $class;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl Add<Myth64> for $class {
            type Output = $class;

            fn add(self, rhs: Myth64) -> Self::Output {
                $class::from(self.0 + $typ::try_from(rhs.as_i64()).expect("Addend out of scope"))
            }
        }

        impl Add<Myth32> for $class {
            type Output = $class;

            fn add(self, rhs: Myth32) -> Self::Output {
                $class::from(self.0 + $typ::try_from(rhs.as_i32()).expect("Addend out of scope"))
            }
        }

        impl Add<Myth16> for $class {
            type Output = $class;

            fn add(self, rhs: Myth16) -> Self::Output {
                $class::from(self.0 + $typ::try_from(rhs.as_i16()).expect("Addend out of scope"))
            }
        }

        impl AddAssign for $class {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl Sub<Myth64> for $class {
            type Output = $class;

            fn sub(self, rhs: Myth64) -> Self::Output {
                $class::from(self.0 - $typ::try_from(rhs.as_i64()).expect("Minuend out of scope"))
            }
        }

        impl Sub<Myth32> for $class {
            type Output = $class;

            fn sub(self, rhs: Myth32) -> Self::Output {
                $class::from(self.0 - $typ::try_from(rhs.as_i32()).expect("Minuend out of scope"))
            }
        }

        impl Sub<Myth16> for $class {
            type Output = $class;

            fn sub(self, rhs: Myth16) -> Self::Output {
                $class::from(self.0 - $typ::try_from(rhs.as_i16()).expect("Minuend out of scope"))
            }
        }

        impl SubAssign for $class {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl Mul for $class {
            type Output = $class;

            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }

        impl Div for $class {
            type Output = $class;

            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }

    }
}

#[inline]
fn str2int(bytes: &[u8]) -> Result<i64, ToleranceError> {
    let mut v = 0i64;
    for c in bytes {
        match c {
            0x30..=0x39 => v = v * 10 + i64::from(c - 0x30),
            _ => {
                return ToleranceError::parse_err(
                    "Cannot parse Tolerance found non-numerical literal",
                )
            }
        }
    }
    Ok(v)
}

#[inline]
pub(crate) fn try_from_str(value: &str) -> Result<i64, ToleranceError> {
    const ERROR_TEXT: &str = "Cannot parse Tolerance from empty string";
    let (base, fraction) = value.split_once('.').unwrap_or((value, "0"));
    let mut base = base.as_bytes();
    let Some(&c) = base.first() else {
        return ToleranceError::parse_err(ERROR_TEXT);
    };
    let sign = 1 - i64::from(c == b'-') * 2;
    if c == b'-' || c == b'+' {
        base = &base[1..];
    }
    if base.is_empty() {
        return ToleranceError::parse_err(ERROR_TEXT);
    }
    let fraction = fraction.to_string() + "00000";
    let fraction = fraction.split_at(4).0.as_bytes();
    Ok((str2int(base)? * Myth64::MM.as_i64() + str2int(fraction)?) * sign)
}

macro_rules! multiply_tolerance {
    ($class:ident, $($typ:ty),+) => {

        $(impl Mul<$typ> for $class {
            type Output = Self;
            fn mul(self, rsh: $typ) -> Self {
                $class {
                    value: self.value * rsh,
                    plus: self.plus * rsh,
                    minus: self.minus * rsh,
                }
            }
        })+
    };
}

pub(crate) use {from_number, multiply_tolerance, standard_myths, try_from_number};
