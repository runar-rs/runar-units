#![cfg_attr(not(test), no_std)]

pub trait QuantityNum: Sized {}

pub trait LinearScale: QuantityNum {
    fn from_f64(value: f64) -> Self;
}

impl QuantityNum for f32 {}

impl LinearScale for f32 {
    fn from_f64(value: f64) -> Self {
        value as f32
    }
}

impl QuantityNum for f64 {}

impl LinearScale for f64 {
    fn from_f64(value: f64) -> Self {
        value
    }
}

pub trait Dimension {
    type ValueType: QuantityNum;

    /// Returns a reference to the underlying value of the dimension.
    #[must_use]
    fn get(&self) -> &Self::ValueType;
}

/// Marker Traits for Units
pub trait Unit {
    const ABBREVIATION: &'static str;
    const SCALE_TO_BASE: f64 = 1.0;
    /// Optional LaTeX abbreviation for the unit. If not provided, the `ABBREVIATION` will be used.
    const LATEX_ABBREVIATION: &'static str = Self::ABBREVIATION;
}

macro_rules! dimension_base_unit {
    (L1) => { crate::types::l1::Meter };
    (M1) => { crate::types::m1::Kilogram };
    (L1_T1) => { crate::types::l1_t1::MeterSecond };
    (L1_TN1) => { crate::types::l1_tn1::MeterPerSecond };
    (L1_M1) => { crate::types::l1_m1::MeterKilogram };
}

macro_rules! impl_dimension_mul_with_unit {
    ($lhs:ident, $rhs:ident => $output:ident, $output_unit:ty, $lhs_bound:path, $rhs_bound:path) => {
        impl<V, UL, UR> core::ops::Mul<$rhs<V, UR>> for $lhs<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
            UR: $rhs_bound,
        {
            type Output = $output<V, $output_unit>;

            fn mul(self, rhs: $rhs<V, UR>) -> Self::Output {
                $output {
                    unit: core::marker::PhantomData,
                    value: (self.value * rhs.value)
                        * V::from_f64(
                            UL::SCALE_TO_BASE
                                * UR::SCALE_TO_BASE
                                / <$output_unit as crate::Unit>::SCALE_TO_BASE,
                        ),
                }
            }
        }
    };
}

macro_rules! impl_dimension_div_with_unit {
    ($lhs:ident, $rhs:ident => $output:ident, $output_unit:ty, $lhs_bound:path, $rhs_bound:path) => {
        impl<V, UL, UR> core::ops::Div<$rhs<V, UR>> for $lhs<V, UL>
        where
            V: crate::LinearScale
                + core::ops::Div<Output = V>
                + core::ops::Mul<Output = V>,
            UL: $lhs_bound,
            UR: $rhs_bound,
        {
            type Output = $output<V, $output_unit>;

            fn div(self, rhs: $rhs<V, UR>) -> Self::Output {
                $output {
                    unit: core::marker::PhantomData,
                    value: (self.value / rhs.value)
                        * V::from_f64(
                            UL::SCALE_TO_BASE
                                / UR::SCALE_TO_BASE
                                / <$output_unit as crate::Unit>::SCALE_TO_BASE,
                        ),
                }
            }
        }
    };
}

macro_rules! impl_dimension_mul {
    ($lhs:ident, $rhs:ident => $output:ident, $lhs_bound:path, $rhs_bound:path) => {
        impl_dimension_mul_with_unit!(
            $lhs,
            $rhs => $output,
            dimension_base_unit!($output),
            $lhs_bound,
            $rhs_bound
        );
    };
}

macro_rules! impl_dimension_div {
    ($lhs:ident, $rhs:ident => $output:ident, $lhs_bound:path, $rhs_bound:path) => {
        impl_dimension_div_with_unit!(
            $lhs,
            $rhs => $output,
            dimension_base_unit!($output),
            $lhs_bound,
            $rhs_bound
        );
    };
}

macro_rules! impl_dimension_ops {
    ($quantity:ident, $unit_bound:path) => {
        impl<V: crate::QuantityNum + core::ops::Add<Output = V>, U: $unit_bound>
            core::ops::Add for $quantity<V, U>
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value + rhs.value,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::AddAssign, U: $unit_bound>
            core::ops::AddAssign for $quantity<V, U>
        {
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        impl<V: crate::QuantityNum + core::ops::Sub<Output = V>, U: $unit_bound>
            core::ops::Sub for $quantity<V, U>
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value - rhs.value,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::SubAssign, U: $unit_bound>
            core::ops::SubAssign for $quantity<V, U>
        {
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        impl<V: crate::QuantityNum + core::ops::Mul<Output = V>, U: $unit_bound>
            core::ops::Mul<V> for $quantity<V, U>
        {
            type Output = Self;

            fn mul(self, rhs: V) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value * rhs,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::MulAssign, U: $unit_bound>
            core::ops::MulAssign<V> for $quantity<V, U>
        {
            fn mul_assign(&mut self, rhs: V) {
                self.value *= rhs;
            }
        }

        impl<V: crate::QuantityNum + core::ops::Div<Output = V>, U: $unit_bound>
            core::ops::Div<V> for $quantity<V, U>
        {
            type Output = Self;

            fn div(self, rhs: V) -> Self::Output {
                Self {
                    unit: core::marker::PhantomData,
                    value: self.value / rhs,
                }
            }
        }

        impl<V: crate::QuantityNum + core::ops::DivAssign, U: $unit_bound>
            core::ops::DivAssign<V> for $quantity<V, U>
        {
            fn div_assign(&mut self, rhs: V) {
                self.value /= rhs;
            }
        }
    };
}

macro_rules! impl_linear_unit_conversion_pair {
    (
        $quantity:ident,
        $from:ty => $to:ty,
        $factor:expr
    ) => {
        impl<V> From<$quantity<V, $from>> for $quantity<V, $to>
        where
            V: crate::LinearScale + core::ops::Mul<Output = V>,
        {
            fn from(value: $quantity<V, $from>) -> Self {
                $quantity {
                    unit: core::marker::PhantomData,
                    value: value.value * V::from_f64($factor),
                }
            }
        }
    };
}

macro_rules! impl_linear_unit_conversions {
    ($quantity:ident; $head:ty $(, $tail:ty)+ $(,)?) => {
        $(
            impl_linear_unit_conversion_pair!(
                $quantity,
                $head => $tail,
                <$head as crate::Unit>::SCALE_TO_BASE
                    / <$tail as crate::Unit>::SCALE_TO_BASE
            );
            impl_linear_unit_conversion_pair!(
                $quantity,
                $tail => $head,
                <$tail as crate::Unit>::SCALE_TO_BASE
                    / <$head as crate::Unit>::SCALE_TO_BASE
            );
        )*

        impl_linear_unit_conversions!(@rest $quantity; $($tail),*);
    };
    (@rest $quantity:ident; $head:ty $(, $tail:ty)+ $(,)?) => {
        impl_linear_unit_conversions!($quantity; $head, $($tail),*);
    };
    (@rest $quantity:ident; $single:ty) => {};
}

pub mod consts;
pub mod types;

#[cfg(test)]
mod test_utils {
    /// Asserts that two floating point numbers are approximately equal, within a given epsilon.
    #[inline(always)]
    pub fn assert_close(a: f64, b: f64, epsilon: f64) {
        assert!(
            (a - b).abs() < epsilon,
            "Expected {} to be approximately equal to {} within epsilon {}",
            a,
            b,
            epsilon
        );
    }
}