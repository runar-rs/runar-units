//! Frequency
use crate::types::l1::{L1, L1Unit};
use crate::types::l1_tn1::L1_TN1;
use crate::types::t1::{T1, T1Unit};
use crate::{Dimension, QuantityNum, Unit};
use core::fmt::Display;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct TN1<V, U>
where
    V: QuantityNum,
    U: TN1Unit,
{
    pub(crate) unit: PhantomData<U>,
    pub(crate) value: V,
}

impl<V: QuantityNum + Display, U: TN1Unit> core::fmt::Display for TN1<V, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREVIATION)
    }
}

impl<V: QuantityNum, U: TN1Unit> Dimension for TN1<V, U> {
    type ValueType = V;

    /// Returns a reference to the underlying value of the dimension.
    fn get(&self) -> &Self::ValueType {
        &self.value
    }
}

impl<U: TN1Unit> TN1<f64, U> {
    #[must_use]
    pub fn new(value: f64) -> Self {
        TN1 {
            unit: PhantomData,
            value,
        }
    }
}

impl_dimension_ops!(TN1, TN1Unit);
impl_dimension_mul!(TN1, TN1Unit, T1,  T1Unit => scalar);
impl_dimension_mul!(TN1, TN1Unit, L1,  L1Unit => L1_TN1);

pub trait TN1Unit: Unit {}

pub struct Quettahertz;
impl Unit for Quettahertz {
    const ABBREVIATION: &'static str = "QHz";
}
impl TN1Unit for Quettahertz {}

pub struct Ronnahertz;
impl Unit for Ronnahertz {
    const ABBREVIATION: &'static str = "RHz";
}
impl TN1Unit for Ronnahertz {}

pub struct Yottahertz;
impl Unit for Yottahertz {
    const ABBREVIATION: &'static str = "YHz";
}
impl TN1Unit for Yottahertz {}

pub struct Zettahertz;
impl Unit for Zettahertz {
    const ABBREVIATION: &'static str = "ZHz";
}
impl TN1Unit for Zettahertz {}

pub struct Exahertz;
impl Unit for Exahertz {
    const ABBREVIATION: &'static str = "EHz";
}
impl TN1Unit for Exahertz {}

pub struct Petahertz;
impl Unit for Petahertz {
    const ABBREVIATION: &'static str = "PHz";
}
impl TN1Unit for Petahertz {}

pub struct Terahertz;
impl Unit for Terahertz {
    const ABBREVIATION: &'static str = "THz";
}
impl TN1Unit for Terahertz {}

pub struct Gigahertz;
impl Unit for Gigahertz {
    const ABBREVIATION: &'static str = "GHz";
}
impl TN1Unit for Gigahertz {}

pub struct Megahertz;
impl Unit for Megahertz {
    const ABBREVIATION: &'static str = "MHz";
}
impl TN1Unit for Megahertz {}

pub struct Kilohertz;
impl Unit for Kilohertz {
    const ABBREVIATION: &'static str = "kHz";
}
impl TN1Unit for Kilohertz {}

#[derive(Debug, Clone, Copy)]
pub struct Hertz;
impl Unit for Hertz {
    const ABBREVIATION: &'static str = "Hz";
}
impl TN1Unit for Hertz {}

impl Hertz {
    pub fn new<V: QuantityNum>(value: V) -> TN1<V, Hertz> {
        TN1 {
            unit: PhantomData,
            value,
        }
    }
}

pub struct Millihertz;
impl Unit for Millihertz {
    const ABBREVIATION: &'static str = "mHz";
}
impl TN1Unit for Millihertz {}

pub struct Microhertz;
impl Unit for Microhertz {
    const ABBREVIATION: &'static str = "μHz";
}
impl TN1Unit for Microhertz {}

pub struct Nanohertz;
impl Unit for Nanohertz {
    const ABBREVIATION: &'static str = "nHz";
}
impl TN1Unit for Nanohertz {}

pub struct Picohertz;
impl Unit for Picohertz {
    const ABBREVIATION: &'static str = "pHz";
}
impl TN1Unit for Picohertz {}

pub struct Femtohertz;
impl Unit for Femtohertz {
    const ABBREVIATION: &'static str = "fHz";
}
impl TN1Unit for Femtohertz {}

pub struct Attohertz;
impl Unit for Attohertz {
    const ABBREVIATION: &'static str = "aHz";
}
impl TN1Unit for Attohertz {}

pub struct Zeptohertz;
impl Unit for Zeptohertz {
    const ABBREVIATION: &'static str = "zHz";
}
impl TN1Unit for Zeptohertz {}

pub struct Yoctohertz;
impl Unit for Yoctohertz {
    const ABBREVIATION: &'static str = "yHz";
}
impl TN1Unit for Yoctohertz {}

pub struct Rontohertz;
impl Unit for Rontohertz {
    const ABBREVIATION: &'static str = "rHz";
}
impl TN1Unit for Rontohertz {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::t1::Second;

    #[test]
    fn test_mul_tn1_t1() {
        let frequency = TN1::<f64, Hertz>::new(2.0);
        let time = T1::<f64, Second>::new(3.0);
        let result = frequency * time;
        assert_eq!(result, 6.0);
    }
}
