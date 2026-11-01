/// Dimension for TN1
/// 
use core::fmt::Display;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::types::t1::{Second, T1};
use crate::{Dimension, QuantityNum, Unit};

pub struct TN1<V, U>
where V: QuantityNum, U: TN1Unit {
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V
}

impl<V: QuantityNum, U: TN1Unit> Dimension for TN1<V, U> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}

impl<V: QuantityNum + Add<Output = V>, U: TN1Unit> Add<TN1<V, U>> for TN1<V, U> {
    type Output = Self;

    fn add(self, rhs: TN1<V, U>) -> Self::Output {
        TN1 {
            unit: PhantomData,
            value: self.value + rhs.value,
        }
    }
}

impl<V: QuantityNum, U: TN1Unit> AddAssign<TN1<V, U>> for TN1<V, U> {
    fn add_assign(&mut self, rhs: TN1<V, U>) {
        self.value += rhs.value;
    }
}

impl<V: QuantityNum + Sub<Output = V>, U: TN1Unit> Sub<TN1<V, U>> for TN1<V, U> {
    type Output = Self;

    fn sub(self, rhs: TN1<V, U>) -> Self::Output {
        TN1 {
            unit: PhantomData,
            value: self.value - rhs.value,
        }
    }
}

impl<V: QuantityNum, U: TN1Unit> SubAssign<TN1<V, U>> for TN1<V, U> {
    fn sub_assign(&mut self, rhs: TN1<V, U>) {
        self.value -= rhs.value;
    }
}

/// Mul with scalar of same ValueType
impl<V: QuantityNum + Mul<Output = V>, U: TN1Unit> Mul<V> for TN1<V, U> {
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        TN1 {
            unit: PhantomData,
            value: self.value * rhs,
        }
    }
}

/// MulAssign with scalar of same ValueType
impl<V: QuantityNum + Mul<Output = V>, U: TN1Unit> MulAssign<V> for TN1<V, U> {
    fn mul_assign(&mut self, rhs: V) {
        self.value *= rhs;
    }
}

/// Div with scalar of same ValueType
impl<V: QuantityNum + Div<Output = V>, U: TN1Unit> Div<V> for TN1<V, U> {
    type Output = Self;

    fn div(self, rhs: V) -> Self::Output {
        TN1 {
            unit: PhantomData,
            value: self.value / rhs,
        }
    }
}

/// DivAssign with scalar of same ValueType
impl<V: QuantityNum + Div<Output = V>, U: TN1Unit> DivAssign<V> for TN1<V, U> {
    fn div_assign(&mut self, rhs: V) {
        self.value /= rhs;
    }
}

/// Div with same ValueType and same Unit
impl<V: QuantityNum + Div<Output = V>, U: TN1Unit> Div<TN1<V, U>> for TN1<V, U> {
    type Output = V;

    fn div(self, rhs: TN1<V, U>) -> Self::Output {
        self.value / rhs.value
    }
}

/// Mul with same ValueType and Hertz and Second
impl<V: QuantityNum + Mul<Output = V>> Mul<T1<V, Second>> for TN1<V, Hertz> {
    type Output = V;

    fn mul(self, rhs: T1<V, Second>) -> Self::Output {
        self.value * rhs.value
    }
}


pub trait TN1Unit: Unit {}

pub struct Quettahertz;
impl Unit for Quettahertz {}
impl TN1Unit for Quettahertz {}

/// Display for TN1 in Quettahertz
impl<V: QuantityNum + Display> Display for TN1<V, Quettahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} QHz", self.value)
    }
}

pub struct Ronnahertz;
impl Unit for Ronnahertz {}
impl TN1Unit for Ronnahertz {}

/// Display for TN1 in Ronnahertz
impl<V: QuantityNum + Display> Display for TN1<V, Ronnahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} RHz", self.value)
    }
}

pub struct Yottahertz;
impl Unit for Yottahertz {}
impl TN1Unit for Yottahertz {}

impl<V: QuantityNum + Display> Display for TN1<V, Yottahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} YHz", self.value)
    }
}

pub struct Zettahertz;
impl Unit for Zettahertz {}
impl TN1Unit for Zettahertz {}

impl<V: QuantityNum + Display> Display for TN1<V, Zettahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ZHz", self.value)
    }
}


pub struct Exahertz;
impl Unit for Exahertz {}
impl TN1Unit for Exahertz {}

/// Display for TN1 in Exahertz
impl<V: QuantityNum + Display> Display for TN1<V, Exahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} EHz", self.value)
    }
}


pub struct Petahertz;
impl Unit for Petahertz {}
impl TN1Unit for Petahertz {}

/// Display for TN1 in Petahertz
impl<V: QuantityNum + Display> Display for TN1<V, Petahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} PHz", self.value)
    }
}


pub struct Terahertz;
impl Unit for Terahertz {}
impl TN1Unit for Terahertz {}

/// Display for TN1 in Terahertz
impl<V: QuantityNum + Display> Display for TN1<V, Terahertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} THz", self.value)
    }
}

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
impl<V: QuantityNum + Display> Display for TN1<V, Microhertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} μHz", self.value)
    }
}

pub struct Nanohertz;
impl Unit for Nanohertz {}
impl TN1Unit for Nanohertz {}

/// Display for TN1 in Nanohertz
impl<V: QuantityNum + Display> Display for TN1<V, Nanohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} nHz", self.value)
    }
}

pub struct Picohertz;
impl Unit for Picohertz {}
impl TN1Unit for Picohertz {}

/// Display for TN1 in Picohertz
impl<V: QuantityNum + Display> Display for TN1<V, Picohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} pHz", self.value)
    }
}

pub struct Femtohertz;
impl Unit for Femtohertz {}
impl TN1Unit for Femtohertz {}

/// Display for TN1 in Femtohertz
impl<V: QuantityNum + Display> Display for TN1<V, Femtohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} fHz", self.value)
    }
}

pub struct Attohertz;
impl Unit for Attohertz {}
impl TN1Unit for Attohertz {}

/// Display for TN1 in Attohertz
impl<V: QuantityNum + Display> Display for TN1<V, Attohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} aHz", self.value)
    }
}

pub struct Zeptohertz;
impl Unit for Zeptohertz {}
impl TN1Unit for Zeptohertz {}

/// Display for TN1 in Zeptohertz
impl<V: QuantityNum + Display> Display for TN1<V, Zeptohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} zHz", self.value)
    }
}

pub struct Yoctohertz;
impl Unit for Yoctohertz {}
impl TN1Unit for Yoctohertz {}

/// Display for TN1 in Yoctohertz
impl<V: QuantityNum + Display> Display for TN1<V, Yoctohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} yHz", self.value)
    }
}

pub struct Rontohertz;
impl Unit for Rontohertz {}
impl TN1Unit for Rontohertz {}

/// Display for TN1 in Rontohertz
impl<V: QuantityNum + Display> Display for TN1<V, Rontohertz> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} rHz", self.value)
    }
}