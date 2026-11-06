//! Force
//! 

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type Force<V, U> = L1_M1_TN2<V, U>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct L1_M1_TN2<V, U>
where V: QuantityNum, U: L1_M1_TN2Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum + Display, U: L1_M1_TN2Unit> Display for L1_M1_TN2<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: L1_M1_TN2Unit> Dimension for L1_M1_TN2<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L1_M1_TN2, L1_M1_TN2Unit);

#[allow(non_camel_case_types)]
pub trait L1_M1_TN2Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Newton;
impl Unit for Newton {
    const ABBREVIATION: &'static str = "N";
}
impl L1_M1_TN2Unit for Newton {}

impl Newton {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1_TN2<V, Newton> {
        L1_M1_TN2 {
            unit: PhantomData,
            value
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KilogramMeterPerSecondSquared;
impl Unit for KilogramMeterPerSecondSquared {
    const ABBREVIATION: &'static str = "kg·m/s²";
}
impl L1_M1_TN2Unit for KilogramMeterPerSecondSquared {}

impl KilogramMeterPerSecondSquared {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1_TN2<V, KilogramMeterPerSecondSquared> {
        L1_M1_TN2 {
            unit: PhantomData,
            value
        }
    }
}

impl_linear_unit_conversions!(L1_M1_TN2; Newton, KilogramMeterPerSecondSquared);