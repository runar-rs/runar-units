//! This module contains the types for the different physical quantities.
//!
//! # Nomenclature
//! The types are named with a list of dimensions, where each dimension is assigned an exponent.
//! The dimensions are sorted alphabetically, different dimensions are separated by an underscore.
//! ## Base Dimensions
//! Dimensions are composed of exponents of the base dimensions. Base dimensions should be named
//! with a single latin letter, or written out greek letters.
//! - `I` - Electric current
//! - `J` - Luminous intensity
//! - `L` - Length
//! - `M` - Mass
//! - `N` - Amount of substance
//! - `T` - Time
//! - `Theta` - Thermodynamic temperature
//!
//! ### Additional Dimensions
//! In addition to the SI base dimensions, the following additional dimensions are defined by this
//! crate:
//! - `Sr` - Steradian (solid angle). In the SI System, the steradian is a dimensionless
//! derived unit. However, this crate defines it as a separate dimension to allow for more precise
//! type checking and to avoid confusion with other dimensionless quantities, or confusion of
//! other quantities, that are given per steradian, such as luminous intensity or radiance.
//!
//! ## Exponents
//! - `1` - Positive exponent
//! - `n1` - Negative exponent

pub mod i_1;
pub use i_1 as electric_current;

pub mod i_1_t_1;
pub use i_1_t_1 as electric_charge;

pub mod i_n1_l_2_m_1_t_n3;
pub use i_n1_l_2_m_1_t_n3 as electric_potential;

pub mod j_1;
pub use j_1 as luminous_intensity;

pub mod j_1_sr_1;
pub use j_1_sr_1 as luminous_flux;

pub mod l_1;
pub use l_1 as length;

pub mod l_1_m_1;

pub mod l_1_m_1_t_n1;
pub use l_1_m_1_t_n1 as impulse;

pub mod l_1_m_1_t_n2;
pub use l_1_m_1_t_n2 as force;

pub mod l_1_t_1;

pub mod l_1_t_n1;
pub use l_1_t_n1 as speed;

pub mod l_2;
pub use l_2 as area;

pub mod l_3;
pub use l_3 as volume;

pub mod m_1;
pub use m_1 as mass;

pub mod n_1;
pub use n_1 as amount_of_substance;

pub mod sr_1;
pub use sr_1 as solid_angle;

pub mod t_1;
pub use t_1 as time;

pub mod t_n1;
pub use t_n1 as frequency;

pub mod t_n1_theta_1;

pub mod theta_1;
pub use theta_1 as thermodynamic_temperature;
