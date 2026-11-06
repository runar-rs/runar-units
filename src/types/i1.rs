//! Electric Current

use core::fmt::Display;
use core::marker::PhantomData;
use crate::{Dimension, QuantityNum, Unit};

pub type ElectricCurrent<V, U> = I1<V, U>;

#[derive(Debug, Clone, Copy)]
pub struct I1<V, U>
where V: QuantityNum, U: I1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum + Display, U: I1Unit> core::fmt::Display for I1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: I1Unit> Dimension for I1<V, U> {
    type ValueType = V;

    /// Returns a reference to the underlying value of the dimension.
    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(I1, I1Unit);

// Units

/// Marker trait for `I1` units.
pub trait I1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Ampere;
impl Unit for Ampere {
    const ABBREVIATION: &'static str = "A";
}
impl I1Unit for Ampere {}

impl Ampere {
    pub fn new<V: QuantityNum>(value: V) -> I1<V, Ampere> {
        I1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Milliampere;
impl Unit for Milliampere {
    const ABBREVIATION: &'static str = "mA";
    const SCALE_TO_BASE: f64 = 1e-3;
}
impl I1Unit for Milliampere {}

impl Milliampere {
    pub fn new<V: QuantityNum>(value: V) -> I1<V, Milliampere> {
        I1 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Microampere;
impl Unit for Microampere {
    const ABBREVIATION: &'static str = "µA";
    const SCALE_TO_BASE: f64 = 1e-6;
}
impl I1Unit for Microampere {}

impl Microampere {
    pub fn new<V: QuantityNum>(value: V) -> I1<V, Microampere> {
        I1 {
            unit: PhantomData,
            value
        }
    }
}

impl_linear_unit_conversions!(I1; Ampere, Milliampere);