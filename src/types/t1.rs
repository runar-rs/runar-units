use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::marker::PhantomData;
use crate::{Dimension, DimensionWithUnit, QuantityNum, Unit};

pub type Time<V, U> = T1<V, U>;
pub struct T1<V, U>
where V: QuantityNum, U: Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum, U: Unit> Dimension for T1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}

// Operators
/// Add with same ValueType and same Unit
impl<V: QuantityNum + Add<Output = V>, U: Unit> Add<T1<V, U>> for T1<V, U> {
    type Output = Self;
    fn add(self, rhs: T1<V, U>) -> Self::Output {
        T1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

/// AddAssign with same ValueType and same Unit
impl<V: QuantityNum, U: Unit> AddAssign<T1<V, U>> for T1<V, U> {
    fn add_assign(&mut self, rhs: T1<V, U>) {
        self.value += rhs.value;
    }
}

/// Sub with same ValueType and same Unit
impl<V: QuantityNum + Sub<Output = V>, U: Unit> Sub<T1<V, U>> for T1<V, U> {
    type Output = Self;
    fn sub(self, rhs: T1<V, U>) -> Self::Output {
        T1 {
            unit: PhantomData,
            value: self.value - rhs.value,
        }
    }
}

/// SubAssign with same ValueType and same Unit
impl<V: QuantityNum, U: Unit> SubAssign<T1<V, U>> for T1<V, U> {
    fn sub_assign(&mut self, rhs: T1<V, U>) {
        self.value -= rhs.value;
    }
}
// Units

pub struct Day;
impl Unit for Day {}

pub struct Hour;
impl Unit for Hour {}

pub struct Minute;
impl Unit for Minute {}
pub struct Second;
impl Unit for Second {}

/// Value is saved in the format, the getter requested.
impl<V: QuantityNum, U: Unit> DimensionWithUnit<U> for T1<V, U> {
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

/// Value is saved in second and getter wants minute
impl DimensionWithUnit<Minute> for T1<f64, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 60.0
    }
}

/// Value is saved in second and getter wants hour
impl DimensionWithUnit<Hour> for T1<f32, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 3600.0
    }
}

/// Value is saved in second and getter wants hour
impl DimensionWithUnit<Hour> for T1<f64, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 3600.0
    }
}

/// Value is saved in second and getter wants day
impl DimensionWithUnit<Day> for T1<f32, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 86400.0
    }
}

/// Value is saved in second and getter wants day
impl DimensionWithUnit<Day> for T1<f64, Second> {
    fn get(&self) -> Self::ValueType {
        self.value.clone() / 86400.0
    }
}