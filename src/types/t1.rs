use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::marker::PhantomData;
use core::fmt::Display;
use crate::{Dimension, DimensionWithUnit, QuantityNum, Unit};

pub type Time<V, U> = T1<V, U>;
/// T1 is a dimension for time, with a value type V and a unit U. The unit U must implement the
/// T1Unit trait, which is a marker trait for units that represent time.
pub struct T1<V, U>
where V: QuantityNum, U: T1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum, U: T1Unit> Dimension for T1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}

// Operators
/// Add with same ValueType and same Unit
impl<V: QuantityNum + Add<Output = V>, U: T1Unit> Add<T1<V, U>> for T1<V, U> {
    type Output = Self;
    fn add(self, rhs: T1<V, U>) -> Self::Output {
        T1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

/// AddAssign with same ValueType and same Unit
impl<V: QuantityNum, U: T1Unit> AddAssign<T1<V, U>> for T1<V, U> {
    fn add_assign(&mut self, rhs: T1<V, U>) {
        self.value += rhs.value;
    }
}

/// Sub with same ValueType and same Unit
impl<V: QuantityNum + Sub<Output = V>, U: T1Unit> Sub<T1<V, U>> for T1<V, U> {
    type Output = Self;
    fn sub(self, rhs: T1<V, U>) -> Self::Output {
        T1 {
            unit: PhantomData,
            value: self.value - rhs.value,
        }
    }
}

/// SubAssign with same ValueType and same Unit
impl<V: QuantityNum, U: T1Unit> SubAssign<T1<V, U>> for T1<V, U> {
    fn sub_assign(&mut self, rhs: T1<V, U>) {
        self.value -= rhs.value;
    }
}
// Units

/// Marker trait for units, that represent time.
pub trait T1Unit: Unit {}

/// Unit Day
/// 
/// # Implemented Conversions
/// ## To Day
/// 
/// ## From Day
pub struct Day;
impl Unit for Day {}
impl T1Unit for Day {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Day> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} d", self.value)
    }
}

pub struct Hour;
impl Unit for Hour {}
impl T1Unit for Hour {}

/// Display for T1 in Hour
impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Hour> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} h", self.value)
    }
}

pub struct Minute;
impl Unit for Minute {}
impl T1Unit for Minute {}

/// Display for T1 in Minute
impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Minute> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} m", self.value)
    }
}

pub struct Second;
impl Unit for Second {}
impl T1Unit for Second {}

/// Display for T1 in Second
impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Second> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} s", self.value)
    }
}

pub struct Millisecond;
impl Unit for Millisecond {}
impl T1Unit for Millisecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Millisecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ms", self.value)
    }
}

pub struct Microsecond;
impl Unit for Microsecond {}
impl T1Unit for Microsecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Microsecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} μs", self.value)
    }
}

pub struct Nanosecond;
impl Unit for Nanosecond {}
impl T1Unit for Nanosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Nanosecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ns", self.value)
    }
}

pub struct Picosecond;
impl Unit for Picosecond {}
impl T1Unit for Picosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Picosecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ps", self.value)
    }
}

pub struct Femtosecond;
impl Unit for Femtosecond {}
impl T1Unit for Femtosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Femtosecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} fs", self.value)
    }
}

/// Unit Attosecond
pub struct Attosecond;
impl Unit for Attosecond {}
impl T1Unit for Attosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Attosecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} as", self.value)
    }
}

/// Unit Zeptosecond
pub struct Zeptosecond;
impl Unit for Zeptosecond {}
impl T1Unit for Zeptosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Zeptosecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} zs", self.value)
    }
}

/// Unit Yoktosecond
pub struct Yoktosecond;
impl Unit for Yoktosecond {}
impl T1Unit for Yoktosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Yoktosecond> {  
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ys", self.value)
    }
}

/// Unit Rontosecond
pub struct Rontosecond;
impl Unit for Rontosecond {}
impl T1Unit for Rontosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Rontosecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} rs", self.value)
    }
}

/// Unit Quectosecond
pub struct Quectosecond;
impl Unit for Quectosecond {}
impl T1Unit for Quectosecond {}

impl<V: QuantityNum + Display> core::fmt::Display for T1<V, Quectosecond> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} qs", self.value)
    }
}

/// Value is saved in the format, the getter requested.
impl<V: QuantityNum, U: T1Unit> DimensionWithUnit<U> for T1<V, U> {
    fn get(&self) -> Self::ValueType {
        self.value.clone()
    }
}

/// Value is saved in second and getter wants minute
impl DimensionWithUnit<Minute> for T1<f32, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 60.0
    }
}

impl Into<T1<f32, Second>> for T1<f32, Minute> {
    fn into(self) -> T1<f32, Second> {
        T1 {
            unit: PhantomData,
            value: self.value / 60.0
        }
    }
}

/// Value is saved in second and getter wants minute
impl DimensionWithUnit<Minute> for T1<f64, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 60.0
    }
}

impl Into<T1<f64, Second>> for T1<f64, Minute> {
    fn into(self) -> T1<f64, Second> {
        T1 {
            unit: PhantomData,
            value: self.value / 60.0
        }
    }
}

/// Value is saved in second and getter wants hour
impl DimensionWithUnit<Hour> for T1<f32, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 3600.0
    }
}

impl Into<T1<f32, Second>> for T1<f32, Hour> {
    fn into(self) -> T1<f32, Second> {
        T1 {
            unit: PhantomData,
            value: self.value / 3600.0
        }
    }
}

/// Value is saved in second and getter wants hour
impl DimensionWithUnit<Hour> for T1<f64, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 3600.0
    }
}

impl Into<T1<f64, Second>> for T1<f64, Hour> {
    fn into(self) -> T1<f64, Second> {
        T1 {
            unit: PhantomData,
            value: self.value / 3600.0
        }
    }
}

/// Value is saved in second and getter wants day
impl DimensionWithUnit<Day> for T1<f32, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 86400.0
    }
}

impl Into<T1<f32, Second>> for T1<f32, Day> {
    fn into(self) -> T1<f32, Second> {
        T1 {
            unit: PhantomData,
            value: self.value / 86400.0,
        }
    }
}

/// Value is saved in second and getter wants day
impl DimensionWithUnit<Day> for T1<f64, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 86400.0
    }
}

impl Into<T1<f64, Second>> for T1<f64, Day> {
    fn into(self) -> T1<f64, Second> {
        T1 {
            unit: PhantomData,
            value: self.value / 86400.0
        }
    }
}