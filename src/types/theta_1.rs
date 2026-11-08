//! Thermodynamic Temperature

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type ThermodynamicTemperature<V, U> = Theta1<V, U>;
#[derive(Debug, Clone, Copy)]
pub struct Theta1<V, U>
where
    V: QuantityNum,
    U: Theta1Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: Theta1Unit> core::fmt::Display for Theta1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: Theta1Unit> Dimension for Theta1<V, U> {
    type ValueType = V;

    /// Returns a reference to the underlying value of the dimension.
    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(Theta1, Theta1Unit);

pub trait Theta1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Kelvin;
impl Unit for Kelvin {
    const ABBREVIATION: &'static str = "K";
}
impl Theta1Unit for Kelvin {}

impl Kelvin {
    #[must_use]
    pub fn new<V: QuantityNum>(value: V) -> Theta1<V, Kelvin> {
        Theta1 {
            unit: PhantomData,
            value,
        }
    }
}
