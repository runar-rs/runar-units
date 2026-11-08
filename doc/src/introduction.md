# Introduction

`runar-units` provides type-safe measurement units for Rust. It puts the
meaning of a number into its type, so the compiler can reject invalid
combinations of quantities before the program runs.

The crate is intended for applications where values such as lengths, times,
and masses should not be interchangeable merely because they share the same
underlying representation. It focuses on a small, explicit type-level model
that works without a runtime unit tag and is usable in `no_std` programs.

## The problem: numbers lose meaning

Consider an API that uses plain floating-point values:

```rust
fn travel(distance: f64, duration: f64) -> f64 {
    distance / duration
}
```

The function gives no indication that `distance` should be a length and
`duration` should be a time. Both arguments have the same type, so callers can
swap them accidentally. The function can also be called with values expressed
in incompatible units, such as centimetres and hours, without the compiler
being able to help.

The problem is not floating-point arithmetic itself. The problem is that the
type `f64` carries no information about what the value means. Comments and
variable names can document intent, but they do not enforce it.

`runar-units` makes the distinction part of the type:

```rust
use runar_units::types::l1_m1_tn1::{Impulse, NewtonSecond, PoundForceSecond};

fn get_thruster_impulse() -> Impulse<f64, PoundForceSecond> { /* ... */}

fn update_trajectory_model(impulse: Impulse<f64, NewtonSecond>) { /* ... */}

fn main() {
    let thruster_impulse = get_thruster_impulse();
    update_trajectory_model(thruster_impulse.into());
}
```
`.into()` takes care of the unit conversion. If the conversion step is ommited, the compiler will
raise an error.
```rust
update_trajectory_model(truster_impulse);
```


## What this crate provides

The central goals of `runar-units` are:

- represent dimensions and units in Rust's type system;
- reject invalid dimensional arithmetic at compile time;
- make unit-aware function signatures self-documenting;
- derive new quantity types through arithmetic;
- store only the underlying numeric value at runtime; and
- support the quantity implementation in `no_std` environments.

The type system checks dimensional correctness, not every possible numerical
or physical error. Floating-point rounding, overflow, underflow, division by
zero, and physically unreasonable values still require application-specific
handling.

## A first example

Quantities are constructed through their unit marker types:

```rust
use runar_units::Dimension;
use runar_units::types::l1::{Length, Meter};
use runar_units::types::t1::{Second, Time};

let distance = Meter::new(100.0_f64);
let duration = Second::new(20.0_f64);
let speed = distance / duration;

assert_eq!(speed.get(), &5.0);
```

`distance` has type `Length<f64, Meter>`, while `duration` has type
`Time<f64, Second>`. Division produces the crate's metres-per-second
quantity. The result can be passed to another function whose signature
requires that dimension.

Invalid combinations fail at compile time:

```rust,compile_fail
use runar_units::types::l1::Meter;
use runar_units::types::t1::Second;

let distance = Meter::new(10.0_f64);
let duration = Second::new(2.0_f64);
let invalid = distance + duration;
```

This is the main benefit of the crate: a unit mistake becomes a compiler error
at the point where it is introduced, instead of a surprising result at
runtime.

## The type-level model

`runar-units` separates the concepts of a quantity, a dimension, and a unit.

### Quantities

A quantity combines a numeric value with a unit marker. For example,
`Length<f64, Meter>` means that the stored value is an `f64` representing a
length in metres.

### Dimensions

A dimension describes the physical exponents involved in a quantity. The
crate's base dimensions include length, mass, time, electric current,
thermodynamic temperature, amount of substance, and luminous intensity. It
also defines steradian as an additional dimension for more precise type
checking.

Arithmetic combines dimensions. Multiplying two lengths produces an area;
dividing a length by a time produces a velocity. The resulting Rust type
records that relationship.

### Units

A unit identifies a scale within a dimension. `Meter` and `Millimeter` are
both length units, while `Second` and `Hour` are time units. Unit marker types
provide abbreviations and, where applicable, a scale relative to the base
unit.

Conversions should be explicit. This keeps changes of unit visible in the
source code instead of introducing hidden coercions.

## Why the abstraction is zero-cost

Unit markers carry compile-time information but do not store a unit value for
each quantity. Internally, a quantity uses `PhantomData` to associate its
numeric value with the marker type:

```rust
pub struct L1<V, U> {
    value: V,
    unit: core::marker::PhantomData<U>,
}
```

`PhantomData<U>` has no runtime storage. It exists so the compiler can select
the appropriate trait implementations and enforce the relationship between
the value and its unit. Ordinary arithmetic operates on the underlying
`f32` or `f64`; it does not need a runtime unit tag, unit lookup, or dynamic
dispatch.

“Zero-cost” does not mean that every operation is free. The numeric type still
determines the cost and precision of the calculation, and explicit conversions
still perform their normal arithmetic.

## `runar-units` and `uom`

[`uom`](https://crates.io/crates/uom) is another Rust crate for type-safe
quantities. Both projects use Rust's type system to prevent incompatible
dimensional arithmetic, and both can derive new quantity types from
arithmetic operations. The difference is primarily one of scope and design
emphasis.

### What they have in common

Both crates:

- attach unit information to a numeric storage type;
- make incompatible quantities distinct at compile time;
- derive dimensions through multiplication and division; and
- aim to prevent accidental unit mixing.

The important distinction is that neither crate replaces numerical reasoning.
Both preserve the normal behavior and limitations of the chosen numeric
representation.

### Why choose `runar-units`

`runar-units` is deliberately focused. Its quantity types and unit marker
types are visible in ordinary Rust signatures, and its implementation is
small enough to inspect or extend for a project's particular dimensions.
This can be useful when:

- the application needs a special set of dimensions, that can't be provided by `uom`, like 
\\(\sqrt L \\)
- explicit marker types are preferable to a more configurable abstraction
- a small API surface is valuable

The crate currently provides a focused collection of SI base dimensions and
selected derived dimensions rather than attempting to model every possible
measurement.

### Why choose `uom`

`uom` may be a better fit when an application needs a broader, more
comprehensive unit system. Its design provides extensive predefined coverage
and configuration for applications that work with many quantities, units, or
unit systems.

That broader scope can reduce the amount of unit infrastructure an application
needs to define itself. It also means that there are more concepts and
configuration choices to learn. Consult the [`uom` documentation](https://docs.rs/uom)
for its supported quantities, features, and version-specific behavior.

### Design trade-offs

| Concern | `runar-units` | `uom` |
| --- | --- | --- |
| Scope | Focused set of dimensions and units | Broad, configurable unit system |
| API style | Explicit quantity and marker types | More extensive abstraction and configuration |
| Learning curve | Smaller surface area | More concepts in exchange for wider coverage |
| Best fit | Focused or custom unit models | Applications needing broad unit coverage |

This is a design comparison, not a performance benchmark. Runtime behavior
depends on the operation, numeric type, compiler, target, and enabled features.

## What this crate does not try to do

`runar-units` is not a physics engine or a general validation framework. It
does not determine whether a value is physically realistic, and it does not
prevent numerical errors that are unrelated to dimensions.

It also does not hide unit changes behind implicit conversion. If a value must
be converted, the conversion should be visible in the code and chosen
deliberately.