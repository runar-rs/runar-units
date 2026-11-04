use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::marker::PhantomData;
use core::fmt::Display;
use crate::{Dimension, QuantityNum, Unit};

pub type Time<V, U> = T1<V, U>;
/// T1 is a dimension for time, with a value type V and a unit U. The unit U must implement the
/// `T1Unit` trait, which is a marker trait for units that represent time.
#[derive(Debug, Clone, Copy)]
pub struct T1<V, U>
where V: QuantityNum, U: T1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum, U: T1Unit> Dimension for T1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

// Operators
/// Add with same `ValueType` and same Unit
impl<V: QuantityNum + Add<Output = V>, U: T1Unit> Add<T1<V, U>> for T1<V, U> {
    type Output = Self;
    fn add(self, rhs: T1<V, U>) -> Self::Output {
        T1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

/// `AddAssign` with same `ValueType` and same Unit
impl<V: QuantityNum + AddAssign, U: T1Unit> AddAssign<T1<V, U>> for T1<V, U> {
    fn add_assign(&mut self, rhs: T1<V, U>) {
        self.value += rhs.value;
    }
}

/// Sub with same `ValueType` and same Unit
impl<V: QuantityNum + Sub<Output = V>, U: T1Unit> Sub<T1<V, U>> for T1<V, U> {
    type Output = Self;
    fn sub(self, rhs: T1<V, U>) -> Self::Output {
        T1 {
            unit: PhantomData,
            value: self.value - rhs.value,
        }
    }
}

/// `SubAssign` with same `ValueType` and same Unit
impl<V: QuantityNum + SubAssign, U: T1Unit> SubAssign<T1<V, U>> for T1<V, U> {
    fn sub_assign(&mut self, rhs: T1<V, U>) {
        self.value -= rhs.value;
    }
}

impl<U: T1Unit> T1<f64, U> {
    #[must_use]
    pub fn new(value: f64) -> Self {
        T1 {
            unit: PhantomData,
            value
        }
    }
}

impl<V: QuantityNum + Display, U: T1Unit> core::fmt::Display for T1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}


// Units

/// Marker trait for units, that represent time.
pub trait T1Unit: Unit {}

/// Unit Day
/// 
/// # Implemented Conversions
/// - [`T1<f32, Day>`] to [`T1<f32, Hour>`]
/// - [`T1<f32, Day>`] to [`T1<f32, Minute>`]
/// - [`T1<f32, Day>`] to [`T1<f32, Second>`]
/// - [`T1<f64, Day>`] to [`T1<f32, Millisecond>`]
/// - [`T1<f64, Day>`] to [`T1<f64, Hour>`]
/// - [`T1<f64, Day>`] to [`T1<f64, Minute>`]
/// - [`T1<f64, Day>`] to [`T1<f64, Second>`]
/// - [`T1<f64, Day>`] to [`T1<f64, Millisecond>`]
#[derive(Debug, Clone, Copy)]
pub struct Day;
impl Unit for Day {
    const ABBREVIATION: &'static str = "d";
}
impl T1Unit for Day {}

impl Day {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Day> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Hour
/// 
/// # Implemented Conversions
/// - [`T1<f32, Hour>`] to [`T1<f32, Day>`]
/// - [`T1<f32, Hour>`] to [`T1<f32, Minute>`]
/// - [`T1<f32, Hour>`] to [`T1<f32, Second>`]
/// - [`T1<f32, Hour>`] to [`T1<f32, Millisecond>`]
/// - [`T1<f64, Hour>`] to [`T1<f64, Day>`]
/// - [`T1<f64, Hour>`] to [`T1<f64, Minute>`]
/// - [`T1<f64, Hour>`] to [`T1<f64, Second>`]
/// - [`T1<f64, Hour>`] to [`T1<f64, Millisecond>`]
#[derive(Debug, Clone, Copy)]
pub struct Hour;
impl Unit for Hour {
    const ABBREVIATION: &'static str = "h";
}
impl T1Unit for Hour {}

impl Hour {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Hour> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Minute
/// 
/// # Implemented Conversions
/// - [`T1<f32, Minute>`] to [`T1<f32, Day>`]
/// - [`T1<f32, Minute>`] to [`T1<f32, Hour>`]
/// - [`T1<f32, Minute>`] to [`T1<f32, Second>`]
/// - [`T1<f32, Minute>`] to [`T1<f32, Millisecond>`]
/// - [`T1<f64, Minute>`] to [`T1<f64, Day>`]
/// - [`T1<f64, Minute>`] to [`T1<f64, Hour>`]
/// - [`T1<f64, Minute>`] to [`T1<f64, Second>`]
/// - [`T1<f64, Minute>`] to [`T1<f64, Millisecond>`]
#[derive(Debug, Clone, Copy)]
pub struct Minute;
impl Unit for Minute {
    const ABBREVIATION: &'static str = "min";
}
impl T1Unit for Minute {}

impl Minute {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Minute> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}

/// Unit Second
/// 
/// # Implemented Conversions
/// - [`T1<f32, Second>`] to [`T1<f32, Day>`]
/// - [`T1<f32, Second>`] to [`T1<f32, Hour>`]
/// - [`T1<f32, Second>`] to [`T1<f32, Minute>`]
/// - [`T1<f32, Second>`] to [`T1<f32, Millisecond>`]
/// - [`T1<f32, Second>`] to [`T1<f32, Microsecond>`]
/// - [`T1<f64, Second>`] to [`T1<f64, Day>`]
/// - [`T1<f64, Second>`] to [`T1<f64, Hour>`]
/// - [`T1<f64, Second>`] to [`T1<f64, Minute>`]
/// - [`T1<f64, Second>`] to [`T1<f64, Millisecond>`]
/// - [`T1<f64, Second>`] to [`T1<f64, Microsecond>`]
#[derive(Debug, Clone, Copy)]
pub struct Second;
impl Unit for Second {
    const ABBREVIATION: &'static str = "s";
}
impl T1Unit for Second {}

impl Second {
    /// Creates a new instance of `T1` with the given value and the unit `Second`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Second> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}

/// Unit Millisecond
#[derive(Debug, Clone, Copy)]
pub struct Millisecond;
impl Unit for Millisecond {
    const ABBREVIATION: &'static str = "ms";
}
impl T1Unit for Millisecond {}

impl Millisecond {
    /// Creates a new instance of `T1` with the given value and the unit `Millisecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Millisecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}

/// Unit Microsecond
#[derive(Debug, Clone, Copy)]
pub struct Microsecond;
impl Unit for Microsecond {
    const ABBREVIATION: &'static str = "μs";
}
impl T1Unit for Microsecond {}

impl Microsecond {
    /// Creates a new instance of `T1` with the given value and the unit `Microsecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Microsecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}

/// Unit Nanosecond
#[derive(Debug, Clone, Copy)]
pub struct Nanosecond;
impl Unit for Nanosecond {
    const ABBREVIATION: &'static str = "ns";
}
impl T1Unit for Nanosecond {}

impl Nanosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Nanosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Nanosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Picosecond
#[derive(Debug, Clone, Copy)]
pub struct Picosecond;
impl Unit for Picosecond {
    const ABBREVIATION: &'static str = "ps";
}
impl T1Unit for Picosecond {}

impl Picosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Picosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Picosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Femtosecond
#[derive(Debug, Clone, Copy)]
pub struct Femtosecond;
impl Unit for Femtosecond {
    const ABBREVIATION: &'static str = "fs";
}
impl T1Unit for Femtosecond {}

impl Femtosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Femtosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Femtosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Attosecond
#[derive(Debug, Clone, Copy)]
pub struct Attosecond;
impl Unit for Attosecond {
    const ABBREVIATION: &'static str = "as";
}
impl T1Unit for Attosecond {}

impl Attosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Attosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Attosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Zeptosecond
#[derive(Debug, Clone, Copy)]
pub struct Zeptosecond;
impl Unit for Zeptosecond {
    const ABBREVIATION: &'static str = "zs";
}
impl T1Unit for Zeptosecond {}

impl Zeptosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Zeptosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Zeptosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Yoktosecond
#[derive(Debug, Clone, Copy)]
pub struct Yoktosecond;
impl Unit for Yoktosecond {
    const ABBREVIATION: &'static str = "ys";
}
impl T1Unit for Yoktosecond {}

impl Yoktosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Yoktosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Yoktosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Rontosecond
#[derive(Debug, Clone, Copy)]
pub struct Rontosecond;
impl Unit for Rontosecond {
    const ABBREVIATION: &'static str = "rs";
}
impl T1Unit for Rontosecond {}

impl Rontosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Rontosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Rontosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
/// Unit Quectosecond
#[derive(Debug, Clone, Copy)]
pub struct Quectosecond;
impl Unit for Quectosecond {
    const ABBREVIATION: &'static str = "qs";
}
impl T1Unit for Quectosecond {}

impl Quectosecond {
    /// Creates a new instance of `T1` with the given value and the unit `Quectosecond`.
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> T1<V, Quectosecond> {
        T1 {
            unit: PhantomData,
            value
        }
    }
}
impl From<T1<f32, Day>> for T1<f32, Hour> {
    fn from(val: T1<f32, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 24.0
        }
    }
}

impl From<T1<f32, Day>> for T1<f32, Minute> {
    fn from(val: T1<f32, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 1440.0
        }
    }
}

impl From<T1<f32, Day>> for T1<f32, Second> {
    fn from(val: T1<f32, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 86400.0,
        }
    }
}

impl From<T1<f32, Day>> for T1<f32, Millisecond> {
    fn from(val: T1<f32, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 86400000.0,
        }
    }
}

impl From<T1<f64, Day>> for T1<f64, Hour> {
    fn from(val: T1<f64, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 24.0
        }
    }
}

impl From<T1<f64, Day>> for T1<f64, Minute> {
    fn from(val: T1<f64, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 1440.0
        }
    }
}

impl From<T1<f64, Day>> for T1<f64, Second> {
    fn from(val: T1<f64, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 86400.0
        }
    }
}

impl From<T1<f64, Day>> for T1<f64, Millisecond> {
    fn from(val: T1<f64, Day>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 86400000.0
        }
    }
}

impl From<T1<f32, Hour>> for T1<f32, Day> {
    fn from(val: T1<f32, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 24.0
        }
    }
}

impl From<T1<f32, Hour>> for T1<f32, Minute> {
    fn from(val: T1<f32, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 60.0
        }
    }
}

impl From<T1<f32, Hour>> for T1<f32, Second> {
    fn from(val: T1<f32, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 3600.0
        }
    }
}

impl From<T1<f32, Hour>> for T1<f32, Millisecond> {
    fn from(val: T1<f32, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 3600000.0
        }
    }
}

impl From<T1<f64, Hour>> for T1<f64, Day> {
    fn from(val: T1<f64, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 24.0
        }
    }
}

impl From<T1<f64, Hour>> for T1<f64, Minute> {
    fn from(val: T1<f64, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 60.0
        }
    }
}

impl From<T1<f64, Hour>> for T1<f64, Second> {
    fn from(val: T1<f64, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 3600.0
        }
    }
}

impl From<T1<f64, Hour>> for T1<f64, Millisecond> {
    fn from(val: T1<f64, Hour>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 3600000.0
        }
    }
}

impl From<T1<f32, Minute>> for T1<f32, Day> {
    fn from(val: T1<f32, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 1440.0
        }
    }
}

impl From<T1<f32, Minute>> for T1<f32, Hour> {
    fn from(val: T1<f32, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 60.0
        }
    }
}

impl From<T1<f32, Minute>> for T1<f32, Second> {
    fn from(val: T1<f32, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 60.0
        }
    }
}

impl From<T1<f32, Minute>> for T1<f32, Millisecond> {
    fn from(val: T1<f32, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 60000.0
        }
    }
}

impl From<T1<f64, Minute>> for T1<f64, Day> {
    fn from(val: T1<f64, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 1440.0
        }
    }
}

impl From<T1<f64, Minute>> for T1<f64, Hour> {
    fn from(val: T1<f64, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 60.0
        }
    }
}

impl From<T1<f64, Minute>> for T1<f64, Second> {
    fn from(val: T1<f64, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 60.0
        }
    }
}

impl From<T1<f64, Minute>> for T1<f64, Millisecond> {
    fn from(val: T1<f64, Minute>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 60000.0
        }
    }
}

impl From<T1<f32, Second>> for T1<f32, Day> {
    fn from(val: T1<f32, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 86400.0
        }
    }
}

impl From<T1<f32, Second>> for T1<f32, Hour> {
    fn from(val: T1<f32, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 3600.0
        }
    }
}

impl From<T1<f32, Second>> for T1<f32, Minute> {
    fn from(val: T1<f32, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 60.0
        }
    }
}

impl From<T1<f32, Second>> for T1<f32, Millisecond> {
    fn from(val: T1<f32, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 1000.0
        }
    }
}

impl From<T1<f32, Second>> for T1<f32, Microsecond> {
    fn from(val: T1<f32, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 1_000_000.0
        }
    }
}

impl From<T1<f64, Second>> for T1<f64, Day> {
    fn from(val: T1<f64, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 86400.0
        }
    }
}

impl From<T1<f64, Second>> for T1<f64, Hour> {
    fn from(val: T1<f64, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 3600.0
        }
    }
}

impl From<T1<f64, Second>> for T1<f64, Minute> {
    fn from(val: T1<f64, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value / 60.0
        }
    }
}

impl From<T1<f64, Second>> for T1<f64, Millisecond> {
    fn from(val: T1<f64, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 1000.0
        }
    }
}

impl From<T1<f64, Second>> for T1<f64, Microsecond> {
    fn from(val: T1<f64, Second>) -> Self {
        T1 {
            unit: PhantomData,
            value: val.value * 1_000_000.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::assert_close, types::t1::Second};

    use super::*;

    #[test]
    fn test_t1_display() {
        let time = Second::new(42.0);
        assert_eq!(format!("{}", time), "42 s");
    }

    #[test]
    fn test_t1_display_precision() {
        let time = Second::new(42.123456789);
        assert_eq!(format!("{:.1}", time), "42.1 s");
        assert_eq!(format!("{:.2}", time), "42.12 s");
        assert_eq!(format!("{:.3}", time), "42.123 s");
    }

    #[test]
    fn test_t1_display_sign() {
        let time = Second::new(42.0);
        assert_eq!(format!("{:+}", time), "+42 s");
        let time = Second::new(-42.0);
        assert_eq!(format!("{:+}", time), "-42 s");
    }

    #[test]
    fn test_t1_add() {
        let a = Second::new(5.0);
        let b = Second::new(3.0);
        let c = a + b;
        assert_close(c.value, 8.0, 1e-10);
    }

    #[test]
    fn test_t1_add_associativity() {
        let a = Second::new(5.0);
        let b = Second::new(3.0);
        let c = Second::new(2.0);

        let result1 = (a + b) + c;
        let result2 = a + (b + c);

        assert_close(result1.value, result2.value, 1e-10);
    }

    #[test]
    fn test_t1_add_commutativity() {
        let a = Second::new(5.0);
        let b = Second::new(3.0);

        let result1 = a + b;
        let result2 = b + a;

        assert_close(result1.value, result2.value, 1e-10);
    }

    #[test]
    fn test_t1_sub() {
        let a = Second::new(5.0);
        let b = Second::new(3.0);
        let c = a - b;
        assert_close(c.value, 2.0, 1e-10);
    }

    #[test]
    fn test_t1_sub_associativity() {
        let a = Second::new(5.0);
        let b = Second::new(3.0);
        let c = Second::new(2.0);

        let result1 = (a - b) - c;
        let result2 = a - (b + c);

        assert_close(result1.value, result2.value, 1e-10);
    }

    #[test]
    fn test_t1_sub_commutativity() {
        let a = Second::new(5.0);
        let b = Second::new(3.0);

        let result1 = a - b;
        let result2 = b - a;

        assert_close(result1.value, -result2.value, 1e-10);
    }

    #[test]
    fn test_t1_units_conversion() {
        let time_in_hours = Hour::new(2.0);
        let time_in_minutes: T1<f64, Minute> = time_in_hours.into();
        let time_in_seconds: T1<f64, Second> = time_in_minutes.into();

        let time_in_hours_back: T1<f64, Hour> = time_in_seconds.into();
        assert_close(time_in_hours.value, time_in_hours_back.value, 1e-10);
    }
}