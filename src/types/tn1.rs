/// Dimension for TN1
/// 
use core::fmt::Display;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use crate::{Dimension, QuantityNum, Unit};

pub struct TN1<V, U>
where V: QuantityNum, U: Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum, U: Unit> Dimension for TN1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}

impl<V: QuantityNum + Add<Output = V>, U: Unit> Add<TN1<V, U>> for TN1<V, U> {
    type Output = Self;

    fn add(self, rhs: TN1<V, U>) -> Self::Output {
        TN1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

impl<V: QuantityNum, U: Unit> AddAssign<TN1<V, U>> for TN1<V, U> {
    fn add_assign(&mut self, rhs: TN1<V, U>) {
        self.value += rhs.value;
    }
}

impl<V: QuantityNum + Sub<Output = V>, U: Unit> Sub<TN1<V, U>> for TN1<V, U> {
    type Output = Self;

    fn sub(self, rhs: TN1<V, U>) -> Self::Output {
        TN1 {
            unit: PhantomData,
            value: self.value - rhs.value,
        }
    }
}

impl<V: QuantityNum, U: Unit> SubAssign<TN1<V, U>> for TN1<V, U> {
    fn sub_assign(&mut self, rhs: TN1<V, U>) {
        self.value -= rhs.value;
    }
}

pub trait TN1Unit: Unit {}


pub struct Gigahertz;
impl Unit for Gigahertz {}
impl TN1Unit for Gigahertz {}

/// Display for TN1 in Gigahertz
impl<V: QuantityNum + Display> Display for TN1<V, Gigahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} GHz", self.value)
    }
}

pub struct Megahertz;
impl Unit for Megahertz {}
impl TN1Unit for Megahertz {}

/// Display for TN1 in Megahertz
impl<V: QuantityNum + Display> Display for TN1<V, Megahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} MHz", self.value)
    }
}

pub struct Kilohertz;
impl Unit for Kilohertz {}
impl TN1Unit for Kilohertz {}

/// Display for TN1 in Kilohertz
impl<V: QuantityNum + Display> Display for TN1<V, Kilohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} kHz", self.value)
    }
}

pub struct Hertz;
impl Unit for Hertz {}
impl TN1Unit for Hertz {}

/// Display for TN1 in Hertz
impl<V: QuantityNum + Display> Display for TN1<V, Hertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} Hz", self.value)
    }
}

pub struct Millihertz;
impl Unit for Millihertz {}
impl TN1Unit for Millihertz {}

/// Display for TN1 in Millihertz
impl<V: QuantityNum + Display> Display for TN1<V, Millihertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} mHz", self.value)
    }
}

pub struct Microhertz;
impl Unit for Microhertz {}
impl TN1Unit for Microhertz {}

/// Display for TN1 in Microhertz
impl<V: QuantityNum + Display> core::fmt::Display for TN1<V, Microhertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} μHz", self.value)
    }
}