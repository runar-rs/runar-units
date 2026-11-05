use core::fmt::Display;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};


use crate::{Dimension, QuantityNum, Unit};



pub type Impulse<V, U> = L1_M1_TN1<V, U>;

#[allow(non_camel_case_types)]
pub struct L1_M1_TN1<V, U>
where V: QuantityNum, U: L1_M1_TN1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum + Display, U: L1_M1_TN1Unit> Display for L1_M1_TN1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, "{}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: L1_M1_TN1Unit> Dimension for L1_M1_TN1<V, U> {
    type ValueType = V;

    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

/// `Add` with same `ValueType` and same `Unit`
impl<V: QuantityNum + Add<Output = V>, U: L1_M1_TN1Unit> Add<L1_M1_TN1<V, U>> for L1_M1_TN1<V, U> {
    type Output = L1_M1_TN1<V, U>;

    fn add(self, rhs: L1_M1_TN1<V, U>) -> Self::Output {
        L1_M1_TN1 {
            unit: PhantomData,
            value: self.value + rhs.value
        }
    }
}

/// `AddAssign` with same `ValueType` and same `Unit`
impl<V: QuantityNum + AddAssign, U: L1_M1_TN1Unit> AddAssign<L1_M1_TN1<V, U>> for L1_M1_TN1<V, U> {
    fn add_assign(&mut self, rhs: L1_M1_TN1<V, U>) {
        self.value += rhs.value;
    }
}

/// `Sub` with same `ValueType` and same `Unit`
impl<V: QuantityNum + Sub<Output = V>, U: L1_M1_TN1Unit> Sub<L1_M1_TN1<V, U>> for L1_M1_TN1<V, U> {
    type Output = L1_M1_TN1<V, U>;

    fn sub(self, rhs: L1_M1_TN1<V, U>) -> Self::Output {
        L1_M1_TN1 {
            unit: PhantomData,
            value: self.value - rhs.value
        }
    }
}

/// `SubAssign` with same `ValueType` and same `Unit`
impl<V: QuantityNum + SubAssign, U: L1_M1_TN1Unit> SubAssign<L1_M1_TN1<V, U>> for L1_M1_TN1<V,U> {
    fn sub_assign(&mut self, rhs: L1_M1_TN1<V, U>) {
        self.value -= rhs.value;
    }
}

/// `Mul` with scalar of same `ValueType`
impl<V: QuantityNum + Mul<Output = V>, U: L1_M1_TN1Unit> Mul<V> for L1_M1_TN1<V, U> {
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        L1_M1_TN1 {
            unit: PhantomData,
            value: self.value * rhs
        }
    }
}

/// `MulAssign` with scalar of same `ValueType`
impl<V: QuantityNum + MulAssign, U: L1_M1_TN1Unit> MulAssign<V> for L1_M1_TN1<V, U> {
    fn mul_assign(&mut self, rhs: V) {
        self.value *= rhs;
    }
}

/// `Div` with scalar of same `ValueType`
impl<V: QuantityNum + Div<Output = V>, U: L1_M1_TN1Unit> Div<V> for L1_M1_TN1<V, U> {
    type Output = Self;

    fn div(self, rhs: V) -> Self::Output {
        L1_M1_TN1 {
            unit: PhantomData,
            value: self.value / rhs
        }
    }
}

/// `DivAssign` with scalar of same `ValueType`
impl<V: QuantityNum + DivAssign, U: L1_M1_TN1Unit> DivAssign<V> for L1_M1_TN1<V, U> {
    fn div_assign(&mut self, rhs: V) {
        self.value /= rhs;
    }
}


/// Marker trait for units of `L1_M1_TN1`
#[allow(non_camel_case_types)]
pub trait L1_M1_TN1Unit: Unit {}


#[derive(Debug, Clone, Copy)]
pub struct NewtonSecond;
impl Unit for NewtonSecond {
    const ABBREVIATION: &'static str = "N·s";
}
impl L1_M1_TN1Unit for NewtonSecond {}

impl NewtonSecond {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1_TN1<V, NewtonSecond> {
        L1_M1_TN1 {
            unit: PhantomData,
            value
        }
    }
}

pub struct KilogramMeterPerSecond;
impl Unit for KilogramMeterPerSecond {
    const ABBREVIATION: &'static str = "kg·m/s";
}
impl L1_M1_TN1Unit for KilogramMeterPerSecond {}

impl KilogramMeterPerSecond {
    pub fn new<V: QuantityNum>(value: V) -> L1_M1_TN1<V, KilogramMeterPerSecond> {
        L1_M1_TN1 {
            unit: PhantomData,
            value
        }
    }
}

/// Transform NetwonSecond to KilogramMeterPerSecond.
/// 
/// As those two units are equivalent, this is a simple conversion that does not change the value.
/// Because of that, this conversion is implemented with genetics.
impl<V: QuantityNum> From<L1_M1_TN1<V, NewtonSecond>> for L1_M1_TN1<V, KilogramMeterPerSecond> {
    fn from(value: L1_M1_TN1<V, NewtonSecond>) -> Self {
        L1_M1_TN1 {
            unit: PhantomData,
            value: value.value
        }
    }
}

impl<V: QuantityNum> From<L1_M1_TN1<V, KilogramMeterPerSecond>> for L1_M1_TN1<V, NewtonSecond> {
    fn from(value: L1_M1_TN1<V, KilogramMeterPerSecond>) -> Self {
        L1_M1_TN1 {
            unit: PhantomData,
            value: value.value
        }
    }
}