//! Math helpers for pitchy.
//!
//! This module provides floating-point functions that abstract over
//! `std` and `no_std` environments. When the `std` feature is enabled,
//! it uses standard library implementations. When disabled, it falls back
//! to the `libm` crate, which provides portable math functions for `f64`.

#[cfg(feature = "std")]
#[inline]
pub fn powf2(exp: f64) -> f64 {
    2f64.powf(exp)
}

#[cfg(not(feature = "std"))]
#[inline]
pub fn powf2(exp: f64) -> f64 {
    libm::pow(2.0, exp)
}

#[cfg(feature = "std")]
#[inline]
pub fn log2(x: f64) -> f64 {
    x.log2()
}

#[cfg(not(feature = "std"))]
#[inline]
pub fn log2(x: f64) -> f64 {
    libm::log2(x)
}

#[cfg(feature = "std")]
#[inline]
pub fn round(x: f64) -> f64 {
    x.round()
}

#[cfg(not(feature = "std"))]
#[inline]
pub fn round(x: f64) -> f64 {
    libm::round(x)
}
