use core::fmt::Display;
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use core::marker::PhantomData;

use crate::types::l1::{L1, Meter};
use crate::types::m1::{Kilogram, M1};
use crate::{Dimension, QuantityNum, Unit};


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct L1_M1<V: QuantityNum, U: L1_M1Unit> {
    pub (crate) unit: PhantomData<U>,
    pub (crate) value: V
}

impl<V: QuantityNum + Display, U: L1_M1Unit> Display for L1_M1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: L1_M1Unit> Dimension for L1_M1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

// Ops

/// Add same `ValueType` and same `Unit`
impl<V: QuantityNum + Add<Output = V>, U: L1_M1Unit> Add<L1_M1<V, U>> for L1_M1<V, U> {
    type Output = L1_M1<V, U>;

    fn add(self, rhs: L1_M1<V, U>) -> Self::Output {
        L1_M1 {
            unit: PhantomData,
            value: self.value + rhs.value
        }
    }
}

/// AddAssign with same `ValueType` and same `Unit`
impl<V: QuantityNum + AddAssign, U: L1_M1Unit> AddAssign<L1_M1<V, U>> for L1_M1<V, U> {
    fn add_assign(&mut self, rhs: L1_M1<V, U>) {
        self.value += rhs.value;
    }
}

/// Sub with same `ValueType` and same `Unit`
impl<V: QuantityNum + Sub<Output = V>, U: L1_M1Unit> Sub<L1_M1<V, U>> for L1_M1<V, U> {
    type Output = L1_M1<V, U>;

    fn sub(self, rhs: L1_M1<V, U>) -> Self::Output {
        L1_M1 {
            unit: PhantomData,
            value: self.value - rhs.value
        }
    }
}

/// SubAssign with same `ValueType` and same `Unit`
impl<V: QuantityNum + SubAssign, U: L1_M1Unit> SubAssign<L1_M1<V, U>> for L1_M1<V,U> {
    fn sub_assign(&mut self, rhs: L1_M1<V, U>) {
        self.value -= rhs.value;
    }
}

/// Mul with same `ValueType` and scalar
impl<V: QuantityNum + Mul<Output = V>, U: L1_M1Unit> Mul<V> for L1_M1<V, U> {
    type Output = L1_M1<V, U>;

    fn mul(self, rhs: V) -> Self::Output {
        L1_M1 {
            unit: PhantomData,
            value: self.value * rhs
        }
    }
}

/// MulAssign with same `ValueType` and scalar
impl<V: QuantityNum + MulAssign, U: L1_M1Unit> MulAssign<V> for L1_M1<V, U> {
    fn mul_assign(&mut self, rhs: V) {
        self.value *= rhs;
    }
}

/// Div with same `ValueType` and scalar
impl<V: QuantityNum + Div<Output = V>, U: L1_M1Unit> Div<V> for L1_M1<V, U> {
    type Output = L1_M1<V, U>;

    fn div(self, rhs: V) -> Self::Output {
        L1_M1 {
            unit: PhantomData,
            value: self.value / rhs
        }
    }
}

/// DivAssign with same `ValueType` and scalar
impl<V: QuantityNum + DivAssign, U: L1_M1Unit> DivAssign<V> for L1_M1<V, U> {
    fn div_assign(&mut self, rhs: V) {
        self.value /= rhs;
    }
}

/// Div with same `ValueType` and MeterKilogram and Meter
impl<V: QuantityNum + Div<Output = V>> Div<L1<V, Meter>> for L1_M1<V, MeterKilogram> {
    type Output = M1<V, Kilogram>;

    fn div(self, rhs: L1<V, Meter>) -> Self::Output {
        M1 {
            unit: PhantomData,
            value: self.value / rhs.value
        }
    }
}

/// Div with same `ValueType` and MeterKilogram and Kilogram
impl<V: QuantityNum + Div<Output = V>> Div<M1<V, Kilogram>> for L1_M1<V, MeterKilogram> {
    type Output = L1<V, Meter>;

    fn div(self, rhs: M1<V, Kilogram>) -> Self::Output {
        L1 {
            unit: PhantomData,
            value: self.value / rhs.value
        }
    }
}

#[allow(non_camel_case_types)]
pub trait L1_M1Unit: Unit {}

#[derive(Debug, Clone, Copy)]
pub struct MeterKilogram;
impl Unit for MeterKilogram {
    const ABBREVIATION: &'static str = "m·kg";
}
impl L1_M1Unit for MeterKilogram {}

impl MeterKilogram {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1<V, MeterKilogram> {
        L1_M1 {
            unit: PhantomData,
            value
        }
    }
}