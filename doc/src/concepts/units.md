# Units

A unit identifies the scale and notation used for a quantity. In
`runar-units`, units are zero-sized marker types that implement the
[`Unit`](https://docs.rs/runar-units/latest/runar_units/trait.Unit.html) trait.
The trait encodes two important pieces of information:

- `ABBREVIATION`, the text used when formatting the quantity; and
- `SCALE_TO_BASE`, the linear factor that converts the unit to the base unit
  of its dimension.

The base unit has a scale factor of `1.0` by default. Other units express their
relationship to it. For length, for example:

```rust
use runar_units::Unit;
use runar_units::types::l1::{Centimeter, Kilometer, Meter};

assert_eq!(Meter::ABBREVIATION, "m");
assert_eq!(Meter::SCALE_TO_BASE, 1.0);
assert_eq!(Kilometer::ABBREVIATION, "km");
assert_eq!(Kilometer::SCALE_TO_BASE, 1e3);
assert_eq!(Centimeter::ABBREVIATION, "cm");
assert_eq!(Centimeter::SCALE_TO_BASE, 1e-2);
```

The scale is relative to the dimension's base unit, not necessarily to the
unit used to display a particular value. Converting from one unit to another
uses the ratio of their scale factors:

```text
conversion_factor = source_scale_to_base / target_scale_to_base
converted_value = value * conversion_factor
```

Thus `2.0` kilometres becomes `2000.0` metres, while `2.0` metres becomes
`200.0` centimetres. For linear units, the conversion is explicit and reduces
to multiplication by a compile-time-known factor.

## Dimension-specific marker traits

`Unit` describes information common to every unit, but by itself it does not
say which dimension a unit belongs to. Each dimension therefore has its own
marker trait, such as `L1Unit` for length units or `T1Unit` for time units:

```rust
pub trait L1Unit: Unit {}
pub trait T1Unit: Unit {}
```

These traits have no methods or stored data. Their purpose is to create
compile-time categories. A length unit implements `L1Unit`, while a time unit
implements `T1Unit`:

```rust
impl Unit for Meter {
    const ABBREVIATION: &'static str = "m";
}
impl L1Unit for Meter {}

impl Unit for Second {
    const ABBREVIATION: &'static str = "s";
}
impl T1Unit for Second {}
```

The quantity type uses the corresponding marker trait as a bound on its unit
parameter. Conceptually, a length quantity is constrained like this:

```rust
struct Length<V, U>
where
    U: L1Unit,
{
    value: V,
    unit: core::marker::PhantomData<U>,
}
```

As a result, `Length<f64, Meter>` is valid because `Meter: L1Unit`, while
`Length<f64, Second>` is rejected by the compiler because `Second` only
implements `T1Unit`. The same numeric representation can therefore be used
for every dimension without allowing units to cross those dimension
boundaries accidentally.

The marker traits also constrain the arithmetic and conversion implementations.
Operations defined for `L1` can accept only `L1Unit` values, and operations
defined for `T1` can accept only `T1Unit` values. This keeps the dimension
relationship attached to the type throughout construction, arithmetic, and
conversion.

The optional `LATEX_ABBREVIATION` constant can provide a notation suitable for
LaTeX output. If it is not specified, it defaults to `ABBREVIATION`.

## Defining a unit

A custom unit implements `Unit` and the marker trait for its dimension:

```rust
use runar_units::Unit;

struct Furlong;

impl Unit for Furlong {
    const ABBREVIATION: &'static str = "fur";
    const SCALE_TO_BASE: f64 = 201.168;
}
```

It must also implement the dimension-specific marker trait:

```rust
impl L1Unit for Furlong {}
```

The unit can then be accepted by the length quantity type. Follow the existing
unit modules for the complete pattern, including a constructor and explicit
conversion implementations.
