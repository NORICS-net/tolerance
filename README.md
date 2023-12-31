# Tolerance 

[![crates.io](https://img.shields.io/crates/v/tolerance.svg)](https://crates.io/crates/tolerance)
[![crates.io](https://img.shields.io/crates/d/tolerance.svg)](https://crates.io/crates/tolerance)
[![Documentation](https://docs.rs/tolerance/badge.svg)](https://docs.rs/tolerance)

Math representation of the physically needed permissible deviation of measures in Rust
avoiding floating point inaccuracy. Allows to calculate with tolerance ranges in a consistent way.

Based of an own type `Myth` with a accuracy of 1/10th my-meter (= 0.1μ).

### Example

```rust
use tolerance::T128;

fn main() {
    let width1 = T128::new(100.0, 0.05, -0.2);
    let width2 = T128::with_sym(50.0, 0.05);

    // Adding two `T128`s is straightforward.
    assert_eq!(width1 + width2, T128::new(150.0, 0.1, -0.25));

    // `!` inverts the direction of tolerance to /subtract/ measures.
    assert_eq!(!width1, T128::new(-100.0, 0.2, -0.05));

    // Adding an inverted `T128` wides the tolerance.
    assert_eq!(width1 + !width1, T128::new(0.0, 0.25, -0.25));
}
```

### Limits 

#### T128

A 128bit wide value. Based on a `Myth64` (64bit) for the value it could handle sizes up to +/-922_337_203 km 
with a deviation of +/-214 m (`Myth32`). 

#### T64

based on a `Myth32` (32bit) for the value it could handle sizes up to +/-214 m
with a deviation of +/-3 mm (`Myth16`). 


### History

Started as [AllowanceValue](https://github.com/migmedia/allowance) renamed and moved for better usability.  

## License

Licensed under  MIT license (([LICENSE-MIT](LICENSE) or https://opensource.org/licenses/MIT)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MIT license, shall be licensed as above,
without any additional terms or conditions.
