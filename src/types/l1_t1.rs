use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{Dimension, QuantityNum, Unit};


#[allow(non_camel_case_types)]
pub struct L1_T1<V: QuantityNum, U: Unit> {
    pub unit: PhantomData<U>,
    pub value: V,
}

impl<V: QuantityNum, U: Unit> Dimension for L1_T1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}

/// Add for same ValueType and same Unit
impl<V: QuantityNum + Add<Output = V>, U: Unit> Add<L1_T1<V, U>> for L1_T1<V, U> {
    type Output = L1_T1<V, U>;

    fn add(self, rhs: L1_T1<V, U>) -> Self::Output {
        L1_T1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

/// AddAssign for same ValueType and same Unit
impl<V: QuantityNum, U: Unit> AddAssign<L1_T1<V, U>> for L1_T1<V, U> {
    fn add_assign(&mut self, rhs: L1_T1<V, U>) {
        self.value += rhs.value;
    }
}

/// Sub for same ValueType and same Unit
impl<V: QuantityNum + Sub<Output = V>, U: Unit> Sub<L1_T1<V, U>> for L1_T1<V, U> {
    type Output = L1_T1<V, U>;

    fn sub(self, rhs: L1_T1<V, U>) -> Self::Output {
        L1_T1 {
            unit: PhantomData,
            value: self.value - rhs.value,
        }
    }
}

/// SubAssign for same ValueType and same Unit
impl<V: QuantityNum, U: Unit> SubAssign<L1_T1<V, U>> for L1_T1<V, U> {
    fn sub_assign(&mut self, rhs: L1_T1<V, U>) {
        self.value -= rhs.value
    }
}

pub struct MeterSecond;
impl Unit for MeterSecond {}