use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

#[derive(Debug, Clone, Copy)]
pub struct L3<V, U>
where V: QuantityNum, U: L3Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum + Display, U: L3Unit> core::fmt::Display for L3<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: L3Unit> Dimension for L3<V, U> {
    type ValueType = V;

    /// Returns a reference to the underlying value of the dimension.
    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(L3, L3Unit);

pub trait L3Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct CubicMeter;
impl Unit for CubicMeter {
    const ABBREVIATION: &'static str = "m³";
    const LATEX_ABBREVIATION: &'static str = "m^3";
}
impl L3Unit for CubicMeter {}

impl CubicMeter {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> L3<V, CubicMeter> {
        L3 {
            unit: PhantomData,
            value
        }
    }
}