//! Solid Angle
//! 
use core::fmt::Display;
use core::marker::PhantomData;

use crate::Dimension;
use crate::QuantityNum;
use crate::Unit;

use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub struct Sr1<V, U>
where
    V: QuantityNum,
    U: Sr1Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: Sr1Unit> Display for Sr1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: Sr1Unit> Dimension for Sr1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl_dimension_ops!(Sr1, Sr1Unit);

pub trait Sr1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct Steradian;
impl Unit for Steradian {
    const ABBREVIATION: &'static str = "sr";
}
impl Sr1Unit for Steradian {}

impl Steradian {
    pub fn new<V: QuantityNum>(value: V) -> Sr1<V, Steradian> {
        Sr1 {
            unit: PhantomData,
            value,
        }
    }
}