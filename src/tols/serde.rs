/// Serializes into a string like [`format!()`](struct.T128.html).
/// ```json
/// "width": "10.0 +/-0.1"
/// ```
#[inline]
pub fn into_string<S>(t: &dyn std::any::Any, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if t.is::<Option<T128>>() {
        let opt = (t as &dyn std::any::Any)
            .downcast_ref::<Option<T128>>()
            .ok_or(serde::ser::Error::custom("Error parsing Option<T128>"))?;
        match opt {
            Some(ref value) => serializer.serialize_some(&value.to_string()),
            None => serializer.serialize_none(),
        }
    } else if t.is::<Option<T64>>() {
        let opt = (t as &dyn std::any::Any)
            .downcast_ref::<Option<T64>>()
            .ok_or(serde::ser::Error::custom("Error parsing Option<T64>"))?;
        match opt {
            Some(ref value) => serializer.serialize_some(&value.to_string()),
            None => serializer.serialize_none(),
        }
    } else if t.is::<T128>() {
        let t128 = (t as &dyn std::any::Any)
            .downcast_ref::<T128>()
            .ok_or(serde::ser::Error::custom("Error parsing T128"))?;
        serializer.serialize_str(&t128.to_string())
    } else if t.is::<T64>() {
        let t64 = (t as &dyn std::any::Any)
            .downcast_ref::<T64>()
            .ok_or(serde::ser::Error::custom("Error parsing T64"))?;
        serializer.serialize_str(&t64.to_string())
    } else {
        unreachable!()
    }
}

/// Serialize the `T128` into a struct like the default serializer but with `f64` fields.
/// ```json
/// "width": {
///   "value": 10.0,
///   "plus": 0.1,
///   "minus": -o.1
/// }
/// ```
pub fn into_float_struct<S>(
    T128 { value, plus, minus }: &T128,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeStruct;
    let mut state = serializer.serialize_struct("T128", 3)?;
    state.serialize_field("value", &value.as_f64())?;
    state.serialize_field("plus", &plus.as_f64())?;
    state.serialize_field("minus", &minus.as_f64())?;
    state.end()
}

/// Serialize the `T128` into a array of 3 `f64` fields (value, plus, minus).
/// ```json
/// "width": [10.0, 0.1, -0.1]
/// ```
pub fn into_float_seq<S>(
    T128 { value, plus, minus }: &T128,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(3))?;
    seq.serialize_element(&value.as_f64())?;
    seq.serialize_element(&plus.as_f64())?;
    seq.serialize_element(&minus.as_f64())?;
    seq.end()
}
