//! This module contains the types for the different physical quantities.
//! 
//! # Nomenclature
//! The types are named with a list of dimensions, where each dimension is assigned an exponent.
//! The dimensions are sorted alphabetically, different dimensions are separated by an underscore.
//! ## Base Dimensions
//! - `I` - Electric current
//! - `J` - Luminous intensity
//! - `L` - Length
//! - `M` - Mass
//! - `N` - Amount of substance
//! - `T` - Time
//! - `Θ` - Thermodynamic temperature
//! 
//! ### Additional Dimensions
//! In addition to the base dimensions, the following additional dimensions are defined by this crate:
//! - Currently none but feel free to suggest any additional dimensions that you would like to see added.
//! 
//! ## Exponents
//! - `1` - Positive exponent
//! - `n1` - Negative exponent

pub mod l1;
pub mod l1_m1;
pub mod l1_m1_tn1;
pub mod l1_t1;
pub mod l1_tn1;
pub mod m1;
pub mod t1;
pub mod tn1;