use core::ops::{Add, AddAssign, Sub, SubAssign, Mul};
use core::marker::PhantomData;

use crate::types::l1_t1::{L1_T1, MeterSecond};
use crate::types::t1::{Second, T1};
use crate::{Dimension, QuantityNum, Unit};

pub type Length<V, U> = L1<V, U>;
pub struct L1<V, U>
where V: QuantityNum, U: Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum, U: Unit> Dimension for L1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}

/// Add with same ValueType and same Unit
impl<V: QuantityNum + Add<Output = V>, U: Unit> Add<L1<V, U>> for L1<V, U> {
    type Output = Self;

    fn add(self, rhs: L1<V, U>) -> Self::Output {
        L1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

/// AddAssign with same ValueType and same Unit
impl<V: QuantityNum, U: Unit> AddAssign<L1<V, U>> for L1<V, U> {
    fn add_assign(&mut self, rhs: L1<V, U>) {
        self.value += rhs.value;
    }
}

/// Sub with same ValueType and same Unit
impl<V: QuantityNum + Sub<Output = V>, U: Unit> Sub<L1<V,U>> for L1<V, U> {
    type Output = Self;

    fn sub(self, rhs: L1<V,U>) -> Self::Output {
        L1 {
            unit: PhantomData,
            value: self.value - rhs.value
        }
    }
}

/// SubAssign with same ValueType and same Unit
impl<V: QuantityNum, U: Unit> SubAssign<L1<V, U>> for L1<V,U> {
    fn sub_assign(&mut self, rhs: L1<V, U>) {
        self.value -= rhs.value;
    }
}

/// Mul with same ValueType and Second and Meter
impl<V: QuantityNum + Mul<Output = V>> Mul<T1<V, Second>> for L1<V, Meter> {
    type Output = L1_T1<V, MeterSecond>;

    fn mul(self, rhs: T1<V, Second>) -> Self::Output {
        L1_T1 {
            unit: PhantomData,
            value: self.value * rhs.value,
        }
    }
}

pub struct Kilometer;
impl Unit for Kilometer {}

pub struct Meter;
impl Unit for Meter {}

pub struct Millimeter;
impl Unit for Millimeter {}

pub struct Micrometer;
impl Unit for Micrometer {}

pub struct Nanometer;
impl Unit for Nanometer {}