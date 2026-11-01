# runar-units

`runar-units` provides type-safe measurement units for Rust without adding a
runtime representation for the unit itself.

The central idea is to make invalid combinations of quantities impossible to
express, while compiling the abstractions away so the generated code operates
on the underlying numeric value.

## Why units belong in the type system

A plain number does not communicate what it represents:

```rust
fn travel(distance: f64, duration: f64) -> f64 {
    distance / duration
}
```

This function accepts any two `f64` values. It cannot distinguish metres from
seconds, or detect that the arguments were accidentally swapped.

With `runar-units`, the unit is part of the type:

```rust
use runar_units::{Dimension, types::l1::{Length, Meter}};
use runar_units::types::t1::{Second, Time};

fn travel(distance: Length<f64, Meter>, duration: Time<f64, Second>) {
    let speed = distance / duration;
    // `speed` has a metres-per-second type.
}
```

The compiler knows that `distance` is a length in metres and `duration` is a
time in seconds. Operations are implemented only for meaningful combinations,
so adding a length to a time, or multiplying incompatible dimensions, is
rejected during compilation rather than discovered through a test or a
production failure.

## The zero-cost abstraction

Units such as `Meter` and `Second` are marker types. They carry compile-time
information but no per-value data. Quantity types use `PhantomData` to associate
that marker with a numeric value:

```rust
pub struct L1<V, U> {
    value: V,
    unit: core::marker::PhantomData<U>,
}
```

`PhantomData<U>` does not allocate memory or store a unit at runtime. It exists
for type checking, trait selection, and documentation. After compilation,
operations such as addition and division are reduced to operations on `V`:

```text
Length<f64, Meter> / Time<f64, Second>
        compile-time type information
                      |
                      v
                 f64 / f64
```

This is the meaning of *zero-cost abstraction* here: the type-level unit
information improves correctness without requiring a unit tag, lookup, dynamic
dispatch, or conversion on every arithmetic operation. The numeric type still
determines the normal cost and precision of the calculation.

## Safety through dimensional types

The library uses generic quantity types and marker traits to express
dimensions. For example, a length and a time are distinct types even when they
use the same underlying `f64`:

```rust,compile_fail
use runar_units::types::l1::{Length, Meter};
use runar_units::types::t1::{Second, Time};

let distance = Length::<f64, Meter>::new(10.0);
let duration = Time::<f64, Second>::new(2.0);

let invalid = distance + duration;
```

The example fails at compile time because no `Add` implementation exists for
those different dimensions. In contrast, operations with matching dimensions
are accepted:

```rust
use runar_units::{Dimension, types::l1::{Length, Meter}};

let first = Length::<f64, Meter>::new(10.0);
let second = Length::<f64, Meter>::new(2.0);
let total = first + second;

assert_eq!(total.get_raw(), &12.0);
```

The safety provided by the type system includes:

- **Unit-aware APIs:** function signatures document the quantities they accept.
- **Invalid arithmetic rejection:** unsupported dimensional combinations do not
  compile.
- **No unit mixing by accident:** metres and seconds are not interchangeable
  merely because both use `f64`.
- **Refactoring confidence:** changing a unit or dimension propagates through
  compiler errors instead of silently changing the meaning of a calculation.
- **No runtime type checks:** correctness checks happen during compilation.

This safety is about dimensional correctness. It does not prevent every
numeric problem: floating-point rounding, overflow, underflow, division by
zero, and physically unreasonable values still need to be handled according
to the application’s requirements.

## Composable derived quantities

Arithmetic can produce a new type that describes the result. For example,
dividing metres by seconds yields metres per second, and the result can then be
passed to an API that specifically requires that dimension:

```rust
use runar_units::{Dimension, types::l1::{Length, Meter}};
use runar_units::types::t1::{Second, Time};

let distance = Length::<f64, Meter>::new(100.0);
let duration = Time::<f64, Second>::new(20.0);
let speed = distance / duration;

assert_eq!(speed.get_raw(), &5.0);
```

The same principle applies to other dimensions and units exposed by the crate.
The type describes the meaning of the value, while the stored representation
remains the numeric value.

## `no_std`

Outside of tests, the crate is built with `no_std`. It uses core language and
library facilities for its quantity types and arithmetic, making the
type-level unit approach suitable for environments where the standard library
is unavailable.

## Current scope

`runar-units` is focused on representing dimensions and units at compile time.
Conversions and additional derived dimensions can be added as explicit,
type-safe operations. The absence of an implicit conversion is intentional:
unit changes should be visible in code rather than hidden behind a runtime
coercion.
