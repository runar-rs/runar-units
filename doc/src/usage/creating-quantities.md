# Creating quantities

There are two ways to construct a quantity. The preferred, unit-oriented form
calls `new` on the unit marker type:

```rust
use runar_units::types::l1::Meter;
use runar_units::types::t1::Second;

let distance = Meter::new(10.0_f64);
let duration = Second::new(2.0_f64);
```

This style makes the unit immediately visible at the construction site. It
returns `Length<f64, Meter>` and `Time<f64, Second>` respectively.

You can also call `new` on the quantity type itself and specify both generic
parameters explicitly:

```rust
use runar_units::types::l1::{Length, Meter};
use runar_units::types::t1::{Second, Time};

let distance = Length::<f64, Meter>::new(10.0);
let duration = Time::<f64, Second>::new(2.0);
```

Here the first parameter is the numeric value type and the second is the unit
marker type. Both forms create the same quantity and have the same runtime
behavior; choose the unit constructor when readability is the priority, and
the generic quantity constructor when the complete type needs to be explicit.
