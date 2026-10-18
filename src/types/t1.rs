use core::marker::PhantomData;
use crate::{Dimension, DimensionWithUnit, QuantityNum, Unit};

pub type Time<V, U> = T1<V, U>;
pub struct T1<V, U>
where V: QuantityNum, U: Unit {
    unit: PhantomData<U>,
    value: V
}

impl<V: QuantityNum, U: Unit> Dimension for T1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
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