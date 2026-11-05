use core::{marker::PhantomData, ops::{Add, AddAssign, Sub, SubAssign}};

use crate::{Dimension, QuantityNum, Unit};

pub type Velocity<V, U> = L1_TN1<V, U>;

#[allow(non_camel_case_types)]
pub struct L1_TN1<V: QuantityNum, U: Unit> {
    pub (crate) unit: PhantomData<U>,
    pub (crate) value: V
}

impl<V: QuantityNum, U: Unit> Dimension for L1_TN1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

/// Add same `ValueType` and same `Unit`
impl<V: QuantityNum + Add<Output = V>, U: Unit> Add<L1_TN1<V, U>> for L1_TN1<V, U> {
    type Output = L1_TN1<V, U>;

    fn add(self, rhs: L1_TN1<V, U>) -> Self::Output {
        L1_TN1 {
            unit: PhantomData,
            value: self.value + rhs.value
        }
    }
}

/// `AddAssign` with same `ValueType` and same `Unit`
impl<V: QuantityNum + AddAssign, U: Unit> AddAssign<L1_TN1<V, U>> for L1_TN1<V, U> {
    fn add_assign(&mut self, rhs: L1_TN1<V, U>) {
        self.value += rhs.value;
    }
}


/// Sub with same `ValueType` and same `Unit`
impl<V: QuantityNum + Sub<Output = V>, U: Unit> Sub<L1_TN1<V, U>> for L1_TN1<V, U> {
    type Output = L1_TN1<V, U>;

    fn sub(self, rhs: L1_TN1<V, U>) -> Self::Output {
        L1_TN1 {
            unit: PhantomData,
            value: self.value - rhs.value
        }
    }
}

/// SubAssign with same `ValueType` and same `Unit`
impl<V: QuantityNum + SubAssign, U: Unit> SubAssign<L1_TN1<V, U>> for L1_TN1<V,U> {
    fn sub_assign(&mut self, rhs: L1_TN1<V, U>) {
        self.value -= rhs.value;
    }
}

pub struct MeterPerSecond;
impl Unit for MeterPerSecond {
    const ABBREVIATION: &'static str = "m/s";
    const LATEX_ABBREVIATION: &'static str = "\\frac{\\text{m}}{\\text{s}}";
}
    
impl MeterPerSecond {
    pub fn new<V: QuantityNum>(value: V) -> L1_TN1<V, MeterPerSecond> {
        L1_TN1 {
            unit: PhantomData,
            value
        }
    }
}

#[cfg(test)]
mod tests {
}