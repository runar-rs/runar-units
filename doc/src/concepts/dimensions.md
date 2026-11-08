# Dimensions

A dimension describes the physical meaning of a quantity independently of its
unit or numeric representation. For example, metres and kilometres are both
units of the length dimension, while seconds and hours belong to the time
dimension. Arithmetic combines dimensions: multiplying two lengths produces an
area, while dividing a length by a time produces a velocity.

## Base dimensions

`runar-units` builds dimensions from the SI base dimensions:
- Length `L`
- Mass `M`
- Time `T`
- Electric current `I`
- Thermodynamic temperature `Theta`,
- Amount of substance `N`
- Luminous intensity `J`

### Additional crate dimensions

In addition to the SI base dimensions, the crate defines steradian (`Sr`) as a
separate dimension for solid angle. Although the SI system treats the
steradian as dimensionless, keeping it distinct allows APIs to distinguish
solid-angle quantities from other dimensionless values and helps avoid
confusion in quantities such as radiance.

The crate also provides selected derived dimensions, including area, volume,
velocity, momentum, force, and related quantities. The complete set is listed
in the [`types` module](https://docs.rs/runar-units/latest/runar_units/types/index.html).

### User-defined dimensions

The same model can be extended for a domain-specific dimension that is not
provided by the crate. A custom dimension should have its own quantity type
and unit marker types, implement the public [`Dimension`](https://docs.rs/runar-units/latest/runar_units/trait.Dimension.html)
and [`Unit`](https://docs.rs/runar-units/latest/runar_units/trait.Unit.html)
traits, and define the arithmetic and conversions that are meaningful for that
domain. Keeping the new dimension distinct from existing ones is important:
the compiler should not allow values to mix merely because they use the same
underlying numeric type.

For custom dimensions, prefer following the implementation patterns used by
the modules in `src/types`. In particular, give the dimension a dedicated
marker trait for its units and make conversions explicit. If custom
dimension-to-dimension arithmetic is required, it must also be implemented
explicitly so that the resulting type and unit are unambiguous.

## Derived dimensions

Derived dimensions are formed by multiplying or dividing base dimensions. For
example, area is length squared (`L2`), velocity is length divided by time
(`L1_TN1`), and force is mass multiplied by length divided by time squared
(`L1_M1_TN2`). The type records the relationship, so the compiler can select
the corresponding arithmetic implementation.

A few examples of the derived dimensions the crate provides:

| Rust dimension | Meaning | Example unit |
| --- | --- | --- |
| `L1_T1`       | \\(LT\\)       | metre-second |
| `L1_TN1`      | \\(LT^{-1}\\)  | metre per second |
| `L1_M1`       | \\(LM\\)       | metre-kilogram |
| `L1_M1_TN1`   | \\(LMT^{-1}\\) | newton-second |
| `L1_M1_TN2`   | \\(LMT^{-2}\\) | newton |
| `L2`          | \\(L^2\\)      | square metre |
| `L3`          | \\(L^3\\)      | cubic metre |
| `TN1`         | \\(T^{-1}\\)   | hertz |

These are dimension types, not merely display names. For example, a value of
type `L1_TN1` can be represented using a unit such as metres per second, while
its numeric storage type can still be `f32` or `f64`.

## Nomenclature

Dimension type names use a compact notation based on the exponents of the
base dimensions:

- Each base dimension is represented by a letter: `I`, `J`, `L`, `M`, or `N`,
  while time is `T` and thermodynamic temperature is `Theta`.
- `1` means a positive exponent of one. For example, `L1` is length.
- `n1` means a negative exponent of one. For example, `TN1` is inverse time.
- Higher positive or negative exponents are written in the same position,
  such as `T2` or `TN2`.
- Multiple dimensions are separated by underscores and sorted alphabetically.
  Thus `L1_M1_TN2` means \\(L^1 M^1 T^{-2} \\).

The names describe dimensions rather than units: `L1_M1_TN2` can represent a
force in newtons or in the equivalent base-unit form kilograms-metres per
second squared. The nomenclature is an implementation-facing convention and
may evolve as the set of supported dimensions grows.