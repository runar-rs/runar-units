//! Contains the dimesion for the thermodynamic temperature (Theta1) and its units.
//! 

use core::fmt::Display;
use core::marker::PhantomData;

use crate::{Dimension, QuantityNum, Unit};

pub type ThermodynamicTemperature<V, U> = Theta1<V, U>;
#[derive(Debug, Clone, Copy)]
pub struct Theta1<V, U>
where V: QuantityNum, U: Theta1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
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

pub trait Theta1Unit: Unit {}