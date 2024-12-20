### Serde

#### Default serializer

Serializes into something like:
```json
"width": {
  "value": 100000,
  "plus": 1000,
  "minus": -1000
}
```

#### Plugable serializer

Serde offers a [`serialize_with`](https://serde.rs/field-attrs.html#serialize_with) parameter.

```rust
# use serde::Serialize;
use tolerance::*;

#[derive(Serialize)]
struct Part {
    #[serde(serialize_with = "into_string")]
    width: Option<T128>
}
```
Available serializers:
* `into_string`: serialize into string-field like the [`to_string()`-method](fn.into_string.html).
* `into_float_struct`: serialize into a [struct like the default-serializer](#method.into_float_struct) but with float-fields.
* `into_float_seq`: serialize into an [array of 3 float-fields](#method.into_float_seq).

#### Deserialization

While deserializing ommiting `minus` is interpreted as `-plus`. Ommitting `plus`- and `minus`-parts defaulting to ZERO.
The properties can be abbreviated to `v`, `p` and `m`. A JSON of `"width":{"v":10000}` would be valid.

Beside the standard deserializer some special-deserializer are offered.
* [interpreting an empty-string as `Self::ZERO`](#method.empty_to_zero)
* [interpreting an empty-string as `None`](#method.empty_to_none)

```rust
# use tolerance::T128;
#
/// JSON-struct
let t: T128 = serde_json::from_slice(
    b"{\"value\": 1245.67, \"plus\": 0.3, \"minus\": -0.5 }"
).unwrap();
assert_eq!(t, T128::new(1245_6700, 0.3, -0.5));

let t: T128 = serde_json::from_slice(b"{\"v\": 1245.67, \"p\": 0.3 }").unwrap();
assert_eq!(t, T128::new(1245_6700, 0.3, -0.3));

let t: T128 = serde_json::from_slice(b"{\"v\": 1245.67}").unwrap();
assert_eq!(t, T128::new(1245_6700, 0, -0));

/// JSON-array
let t: T128 = serde_json::from_slice(b"[ 1245.67, 0.3,  -0.5 ]").unwrap();
assert_eq!(t, T128::new(1245_6700, 0.3, -0.5));

let t: T128 = serde_json::from_slice(b"[ 1245.67, 0.3 ]").unwrap();
assert_eq!(t, T128::new(1245_6700, 0.3, -0.3));

let t: T128 = serde_json::from_slice(b"[ 1245.67 ]").unwrap();
assert_eq!(t, T128::new(1245_6700, 0, -0));

/// JSON single value `Float`
let t: T128 = serde_json::from_slice(b"1245.67").unwrap();
assert_eq!(t, T128::new(1245_6700, 0, -0));

/// JSON single value `Integer`
let t: T128 = serde_json::from_slice(b"12456700").unwrap();
assert_eq!(t, T128::new(1245_6700, 0, -0));

/// JSON single value `String`
let t: T128 = serde_json::from_slice(b"\"1245.6700 +0.45 -0.2\"").unwrap();
assert_eq!(t, T128::new(1245_6700, 0.45, -0.2));

let t: T128 = serde_json::from_slice(b"\"1245.67 +/- 0.45\"").unwrap();
assert_eq!(t, T128::new(1245_6700, 0.45, -0.45));

let t: T128 = serde_json::from_slice(b"\"1245.67;0.45\"").unwrap();
assert_eq!(t, T128::new(1245_6700, 0.45, -0.45));

let t: T128 = serde_json::from_slice(b"\"1245.67\"").unwrap();
assert_eq!(t, T128::new(1245_6700, 0, -0));

```
