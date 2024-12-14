pub trait MythBased {
    fn is_option(&self) -> bool;
    fn ser_as_string(&self) -> Option<String>;
}

macro_rules! impl_myth_based {
    ($Self:ident) => {
        impl MythBased for $Self {
            fn is_option(&self) -> bool {
                false
            }
            fn ser_as_string(&self) -> Option<String> {
                Some(self.to_string())
            }
        }

        impl MythBased for Option<$Self> {
            fn is_option(&self) -> bool {
                true
            }
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

/// Serializes into a string like [`format!("{}")`](struct.T128.html).
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
    if t.is_option() {
        match t.ser_as_string() {
            Some(ref value) => serializer.serialize_some(&value.to_string()),
            None => serializer.serialize_none(),
        }
    } else {
        serializer.serialize_some(&t.ser_as_string().unwrap().to_string())
    }
}

macro_rules! impl_t_into_f64s {
    ($Self:ident, $fn_struct:expr, $fn_seq:expr) => {
        impl $Self {
            #[doc = concat!("Serializes a `", stringify!($Self), "` into a struct like the default serializer but with `f64` fields.")]
            /// ```json
            /// "width": {
            ///   "value": 10.0,
            ///   "plus": 0.1,
            ///   "minus": -0.1
            /// }
            /// ```
            /// ### Example
            /// ```rust
            ///# use serde::*;
            ///# use serde_json::to_string;
            ///# use tolerance::*;
            ///#
            /// #[derive(Serialize)]
            /// struct T2 {
            #[doc = concat!("     #[serde(serialize_with = \"", stringify!($Self), "::into_float_struct\")]")]
            #[doc = concat!("     width: ", stringify!($Self), ",")]
            /// }
            /// let t = T2 {
            #[doc = concat!("     width: ", stringify!($Self), "::from(123455),")]
            /// };
            /// assert_eq!(
            ///     r#"{"width":{"value":12.3455,"plus":0.0,"minus":0.0}}"#,
            ///     serde_json::to_string(&t).unwrap()
            /// );
            /// ```
            pub fn into_float_struct<S>(t: &$Self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(stringify!($Self), 3)?;
                state.serialize_field("value", &t.value.as_f64())?;
                state.serialize_field("plus", &t.plus.as_f64())?;
                state.serialize_field("minus", &t.minus.as_f64())?;
                state.end()
            }

            #[doc = concat!("Serializes an `Option<", stringify!($Self), ">` into a struct like the default serializer but with `f64` fields.")]
            /// ### Example
            /// ```rust
            ///# use serde::*;
            ///# use serde_json::to_string;
            ///# use tolerance::*;
            ///#
            /// #[derive(Serialize)]
            /// struct T2 {
            #[doc = concat!("     #[serde(serialize_with = \"", stringify!($Self), "::option_into_float_struct\")]")]
            #[doc = concat!("     width: Option<", stringify!($Self), ">,")]
            /// }
            /// let t = T2 {
            #[doc = concat!("     width: Some(", stringify!($Self), "::from(123455)),")]
            /// };
            /// assert_eq!(
            ///     r#"{"width":{"value":12.3455,"plus":0.0,"minus":0.0}}"#,
            ///     serde_json::to_string(&t).unwrap()
            /// );
            /// let t = T2 { width: None };
            /// assert_eq!(r#"{"width":null}"#, serde_json::to_string(&t).unwrap());
            /// ```
            pub fn option_into_float_struct<S>(
                t: &Option<$Self>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #[derive(serde::Serialize)]
                #[serde(transparent)]
                struct W<'a>(#[serde(serialize_with = $fn_struct)] &'a $Self);
                match t {
                    Some(ref v) => serializer.serialize_some(&W(v)),
                    None => serializer.serialize_none(),
                }
            }

            #[doc = concat!("Serializes a `", stringify!($Self), "` into an a array of 3 `f64` fields (value, plus, minus).")]
            /// ```json
            /// "width": [10.0, 0.1, -0.1]
            /// ```
            /// ### Example
            /// ```rust
            ///# use serde::*;
            ///# use serde_json::to_string;
            ///# use tolerance::*;
            ///#
            /// #[derive(Serialize)]
            /// struct T2 {
            #[doc = concat!("     #[serde(serialize_with = \"", stringify!($Self), "::into_float_seq\")]")]
            #[doc = concat!("     width: ", stringify!($Self), ",")]
            /// }
            /// let t = T2 {
            #[doc = concat!("     width: ", stringify!($Self), "::from(123455),")]
            /// };
            /// assert_eq!(
            ///     r#"{"width":[12.3455,0.0,0.0]}"#,
            ///     serde_json::to_string(&t).unwrap()
            /// );
            /// ```
            pub fn into_float_seq<S>(t: &$Self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&t.value.as_f64())?;
                seq.serialize_element(&t.plus.as_f64())?;
                seq.serialize_element(&t.minus.as_f64())?;
                seq.end()
            }

            #[doc = concat!("Serializes an `Option<", stringify!($Self), ">` into an a array of 3 `f64` fields (value, plus, minus).")]
            /// ### Example
            /// ```rust
            ///# use serde::*;
            ///# use serde_json::to_string;
            ///# use tolerance::*;
            ///#
            /// #[derive(Serialize)]
            /// struct T2 {
            #[doc = concat!("     #[serde(serialize_with = \"", stringify!($Self), "::option_into_float_seq\")]")]
            #[doc = concat!("     width: Option<", stringify!($Self), ">,")]
            /// }
            /// let t = T2 {
            #[doc = concat!("     width: Some(", stringify!($Self), "::from(123455)),")]
            /// };
            /// assert_eq!(
            ///     r#"{"width":[12.3455,0.0,0.0]}"#,
            ///     serde_json::to_string(&t).unwrap()
            /// );
            /// let t = T2 { width: None };
            /// assert_eq!(r#"{"width":null}"#, serde_json::to_string(&t).unwrap());
            /// ```
            pub fn option_into_float_seq<S>(
                t: &Option<$Self>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #[derive(serde::Serialize)]
                #[serde(transparent)]
                struct W<'a>(#[serde(serialize_with = $fn_seq)] &'a $Self);
                match t {
                    Some(ref v) => serializer.serialize_some(&W(v)),
                    None => serializer.serialize_none(),
                }
            }
        }
    };
}

impl_t_into_f64s!(T128, "T128::into_float_struct", "T128::into_float_seq");
impl_t_into_f64s!(T64, "T64::into_float_struct", "T64::into_float_seq");

macro_rules! empty_to_case {
    ($Self:ident) => {
        impl $Self {
            #[doc = concat!("Deserialzes the annoted `Option<", stringify!($Self), ">` from a string-value.")]
            /// In the special-case of an empty string, it would return `Some(Self::ZERO)`.
            ///
            /// ```rust
            ///# use tolerance::*;
            ///# use serde::{Deserialize, Serialize};
            ///#
            /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
            /// struct T3 {
            #[doc = concat!("     #[serde(deserialize_with = \"", stringify!($Self), "::empty_to_zero\")]")]
            #[doc = concat!("     width: Option<", stringify!($Self), ">,")]
            /// }
            /// assert_eq!(
            ///   serde_json::from_str::<T3>(r#"{"width": ""}"#).unwrap(),
            ///    T3 {
            #[doc = concat!("        width: Some(", stringify!($Self), "::ZERO)")]
            ///    }
            ///);
            ///```
            pub fn empty_to_zero<'de, D>(deserializer: D) -> Result<Option<$Self>, D::Error>
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

            #[doc = concat!("Deserialzes the annoted `Option<", stringify!($Self), ">` from a string-value.")]
            /// In the special-case of an empty string, it would return `None`.
            ///
            /// ```rust
            ///# use tolerance::*;
            ///# use serde::{Deserialize, Serialize};
            ///#
            /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
            /// struct T3 {
            #[doc = concat!("     #[serde(deserialize_with = \"", stringify!($Self), "::empty_to_none\")]")]
            #[doc = concat!("     width: Option<", stringify!($Self), ">,")]
            /// }
            /// assert_eq!(
            ///   serde_json::from_str::<T3>(r#"{"width": ""}"#).unwrap(),
            ///    T3 {
            ///         width: None
            ///    }
            ///);
            ///```
            pub fn empty_to_none<'de, D>(deserializer: D) -> Result<Option<$Self>, D::Error>
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
        }
    };
}

empty_to_case!(Myth16);
empty_to_case!(Myth32);
empty_to_case!(Myth64);
empty_to_case!(T128);
empty_to_case!(T64);
