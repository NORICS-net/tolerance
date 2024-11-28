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
        const PPOS : usize = std::mem::size_of::<$value>();
        const MPOS : usize = std::mem::size_of::<$value>() + std::mem::size_of::<$tol>();

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

            #[doc = concat!("Returns the maximum allowed value of this ", stringify!($Self), ".")]
            pub fn upper_limit(&self) -> $value {
                self.value + self.plus
            }

            /// Returns the minimum value of this tolerance.
            #[doc = concat!("Returns the minimum allowed value of this ", stringify!($Self), ".")]
            pub fn lower_limit(&self) -> $value {
                self.value + self.minus
            }

            /// Returns `true`, if `self` is more narrow than the `other`.
            #[must_use]
            pub fn is_inside_of(&self, other: Self) -> bool {
                self.lower_limit() >= other.lower_limit()
                    && self.upper_limit() <= other.upper_limit()
            }

            /// Returns `true`, if `self` is less strict (around) the `other`.
            #[must_use]
            pub fn enfold(&self, other: impl Into<$Self>) -> bool {
                let other = other.into();
                self.lower_limit() <= other.lower_limit()
                    && self.upper_limit() >= other.upper_limit()
            }

            #[doc = concat!("Inverts this `", stringify!($Self), "`.")]
            /// Interchanges the `plus` and `minus` parts.
            /// Required when measuring back in the opposite direction.
            ///
            #[doc = concat!("Same as [`!value`](#impl-Not-for-", stringify!($Self), ").")]
            pub fn invert(&self) -> Self {
                Self {
                    value: -self.value,
                    plus: -self.minus,
                    minus: -self.plus,
                }
            }

            #[doc = concat!("Returns the memory representation of this ", stringify!($Self), " as a byte array in")]
            /// big-endian (network) byte order.
            #[must_use]
            pub fn to_be_bytes(&self) -> [u8; std::mem::size_of::<$Self>()] {
                let mut buffer = [0u8; std::mem::size_of::<$Self>()];
                buffer[..PPOS].clone_from_slice(&$value::to_be_bytes(&self.value));
                buffer[PPOS..MPOS].clone_from_slice(&$tol::to_be_bytes(&self.plus));
                buffer[MPOS..].clone_from_slice(&$tol::to_be_bytes(&self.minus));
                buffer
            }

            #[doc = concat!("Creates a ", stringify!($Self), " value from its representation")]
            /// as a byte array in big-endian.
            pub fn from_be_bytes(bytes: [u8; std::mem::size_of::<$Self>()]) -> Self {
                Self {
                    value: $value::from_be_bytes(bytes[..PPOS].try_into().expect("Slice has the wrong length")),
                    plus: $tol::from_be_bytes(bytes[PPOS..MPOS].try_into().expect("Slice has the wrong length")),
                    minus: $tol::from_be_bytes(bytes[MPOS..].try_into().expect("Slice has the wrong length")),
                }
            }

            #[doc = concat!("Returns the memory representation of this ", stringify!($Self), " as a byte array in")]
            /// little-endian byte order.
            #[must_use]
            pub fn to_le_bytes(&self) -> [u8; std::mem::size_of::<$Self>()] {
                let mut buffer = [0u8; std::mem::size_of::<$Self>()];
                buffer[..PPOS].clone_from_slice(&$value::to_le_bytes(&self.value));
                buffer[PPOS..MPOS].clone_from_slice(&$tol::to_le_bytes(&self.plus));
                buffer[MPOS..].clone_from_slice(&$tol::to_le_bytes(&self.minus));
                buffer
            }

            #[doc = concat!("Creates a ", stringify!($Self), " value from its representation")]
            /// as a byte array in little-endian.
            pub fn from_le_bytes(bytes: [u8; std::mem::size_of::<$Self>()]) -> Self {
                Self {
                    value: $value::from_le_bytes(bytes[..PPOS].try_into().expect("Slice has the wrong length")),
                    plus: $tol::from_le_bytes(bytes[PPOS..MPOS].try_into().expect("Slice has the wrong length")),
                    minus: $tol::from_le_bytes(bytes[MPOS..].try_into().expect("Slice has the wrong length")),
                }
            }

            #[doc = concat!("Returns the memory representation of this ", stringify!($Self), " as a byte array in")]
            /// native byte order.
            #[must_use]
            pub fn to_ne_bytes(&self) -> [u8; std::mem::size_of::<$Self>()] {
                let mut buffer = [0u8; std::mem::size_of::<$Self>()];
                buffer[..PPOS].clone_from_slice(&$value::to_ne_bytes(&self.value));
                buffer[PPOS..MPOS].clone_from_slice(&$tol::to_ne_bytes(&self.plus));
                buffer[MPOS..].clone_from_slice(&$tol::to_ne_bytes(&self.minus));
                buffer
            }

            #[doc = concat!("Creates a ", stringify!($Self), " value from its representation")]
            /// as a byte array in native byte order.
            pub fn from_ne_bytes(bytes: [u8; std::mem::size_of::<$Self>()]) -> Self {
                Self {
                    value: $value::from_ne_bytes(bytes[..PPOS].try_into().expect("Slice has the wrong length")),
                    plus: $tol::from_ne_bytes(bytes[PPOS..MPOS].try_into().expect("Slice has the wrong length")),
                    minus: $tol::from_ne_bytes(bytes[MPOS..].try_into().expect("Slice has the wrong length")),
                }
            }
        }

        #[doc = concat!("Inverts this `", stringify!($Self), "`.")]
        /// Interchanges the `plus` and `minus` parts.
        /// Required when measuring back in the opposite direction.
        ///
        #[doc = concat!("Shortcut for the [`", stringify!($Self), ".invert()`](#method.invert)-method.")]
        impl Not for $Self {
            type Output = $Self;

            fn not(self) -> Self::Output {
                self.invert()
            }
        }

        #[doc = concat!("Inverts this `", stringify!($Self), "`.")]
        /// Interchanges the `plus` and `minus` parts.
        /// Required when measuring back in the opposite direction.
        ///
        #[doc = concat!("Shortcut for the [`", stringify!($Self), ".invert()`](#method.invert)-method.")]
        impl Not for &$Self {
            type Output = $Self;

            fn not(self) -> Self::Output {
                self.invert()
            }
        }

        #[doc = concat!("Inverts the signs of all fields in this `", stringify!($Self), "`.")]
        /// Like multiplying by `-1`.
        impl Neg for $Self {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new(-self.value, -self.plus, -self.minus)
            }
        }

        #[doc = concat!("Inverts the signs of all fields in this `", stringify!($Self), "`.")]
        /// Like multiplying by `-1`.
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

        impl Add<$tol> for $Self {
            type Output = $Self;

            fn add(self, other: $tol) -> $Self {
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

        impl AddAssign<$value> for $Self   {
            fn add_assign(&mut self, other: $value) {
                self.value += other;
            }
        }

        impl AddAssign<$tol> for $Self   {
            fn add_assign(&mut self, other: $tol) {
                self.value += $value::from(other);
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

        impl Sub<$tol> for $Self {
            type Output = $Self;

            fn sub(self, other: $tol) -> $Self {
                $Self {
                    value: self.value - other,
                    plus: self.plus,
                    minus: self.minus,
                }
            }
        }

        impl SubAssign for $Self {
            fn sub_assign(&mut self, other: Self) {
                self.value -= other.value;
                self.plus -= other.plus;
                self.minus -= other.minus;
            }
        }

        impl SubAssign<$value> for $Self   {
            fn sub_assign(&mut self, other: $value) {
                self.value -= other;
            }
        }

        impl SubAssign<$tol> for $Self   {
            fn sub_assign(&mut self, other: $tol) {
                self.value -= $value::from(other);
            }
        }

        impl PartialOrd for $Self {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        /// Defines the order by comparing:
        /// 1. value
        /// 2. minus
        /// 3. plus
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
                let tol_round = crate::Unit::potency(4 - t.min(4));
                let plus = self.plus.round(tol_round);
                let minus = self.minus.round(tol_round);
                let value = self.value;
                if plus == -minus && !f.alternate() && !plus.is_negative() {
                    if f.precision().is_some() {
                        write!(f, "{value:.v$} +/-{plus:.t$}")
                    } else {
                        write!(f, "{value} +/-{plus}")
                    }
                } else {
                    let m = if minus.0 > 0 { "+" } else if minus.0 == 0 { "-" } else { "" };
                    if f.alternate() {
                        write!(f, "{value:#.v$} {plus:+#.t$}/{m}{minus:#.t$}")
                    } else {
                        if f.precision().is_some() {
                        write!(f, "{value:.v$} {plus:+.t$}/{m}{minus:.t$}")
                        } else {
                            write!(f, "{value} {plus:+}/{m}{minus}")
                        }
                    }
                }
            }
        }

        impl Debug for $Self {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let $Self{value, plus, minus} = self;
                if let Some(t) = f.precision() {
                    write!(f, "{}({value:.t$} {plus:+.t$} {minus:+.t$})", stringify!($Self))
                } else {
                    write!(f, "{}({value} {plus:+} {minus:+})", stringify!($Self))
                }
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
                    _ => Err(ParseError(format!("{} not parsable from '{triple:?}'", stringify!($Self)))),
                }
            }
        }

        #[doc = concat!("This function is an alias of the [FromStr](#impl-FromStr-for-",
            stringify!($Self), ")-trait implementation.")]
        impl TryFrom<&str> for $Self {
            type Error = error::ToleranceError;

            fn try_from(text : &str) -> Result<Self, Self::Error> {
                $Self::from_str(text)
            }
        }

        #[doc = concat!("This function is an alias of the [FromStr](#impl-FromStr-for-",
            stringify!($Self), ")-trait implementation.")]
        impl TryFrom<String> for $Self {
            type Error = error::ToleranceError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                $Self::from_str(value.as_str())
            }
        }

        #[doc = concat!("Converts a string into a ", stringify!($Self), ".")]
        ///
        /// ### Input-interpretation:
        ///
        /// * Values are interpreted as *mm* — the point and decimal places can be omitted. (`140` => `140.0000`)
        /// * A leading zero can be omitted. (`.04` => `0.0400`)
        /// * Possible divider between the 3 parts are `' '` (blank #32), `/` or `;`.
        /// * 3 parts  =>  value, plus, minus
        /// * 2 parts  =>  value, plus, -plus
        /// * 1 part   =>  value, 0.0, 0.0
        ///
        impl FromStr for $Self {
            type Err = error::ToleranceError;

                // Required method
                fn from_str(text: &str) -> Result<Self, Self::Err> {
                    let s = text.replace("+/-", " ").replace("+-", " ").replace('/', " ").replace(';', " ");
                    let parts: Vec<Result<i64, Self::Err>> = s.split_whitespace().map(| part | {
                        crate::try_from_str(part, &stringify!($Self))
                    }).collect();
                    if parts.iter().find(|r| r.is_err()).is_some() {
                        return Err(ParseError(format!("{} not parsable from '{text}'!", stringify!($Self))))
                    };
                    if parts.is_empty() {
                        return Err(ParseError(format!("Can not parse an empty string into a {}!", stringify!($Self))))
                    }
                    let mut parts = parts.into_iter().map(Result::unwrap);
                    $Self::try_from((parts.next(), parts.next(), parts.next()))
                }
        }

        impl TryFrom<(Option<&i32>, Option<&i32>, Option<&i32>)> for $Self {
            type Error = error::ToleranceError;

            fn try_from(triple: (Option<&i32>, Option<&i32>, Option<&i32>)) -> Result<Self, Self::Error> {
                match triple {
                    (Some(&v), Some(&p), Some(&m)) => Ok($Self::new(v, p, m)),
                    (Some(&v), Some(&p), None) => Ok($Self::new(v, p, -p)),
                    (Some(&v), None, None) => Ok($Self::new(v, 0, 0)),
                    _ => Err(ParseError(format!("{} not parsable from '{triple:?}'!", stringify!($Self)))),
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
                    _ => Err(ParseError(format!("{} not parsable from '{triple:?}'!", stringify!($Self)))),
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

#[cfg(feature = "serde")]
macro_rules! de_serde_tol {
    ($Self:ident, $Val:ident, $Tol:ident) => {
        use serde::{
            de::{MapAccess, Visitor},
            ser::SerializeStruct,
            Deserialize, Deserializer, Serialize, Serializer,
        };

        impl Serialize for $Self {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let mut state = serializer.serialize_struct(stringify!($Self), 3)?;
                    state.serialize_field("value", &self.value)?;
                    state.serialize_field("plus", &self.plus)?;
                    state.serialize_field("minus", &self.minus)?;
                    state.end()
                }
        }

        impl<'de> Deserialize<'de> for $Self {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                enum Field {
                    Value,
                    Plus,
                    Minus,
                }
                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        struct FieldVisitor;

                        impl<'de> Visitor<'de> for FieldVisitor {
                            type Value = Field;

                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter,
                            ) -> std::fmt::Result {
                                formatter.write_str("`value`, `plus` or `minus`")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Field, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "value" | "v" => Ok(Field::Value),
                                    "plus" | "p" => Ok(Field::Plus),
                                    "minus" | "m" => Ok(Field::Minus),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }
                struct TolVisitor;

                impl<'de> Visitor<'de> for TolVisitor {
                    type Value = $Self;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!(
                            "a ",
                            stringify!($Self),
                            " either as a struct `{v=1.0,p=0.2,m=-0.2}` or as string `\"1.0 +/-0.2\"`"
                        ))
                    }

                    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        $Self::try_from(v).map_err(|_| {
                            serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(v),
                                &"1.0 +/- 0.2",
                            )
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
                        let m = $Val::try_from(v).map_err(|_| {
                            serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &"10000")
                        })?;
                        Ok($Self::from(m))
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let m = $Val::try_from(v).map_err(|_| {
                            serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &"10000")
                        })?;
                        Ok($Self::from(m))
                    }

                    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($Self::from(v))
                    }

                    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let m = $Val::try_from(v).map_err(|_| {
                            serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v as u64), &"10000")
                        })?;
                        Ok($Self::from(m))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                        where
                            V: serde::de::SeqAccess<'de>,
                        {
                            let value : $Val = seq.next_element()?
                                .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                            let plus : $Tol = seq.next_element()?.unwrap_or($Tol::ZERO);
                            let minus : $Tol = seq.next_element()?.unwrap_or(plus.neg());
                            Ok($Self {
                                value, plus, minus
                            })
                        }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        let mut value = None;
                        let mut plus = None;
                        let mut minus = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                Field::Value => {
                                    if value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    value = Some(map.next_value()?);
                                }
                                Field::Plus => {
                                    if plus.is_some() {
                                        return Err(serde::de::Error::duplicate_field("plus"));
                                    }
                                    plus = Some(map.next_value()?);
                                }
                                Field::Minus => {
                                    if minus.is_some() {
                                        return Err(serde::de::Error::duplicate_field("minus"));
                                    }
                                    minus = Some(map.next_value()?);
                                }
                            }
                        }

                        let value : $Val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                        let plus : $Tol = plus.unwrap_or($Tol::ZERO);
                        let minus : $Tol = minus.unwrap_or(plus.neg());
                        Ok($Self {
                            value, plus, minus
                        })
                    }

                    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        deserializer.deserialize_any(TolVisitor)
                    }

                    fn visit_newtype_struct<D>(
                        self,
                        deserializer: D,
                    ) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        deserializer.deserialize_any(TolVisitor)
                    }
                }

                const FIELDS: &[&str] = &["value", "plus", "minus"];
                deserializer.deserialize_any(TolVisitor)
            }
        }
    };
}

#[cfg(feature = "serde")]
pub(crate) use de_serde_tol;
