#![doc = include_str!("../README.md")]

pub mod error;
mod myths;
mod tols;
mod unit;

pub use self::unit::*;
pub use myths::myth16::*;
pub use myths::myth32::*;
pub use myths::myth64::*;
pub use tols::tol128::*;
pub use tols::tol64::*;

use error::ToleranceError;

#[inline]
fn str2int(bytes: &[u8], t_type: &str) -> Result<i64, ToleranceError> {
    let mut v = 0i64;
    for c in bytes {
        match c {
            0x30..=0x39 => v = v * 10 + i64::from(c - 0x30),
            _ => {
                return Err(ToleranceError::ParseError(format!(
                    "Found ascii #{c} (a non-numerical literal) in input, can't parse input into a {t_type}!",
                )))
            }
        }
    }
    Ok(v)
}

/// helper-method used from all types.
#[inline]
pub(crate) fn try_from_str(value: &str, t_type: &str) -> Result<i64, ToleranceError> {
    let value = value.trim();
    if value.is_empty() {
        return ToleranceError::parse_err(format!("Cannot parse an empty string into a {t_type}!"));
    }
    let (base, fraction) = value.split_once('.').unwrap_or((value, "0"));
    let mut base = base.as_bytes();
    let &c = base.first().unwrap_or(&b'0');
    let sign = 1 - i64::from(c == b'-') * 2;
    if c == b'-' || c == b'+' {
        base = &base[1..];
    }
    let fraction = fraction.to_string() + "00000";
    let fraction = fraction.split_at(4).0.as_bytes();
    Ok((str2int(base, t_type)? * Unit::MM + str2int(fraction, t_type)?) * sign)
}
