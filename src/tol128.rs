#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Not, Sub};

use crate::error::ToleranceError::ParseError;
use crate::{error, Myth32, Myth64};

/// # `T128`
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
/// assert_eq!(format!("{width}"), "100.00 +0.050/-0.200");
/// assert_eq!(format!("{width:?}"), "T128(100.0000 +0.0500 -0.2000)");
/// ```
///
/// The `plus` and `minus` tolerances are in the same scale unit as the `value`.
/// `plus` is signed positiv (`+`) and `minus` is signed negative (`-`).
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct T128 {
    pub value: Myth64,
    pub plus: Myth32,
    pub minus: Myth32,
}

impl T128 {
    pub const ZERO: T128 = T128 {
        value: Myth64::ZERO,
        plus: Myth32::ZERO,
        minus: Myth32::ZERO,
    };

    /// Creates a `T128` with asymmetrical tolerances.
    ///
    /// Provided parameters as `f64` are interpreted as `mm`-values.
    ///
    #[inline]
    pub fn new(
        value: impl Into<Myth64>,
        plus: impl Into<Myth32>,
        minus: impl Into<Myth32>,
    ) -> Self {
        let plus = plus.into();
        let minus = minus.into();
        assert!(plus >= minus);
        Self {
            value: value.into(),
            plus,
            minus,
        }
    }

    /// Creates a `T128` with symmetrical tolerances.
    ///
    pub fn with_sym(value: impl Into<Myth64>, tol: impl Into<Myth32>) -> Self {
        let tol = tol.into();
        Self::new(value, tol, -tol)
    }

    /// narrows a `T128` to the given tolerances.
    ///
    pub fn narrow(&self, plus: impl Into<Myth32>, minus: impl Into<Myth32>) -> Self {
        Self::new(self.value, plus, minus)
    }

    /// narrows a `T128` to the given symmetric tolerances.
    ///
    pub fn narrow_sym(&self, tol: impl Into<Myth32>) -> Self {
        let tol = tol.into();
        Self::new(self.value, tol, -tol)
    }

    /// returns the maximum value of this tolerance.
    ///
    pub fn upper_limit(&self) -> Myth64 {
        self.value + self.plus
    }

    /// returns the minimum value of this tolerance.
    ///
    pub fn lower_limit(&self) -> Myth64 {
        self.value + self.minus
    }

    /// returns `true`, if `this` tolerance is more narrow than the `other`.
    ///
    #[must_use]
    pub fn is_inside_of(&self, other: Self) -> bool {
        self.lower_limit() >= other.lower_limit() && self.upper_limit() <= other.upper_limit()
    }

    /// returns `true`, if `this` tolerance is less strict (around) the `other`.
    ///
    pub fn embrace(&self, other: impl Into<T128>) -> bool {
        let other = other.into();
        self.lower_limit() <= other.lower_limit() && self.upper_limit() >= other.upper_limit()
    }

    /// Inverses this `T128`.
    /// Interchanges the `plus` and `minus` parts of this `T128`.
    /// Same as `!value`.
    pub fn invert(&self) -> Self {
        Self {
            value: -self.value,
            plus: -self.minus,
            minus: -self.plus,
        }
    }
}

/// Inverses this `T128`.
/// Interchanges the `plus` and `minus` parts of this `T128`.
/// Shortcut for the `T128.invert()`-method.
impl Not for T128 {
    type Output = T128;

    fn not(self) -> Self::Output {
        self.invert()
    }
}

impl Add<T128> for T128 {
    type Output = T128;

    fn add(self, other: T128) -> T128 {
        T128 {
            value: self.value + other.value,
            plus: self.plus + other.plus,
            minus: self.minus + other.minus,
        }
    }
}

impl Add<Myth64> for T128 {
    type Output = T128;

    fn add(self, other: Myth64) -> T128 {
        T128 {
            value: self.value + other,
            plus: self.plus,
            minus: self.minus,
        }
    }
}

impl AddAssign for T128 {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
        self.plus += other.plus;
        self.minus += other.minus;
    }
}

impl Sum for T128 {
    fn sum<I: Iterator<Item = T128>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl Sub<T128> for T128 {
    type Output = T128;

    fn sub(self, other: T128) -> T128 {
        T128 {
            value: self.value - other.value,
            plus: self.plus - other.minus,
            minus: self.minus - other.plus,
        }
    }
}

impl Sub<Myth64> for T128 {
    type Output = T128;

    fn sub(self, other: Myth64) -> T128 {
        T128 {
            value: self.value - other,
            plus: self.plus,
            minus: self.minus,
        }
    }
}

impl PartialOrd<T128> for T128 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(Ordering::Equal) => match self.minus.partial_cmp(&other.minus) {
                Some(Ordering::Equal) => self.plus.partial_cmp(&other.plus),
                x => x,
            },
            x => x,
        }
    }
}

impl Ord for T128 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => match self.minus.cmp(&other.minus) {
                Ordering::Equal => self.plus.cmp(&other.plus),
                x => x,
            },
            x => x,
        }
    }
}

impl Default for T128 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::fmt::Display for T128 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (v, t) = f.precision().map_or((2, 3), |p| (p, p + 1));
        let Self { value, plus, .. } = self;
        let minus = &-self.minus;
        if f.alternate() {
            return write!(f, "{value:#.v$} +{plus:#.t$}/-{minus:#.t$}");
        }
        if plus == minus {
            write!(f, "{value:.v$} +/-{plus:.t$}")
        } else {
            write!(f, "{value:.v$} +{plus:.t$}/-{minus:.t$}")
        }
    }
}

impl Debug for T128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T128({} +{} -{})", self.value, self.plus, -self.minus)
    }
}

/// A maybe harmful conversation. Ignores all possible tolerance.
/// Returns a f64 representing a mm value.    
impl From<T128> for f64 {
    fn from(v: T128) -> Self {
        v.value.as_mm()
    }
}

/// May be harmful
impl<V> From<V> for T128
where
    V: Into<Myth64>,
{
    fn from(f: V) -> Self {
        T128::new(f, 0, 0)
    }
}

impl<V, T> From<(V, T)> for T128
where
    V: Into<Myth64>,
    T: Into<Myth32>,
{
    fn from(v: (V, T)) -> Self {
        let t = v.1.into();
        T128::new(v.0, t, -t)
    }
}

impl<V, P, M> From<(V, P, M)> for T128
where
    V: Into<Myth64>,
    P: Into<Myth32>,
    M: Into<Myth32>,
{
    fn from(v: (V, P, M)) -> Self {
        T128::new(v.0, v.1, v.2)
    }
}

impl From<T128> for (f64, f64, f64) {
    fn from(v: T128) -> Self {
        (v.value.into(), v.plus.into(), v.minus.into())
    }
}

super::multiply_all!(T128, u64, u32, i64, i32);

impl<V, P, M> TryFrom<(Option<V>, Option<P>, Option<M>)> for T128
where
    V: Into<Myth64> + Debug,
    P: Into<Myth32> + Debug,
    M: Into<Myth32> + Debug,
{
    type Error = error::ToleranceError;

    fn try_from(triple: (Option<V>, Option<P>, Option<M>)) -> Result<Self, Self::Error> {
        match triple {
            (Some(v), Some(p), Some(m)) => Ok(T128::new(v, p, m)),
            (Some(v), Some(p), None) => {
                let p = p.into();
                Ok(T128::new(v, p, -p))
            }
            (Some(v), None, None) => Ok(T128::new(v, 0, 0)),
            _ => Err(ParseError(format!("T128 not parsable from '{triple:?}'"))),
        }
    }
}

impl TryFrom<(Option<&i64>, Option<&i64>, Option<&i64>)> for T128 {
    type Error = error::ToleranceError;

    fn try_from(triple: (Option<&i64>, Option<&i64>, Option<&i64>)) -> Result<Self, Self::Error> {
        match triple {
            (Some(&v), Some(&p), Some(&m)) => Ok(T128::new(v, p as i32, m as i32)),
            (Some(&v), Some(&p), None) => Ok(T128::new(v, p as i32, -p as i32)),
            (Some(&v), None, None) => Ok(T128::new(v, 0, 0)),
            _ => Err(ParseError(format!("T128 not parsable from '{triple:?}'"))),
        }
    }
}

impl TryFrom<&str> for T128 {
    type Error = error::ToleranceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::from(super::try_from_str(value.trim())?))
    }
}

impl TryFrom<String> for T128 {
    type Error = error::ToleranceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::from(super::try_from_str(value.trim())?))
    }
}

impl TryFrom<&[i64]> for T128 {
    type Error = error::ToleranceError;

    fn try_from(value: &[i64]) -> Result<Self, Self::Error> {
        let mut iter = value.iter();
        Self::try_from((iter.next(), iter.next(), iter.next()))
    }
}

#[cfg(test)]
mod should {
    use super::T128;
    use crate::error::ToleranceError;
    use std::convert::TryFrom;

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
    fn display_is_adjustible() {
        let o = T128::new(20_000, 50, -100);
        assert_eq!(format!("{o}"), String::from("2.00 +0.005/-0.010"));
        assert_eq!(format!("{o:.3}"), "2.000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.4}"), "2.0000 +0.0050/-0.0100".to_string());
        assert_eq!(format!("{o:.0}"), String::from("2 +0.0/-0.0"));
        assert_eq!(format!("{o:.1}"), String::from("2.0 +0.01/-0.01"));

        let o = T128::with_sym(20_000, 50);
        assert_eq!(format!("{o}"), String::from("2.00 +/-0.005"));
        assert_eq!(format!("{o:.0}"), String::from("2 +/-0.0"));

        let o = T128::new(0.345, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("0.345 +0.0100/-0.0140"));
        let o = T128::new(-0.35, 0.010, -0.014);
        assert_eq!(format!("{o:.3}"), String::from("-0.350 +0.0100/-0.0140"));

        assert_eq!(format!("{o:#}"), String::from("-3500 +100/-140"));

        assert_eq!(
            format!("{o:.3?}"),
            String::from("T128(-0.3500 +0.0100 -0.0140)")
        );
    }

    #[test]
    fn construct_consistent() {
        let o = T128::from((2.0, 0.005, -0.01));
        assert_eq!(o.to_string(), "2.00 +0.005/-0.010".to_string())
    }

    #[test]
    fn substract() {
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
            ToleranceError::parse_err("cannot parse Tolerance found non-numerical literal")
        );

        let tol = T128::try_from("");
        assert!(tol.is_err(), "T128 ");
        assert_eq!(
            tol,
            ToleranceError::parse_err("cannot parse Tolerance from empty string")
        );
    }

    #[cfg(feature = "serde")]
    mod serde {
        use crate::T128;
        use serde_test::{assert_tokens, Token};

        #[test]
        fn serialize() {
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
    }
}
