pub trait MythBased {
    fn ser_as_string(&self) -> Option<String>;
}

macro_rules! impl_myth_based {
    ($Self:ident) => {
        impl MythBased for $Self {
            fn ser_as_string(&self) -> Option<String> {
                Some(self.to_string())
            }
        }

        impl MythBased for Option<$Self> {
            fn ser_as_string(&self) -> Option<String> {
                self.map(|t| t.to_string())
            }
        }
    };
}

impl_myth_based!(T128);
impl_myth_based!(T64);
impl_myth_based!(Myth64);
impl_myth_based!(Myth32);
impl_myth_based!(Myth16);

/// Serializes into a string like [`format!()`](struct.T128.html).
/// ```json
/// "width": "10.0 +/-0.1"
/// ```
///
/// From [T128], [T64], [Myth64], [Myth32], [Myth16] or wraped in an `Option`.
///
#[inline]
pub fn into_string<S>(t: &dyn MythBased, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match t.ser_as_string() {
        Some(ref value) => serializer.serialize_some(&value.to_string()),
        None => serializer.serialize_none(),
    }
}

pub trait ToleranceF64 {
    fn _type(&self) -> &'static str;
    fn value_f64(&self) -> f64;
    fn plus_f64(&self) -> f64;
    fn minus_f64(&self) -> f64;
}

macro_rules! impl_tolerance_f64 {
    ($Self:ident) => {
        impl ToleranceF64 for $Self {
            fn _type(&self) -> &'static str {
                stringify!($Self)
            }
            fn value_f64(&self) -> f64 {
                self.value.as_f64()
            }
            fn plus_f64(&self) -> f64 {
                self.plus.as_f64()
            }
            fn minus_f64(&self) -> f64 {
                self.minus.as_f64()
            }
        }
    };
}

impl_tolerance_f64!(T128);
impl_tolerance_f64!(T64);

/// Serialize the `T128` or `T64` into a struct like the default serializer but with `f64` fields.
/// ```json
/// "width": {
///   "value": 10.0,
///   "plus": 0.1,
///   "minus": -o.1
/// }
/// ```
pub fn into_float_struct<S>(t: &dyn ToleranceF64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeStruct;
    let mut state = serializer.serialize_struct(&t._type(), 3)?;
    state.serialize_field("value", &t.value_f64())?;
    state.serialize_field("plus", &t.plus_f64())?;
    state.serialize_field("minus", &t.minus_f64())?;
    state.end()
}

/// Serialize the `T128` into a array of 3 `f64` fields (value, plus, minus).
/// ```json
/// "width": [10.0, 0.1, -0.1]
/// ```
pub fn into_to_float_seq<S>(t: &dyn ToleranceF64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(3))?;
    seq.serialize_element(&t.value_f64())?;
    seq.serialize_element(&t.plus_f64())?;
    seq.serialize_element(&t.minus_f64())?;
    seq.end()
}

macro_rules! empty_to_zero {
    ($Self:ident, $fn:ident) => {
        #[doc = concat!("Deserialzes the annoted `Option<", stringify!($Self), ">` from a string-value.")]
        /// In the special-case of an empty string, it would return `Some(Self::ZERO)`.
        ///
        /// ```rust
        ///# use tolerance::*;
        ///# use serde::{Deserialize, Serialize};
        ///#
        /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
        /// struct T3 {
        #[doc = concat!("     #[serde(deserialize_with = \"", stringify!($fn), "\")]")]
        #[doc = concat!("     width: Option<", stringify!($Self), ">,")]
        /// }
        /// assert_eq!(
        ///   serde_json::from_str::<T3>(r#"{"width": ""}"#).unwrap(),
        ///    T3 {
        #[doc = concat!("        width: Some(", stringify!($Self), "::ZERO)")]
        ///    }
        ///);
        ///```
        pub fn $fn<'de, D>(deserializer: D) -> Result<Option<$Self>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct MyVisitor;

            impl<'de> serde::de::Visitor<'de> for MyVisitor {
                type Value = Option<$Self>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str(concat!(
                        "a string parsable to ",
                        stringify!($Self),
                        " or empty!"
                    ))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v.trim().is_empty() {
                        return Ok(Some($Self::ZERO));
                    }
                    $Self::try_from(v).map(Some).map_err(|_| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &"\"1.0\"")
                    })
                }

                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    deserializer.deserialize_any(MyVisitor)
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(None)
                }
            }
            deserializer.deserialize_option(MyVisitor)
        }
    };
}

macro_rules! empty_to_none {
    ($Self:ident, $fn:ident) => {
        #[doc = concat!("Deserialzes the annoted `Option<", stringify!($Self), ">` from a string-value.")]
        /// In the special-case of an empty string, it would return `None`.
        ///
        /// ```rust
        ///# use tolerance::*;
        ///# use serde::{Deserialize, Serialize};
        ///#
        /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
        /// struct T3 {
        #[doc = concat!("     #[serde(deserialize_with = \"", stringify!($fn), "\")]")]
        #[doc = concat!("     width: Option<", stringify!($Self), ">,")]
        /// }
        /// assert_eq!(
        ///   serde_json::from_str::<T3>(r#"{"width": ""}"#).unwrap(),
        ///    T3 {
        ///         width: None
        ///    }
        ///);
        ///```
        pub fn $fn<'de, D>(deserializer: D) -> Result<Option<$Self>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct MyVisitor;

            impl<'de> serde::de::Visitor<'de> for MyVisitor {
                type Value = Option<$Self>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str(concat!(
                        "a string parsable to ",
                        stringify!($Self),
                        " or empty!"
                    ))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v.trim().is_empty() {
                        return Ok(None);
                    }
                    $Self::try_from(v).map(Some).map_err(|_| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &"\"1.0\"")
                    })
                }

                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    deserializer.deserialize_any(MyVisitor)
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(None)
                }
            }
            deserializer.deserialize_option(MyVisitor)
        }
    };
}

empty_to_zero!(T128, empty_to_zero_t128);
empty_to_none!(T128, empty_to_none_t128);
empty_to_zero!(T64, empty_to_zero_t64);
empty_to_none!(T64, empty_to_none_t64);
empty_to_zero!(Myth64, empty_to_zero_myth64);
empty_to_zero!(Myth32, empty_to_zero_myth32);
empty_to_zero!(Myth16, empty_to_zero_myth16);
