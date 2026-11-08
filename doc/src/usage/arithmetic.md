# Arithmetic

Every dimension receives a common set of arithmetic operations. A quantity can
be added to or subtracted from another quantity of the same type, including
the same unit. It can also be multiplied by or divided by its underlying
numeric value, and supports the corresponding assignment forms (`+=`, `-=`,
`*=`, and `/=`). Dividing one quantity by another quantity of the same type
produces the underlying numeric value:

```rust
use runar_units::types::l1::Meter;

let mut distance = Meter::new(10.0_f64);
distance += Meter::new(2.0);
distance -= Meter::new(1.0);
let doubled = distance * 2.0;
let halved = doubled / 2.0;
let ratio = halved / Meter::new(11.0);

assert_eq!(distance.get(), &11.0);
assert_eq!(ratio, 1.0);
```

These operations preserve the quantity's dimension and unit, except for
quantity-by-quantity division, which returns a scalar. Operations between
different dimensions are not provided by this default implementation, so an
invalid expression such as adding a length to a time fails to compile.

## Multiplying and dividing dimensions

Derived dimensions use the `impl_dimension_mul_with_unit!` and
`impl_dimension_div_with_unit!` macros. Despite their longer names, these are
the macros used by the crate to define multiplication and division between
different quantity types with an explicit output unit.

`impl_dimension_mul_with_unit!` generates an implementation of `Mul` for a
left-hand quantity and a right-hand quantity. It returns the requested output
quantity and output unit:

```text
impl_dimension_mul_with_unit!(
    crate::types::l1::L1,
    crate::types::l1::L1Unit,
    crate::types::t1::T1,
    crate::types::t1::T1Unit
        => crate::types::l1_t1::L1_T1,
    crate::types::l1_t1::MeterSecond
);
```

Conceptually, this defines:

```text
L1<V, LengthUnit> * T1<V, TimeUnit>
    -> L1_T1<V, MeterSecond>
```

The generated implementation scales the numeric result using the operand
units and the selected output unit:

```text
lhs.value * rhs.value
    * lhs_scale_to_base
    * rhs_scale_to_base
    / output_scale_to_base
```

`impl_dimension_div_with_unit!` follows the same pattern for division:

```text
impl_dimension_div_with_unit!(
    crate::types::l1::L1,
    crate::types::l1::L1Unit,
    crate::types::t1::T1,
    crate::types::t1::T1Unit
        => crate::types::l1_tn1::L1_TN1,
    crate::types::l1_tn1::MeterPerSecond
);
```

This defines a length divided by a time as a velocity and scales the result
according to:

```text
lhs.value / rhs.value
    * lhs_scale_to_base
    / rhs_scale_to_base
    / output_scale_to_base
```

The macros are useful when defining a new dimension or a derived quantity with
a particular canonical unit. The shorter `impl_dimension_mul!` and
`impl_dimension_div!` forms can be used when the output unit should be resolved
automatically from the derived dimension's registered base unit.
