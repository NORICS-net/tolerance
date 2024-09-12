pub(crate) mod tol128;
pub(crate) mod tol64;

macro_rules! multiply_tolerance {
    ($Self:ident, $($typ:ty),+) => {

        $(impl Mul<$typ> for $Self {
            type Output = Self;
            fn mul(self, rsh: $typ) -> Self {
                $Self {
                    value: self.value * rsh,
                    plus: self.plus * rsh,
                    minus: self.minus * rsh,
                }
            }
        })+
    };
}

pub(crate) use multiply_tolerance;

macro_rules! tolerance_body {
    ($Self:ident, $value:ident, $tol:ident) => {

        impl $Self {
            /// The neutral element in relation to addition and subtraction
            pub const ZERO: $Self = $Self {
                value: $value::ZERO,
                plus: $tol::ZERO,
                minus: $tol::ZERO,
            };

            ///
            #[doc = concat!("Creates a `", stringify!($Self), "` with asymmetrical tolerance.")]
            ///
            /// Provided parameters as `f64` are interpreted as `mm`-values.
            ///
            #[inline]
            pub fn new(
                value: impl Into<$value>,
                plus: impl Into<$tol>,
                minus: impl Into<$tol>,
            ) -> Self {
                let plus = plus.into();
                let minus = minus.into();
                assert!(plus >= minus, "Plus has to be bigger than minus.");
                Self {
                    value: value.into(),
                    plus,
                    minus,
                }
            }

            #[doc = concat!("Creates a `", stringify!($Self), "` with symmetrical tolerance.")]
            pub fn with_sym(value: impl Into<$value>, tol: impl Into<$tol>) -> Self {
                let tol = tol.into();
                Self::new(value, tol, -tol)
            }

            #[doc = concat!("Narrows a `", stringify!($Self), "` to the given tolerance.")]
            pub fn narrow(&self, plus: impl Into<$tol>, minus: impl Into<$tol>) -> Self {
                Self::new(self.value, plus, minus)
            }

            #[doc = concat!("Narrows a `", stringify!($Self), "` to the given symmetric tolerance.")]
            pub fn narrow_sym(&self, tol: impl Into<$tol>) -> Self {
                let tol = tol.into();
                Self::new(self.value, tol, -tol)
            }

            /// Returns the maximum value of this tolerance.
            ///
            pub fn upper_limit(&self) -> $value {
                self.value + self.plus
            }

            /// Returns the minimum value of this tolerance.
            ///
            pub fn lower_limit(&self) -> $value {
                self.value + self.minus
            }

            /// Returns `true`, if `this` tolerance is more narrow than the `other`.
            ///
            #[must_use]
            pub fn is_inside_of(&self, other: Self) -> bool {
                self.lower_limit() >= other.lower_limit()
                    && self.upper_limit() <= other.upper_limit()
            }

            /// Returns `true`, if `this` tolerance is less strict (around) the `other`.
            ///
            pub fn enfold(&self, other: impl Into<$Self>) -> bool {
                let other = other.into();
                self.lower_limit() <= other.lower_limit()
                    && self.upper_limit() >= other.upper_limit()
            }

            #[doc = concat!("Inverts this `", stringify!($Self), "`.")]
            /// Interchanges the `plus` and `minus` parts.
            /// Required when measuring back in the opposite direction.
            #[doc = concat!("Same as [`!value`](#impl-Not-for-", stringify!($Self), ").")]
            pub fn invert(&self) -> Self {
                Self {
                    value: -self.value,
                    plus: -self.minus,
                    minus: -self.plus,
                }
            }
        }

        #[doc = concat!("Inverts this `", stringify!($Self), "`.")]
        /// Interchanges the `plus` and `minus` parts.
        /// Required when measuring back in the opposite direction.
        #[doc = concat!("Shortcut for the [`", stringify!($Self), ".invert()`](#method.invert)-method.")]
        impl Not for $Self {
            type Output = $Self;

            fn not(self) -> Self::Output {
                self.invert()
            }
        }

        impl Not for &$Self {
            type Output = $Self;

            fn not(self) -> Self::Output {
                self.invert()
            }
        }

        #[doc = concat!("Inverses this `", stringify!($Self), "` as a whole.")]
        /// Like multiplying by `-1`.
        impl Neg for $Self {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new(-self.value, -self.plus, -self.minus)
            }
        }

        impl <'a> Neg for &'a $Self {
            type Output = $Self;

            fn neg(self) -> Self::Output {
                $Self::new(-self.value, -self.plus, -self.minus)
            }
        }

        impl Add<$Self> for $Self {
            type Output = $Self;

            fn add(self, other: $Self) -> $Self {
                $Self {
                    value: self.value + other.value,
                    plus: self.plus + other.plus,
                    minus: self.minus + other.minus,
                }
            }
        }

        impl <'a> Add<&'a $Self> for $Self {
            type Output = $Self;

            fn add(self, other: &'a $Self) -> $Self {
                $Self {
                    value: self.value + other.value,
                    plus: self.plus + other.plus,
                    minus: self.minus + other.minus,
                }
            }
        }

        impl Add<$value> for $Self {
            type Output = $Self;

            fn add(self, other: $value) -> $Self {
                $Self {
                    value: self.value + other,
                    plus: self.plus,
                    minus: self.minus,
                }
            }
        }

        impl AddAssign for $Self {
            fn add_assign(&mut self, other: Self) {
                self.value += other.value;
                self.plus += other.plus;
                self.minus += other.minus;
            }
        }

        impl Sum for $Self {
            fn sum<I: Iterator<Item = $Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Add::add)
            }
        }

        impl<'a> Sum<&'a $Self> for $Self {
            fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    Self::ZERO,
                    |a, b| a + b,
                )
            }
        }

        impl Sub<$Self> for $Self {
            type Output = $Self;

            fn sub(self, other: $Self) -> $Self {
                $Self {
                    value: self.value - other.value,
                    plus: self.plus - other.minus,
                    minus: self.minus - other.plus,
                }
            }
        }

        impl <'a> Sub<&'a $Self> for $Self {
            type Output = $Self;

            fn sub(self, other: &'a $Self) -> $Self {
                $Self {
                    value: self.value - other.value,
                    plus: self.plus - other.minus,
                    minus: self.minus - other.plus,
                }
            }
        }

        impl Sub<$value> for $Self {
            type Output = $Self;

            fn sub(self, other: $value) -> $Self {
                $Self {
                    value: self.value - other,
                    plus: self.plus,
                    minus: self.minus,
                }
            }
        }

        impl PartialOrd for $Self {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $Self {
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

        impl Default for $Self {
            fn default() -> Self {
                Self::ZERO
            }
        }

        impl std::fmt::Display for $Self {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let (v, t) = f.precision().map_or((2, 3), |p| (p, p + 1));
                let Self { value, plus, minus } = self;
                if f.alternate() {
                    if minus.0 == 0 {
                        return write!(f, "{value:#.v$} {plus:+#.t$}/-{minus:#.t$}");
                    } else {
                        return write!(f, "{value:#.v$} {plus:+#.t$}/{minus:+#.t$}");
                    }
                }
                let tol_round = crate::Unit::potency(4 - t.min(4));
                let plus = plus.round(tol_round);
                let minus = minus.round(tol_round);
                if plus == -minus {
                    write!(f, "{value:.v$} +/-{plus:.t$}")
                } else {
                    if minus.0 == 0 {
                        write!(f, "{value:.v$} {plus:+.t$}/-{minus:.t$}")
                    } else {
                        write!(f, "{value:.v$} {plus:+.t$}/{minus:+.t$}")
                    }

                }
            }
        }

        impl Debug for $Self {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({} {:+} {:+})", stringify!($Self), self.value, self.plus, self.minus)
            }
        }

        /// A maybe harmful conversation. Ignores all possible tolerance.
        /// Returns a f64 representing a mm value.
        impl From<$Self> for f64 {
            fn from(v: $Self) -> Self {
                v.value.as_f64()
            }
        }

        /// May be harmful
        impl<V> From<V> for $Self
        where
            V: Into<$value>,
        {
            fn from(f: V) -> Self {
                Self { value: f.into(), plus: $tol::ZERO, minus: $tol::ZERO}
            }
        }

        impl<V, T> From<(V, T)> for $Self
        where
            V: Into<$value>,
            T: Into<$tol>,
        {
            fn from(v: (V, T)) -> Self {
                let t = v.1.into();
                Self::new(v.0, t, -t)
            }
        }

        impl<V, P, M> From<(V, P, M)> for $Self
        where
            V: Into<$value>,
            P: Into<$tol>,
            M: Into<$tol>,
        {
            fn from(v: (V, P, M)) -> Self {
                Self::new(v.0, v.1, v.2)
            }
        }

        impl From<$Self> for (f64, f64, f64) {
            fn from(v: $Self) -> Self {
                (v.value.into(), v.plus.into(), v.minus.into())
            }
        }

        impl<V, P, M> TryFrom<(Option<V>, Option<P>, Option<M>)> for $Self
        where
            V: TryInto<$value> + Debug ,
            P: TryInto<$tol> + Debug ,
            M: TryInto<$tol> + Debug ,
            error::ToleranceError: From<<V as TryInto<$value>>::Error>,
            error::ToleranceError: From<<P as TryInto<$tol>>::Error>,
            error::ToleranceError: From<<M as TryInto<$tol>>::Error>,
        {
            type Error = error::ToleranceError;

            fn try_from(triple: (Option<V>, Option<P>, Option<M>)) -> Result<Self, Self::Error> {
                match triple {

                    (Some(v), Some(p), Some(m)) => {
                        let value = v.try_into()?;
                        Ok(Self {
                            value,
                            plus: p.try_into()?,
                            minus: m.try_into()?,
                        })
                    }
                    (Some(v), Some(p), None) => {
                        let p: $tol = p.try_into()?;
                        Ok(Self {
                            value: v.try_into()?,
                            plus: p,
                            minus: -p,
                        })
                    },
                    (Some(v), None, None) => Ok(Self {
                        value: v.try_into()?,
                        plus: $tol::ZERO,
                        minus: $tol::ZERO,
                    }),
                    _ => Err(ParseError(format!("{} not parseble from '{triple:?}'", stringify!($Self)))),
                }
            }
        }

        impl TryFrom<&str> for $Self {
            type Error = error::ToleranceError;

            fn try_from(text : &str) -> Result<Self, Self::Error> {
                let s = text.replace("+/-", " ").replace("+-", " ").replace('/', " ");
                let parts: Vec<Result<i64, Self::Error>> = s.split_whitespace().map(| part | {
                    crate::try_from_str(part, &stringify!($Self))
                }).collect();
                if parts.iter().find(|r| r.is_err()).is_some() {
                    return Err(ParseError(format!("{} not parseble from '{text}'!", stringify!($Self))))
                };
                if parts.is_empty() {
                    return Err(ParseError(format!("Cannot parse an empty string into a {}!", stringify!($Self))))
                }
                let mut parts = parts.into_iter().map(Result::unwrap);
                $Self::try_from((parts.next(), parts.next(), parts.next()))
            }
        }

        impl TryFrom<String> for $Self {
            type Error = error::ToleranceError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_from(value.as_str())
            }
        }

        impl TryFrom<(Option<&i32>, Option<&i32>, Option<&i32>)> for $Self {
            type Error = error::ToleranceError;

            fn try_from(triple: (Option<&i32>, Option<&i32>, Option<&i32>)) -> Result<Self, Self::Error> {
                match triple {
                    (Some(&v), Some(&p), Some(&m)) => Ok($Self::new(v, p, m)),
                    (Some(&v), Some(&p), None) => Ok($Self::new(v, p, -p)),
                    (Some(&v), None, None) => Ok($Self::new(v, 0, 0)),
                    _ => Err(ParseError(format!("{} not parseble from '{triple:?}'!", stringify!($Self)))),
                }
            }
        }

        impl TryFrom<(Option<&i64>, Option<&i64>, Option<&i64>)> for $Self {
            type Error = error::ToleranceError;

            fn try_from(triple: (Option<&i64>, Option<&i64>, Option<&i64>)) -> Result<Self, Self::Error> {
                match triple {
                    (Some(&v), Some(&p), Some(&m)) => Ok(Self {
                        value: $value::try_from(v)?,
                        plus: $tol::try_from(p)?,
                        minus: $tol::try_from(m)?,
                    }),
                    (Some(&v), Some(&p), None) => {
                        let p = $tol::try_from(p)?;
                        Ok(Self {
                            value: $value::try_from(v)?,
                            plus: p,
                            minus: -p,
                        })
                    }
                    (Some(&v), None, None) => Ok(Self {
                        value: $value::try_from(v)?,
                        plus: $tol::ZERO,
                        minus: $tol::ZERO,
                    }),
                    _ => Err(ParseError(format!("{} not parseble from '{triple:?}'!", stringify!($Self)))),
                }
            }
        }

        impl TryFrom<&[i64]> for $Self {
            type Error = error::ToleranceError;

            fn try_from(value: &[i64]) -> Result<Self, Self::Error> {
                let mut iter = value.iter();
                Self::try_from((iter.next(), iter.next(), iter.next()))
            }
        }

    };
}

pub(crate) use tolerance_body;
