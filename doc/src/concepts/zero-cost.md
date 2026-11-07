# Zero-cost abstractions

`runar-units` keeps unit information in types rather than in every value.
Quantity types use marker types and `PhantomData` to associate a numeric value
with its dimension and unit:

```rust
pub struct L1<V, U> {
    value: V,
    unit: core::marker::PhantomData<U>,
}
```

`PhantomData<U>` does not store a unit at runtime. It exists for the compiler:
it lets Rust select the correct arithmetic and conversion implementations
while the generated value contains only `V`. Ordinary addition, subtraction,
multiplication, and division therefore operate directly on the underlying
numeric value without a runtime unit tag, lookup table, or dynamic dispatch.

## Linear conversions

For linear units, each unit declares a compile-time `SCALE_TO_BASE` factor.
For example, a kilometre has a factor of `1e3` relative to metres, while a
centimetre has a factor of `1e-2`. A conversion between two units computes the
ratio of their factors:

```text
conversion_factor = source_scale_to_base / target_scale_to_base
```

The conversion implementation then reduces to the underlying value multiplied
by that factor:

```text
converted_value = value * conversion_factor
```

For example, converting `2.0` kilometres to metres is equivalent to:

```text
2.0 * (1000.0 / 1.0)
```

The source and target units are known at compile time. Consequently, after
monomorphization and optimization, the conversion is essentially the numeric
operation `value * conversion_factor`; no unit object needs to be created or
inspected at runtime. The exact machine code still depends on the numeric type,
compiler, optimization settings, and target platform.

The same principle applies when arithmetic produces a derived dimension.
`runar-units` computes the appropriate scale factor from the operand and output
units, then performs the operation on the underlying values. Type-level
information improves correctness without requiring a runtime representation of
the unit.

“Zero-cost” does not mean that numeric operations or conversions cost
nothing. Multiplication and any required floating-point rounding still have
their normal costs. It means that the compile-time safety information does not
add a separate runtime unit representation or a general dispatch mechanism to
ordinary operations.
