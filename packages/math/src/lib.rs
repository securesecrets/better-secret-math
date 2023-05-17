//! Math library for advanced fixed-point math that works with numbers which are considered to have 18 trailing decimals.
//! Uses U256 and I256 for better performance.

pub mod macros;
pub use ethnum::*;
pub use primitives::*;
pub mod asserter;
mod primitives;

pub mod common;
pub mod sd59x18;
pub mod ud60x18;

mod asm;
pub(crate) mod tens;

#[cfg(test)]
mod tests;

pub(crate) const UNIT_U128: u128 = 1_000_000_000_000_000_000u128;
pub(crate) const HALF_UNIT_U128: u128 = 500_000_000_000_000_000u128;
pub(crate) const LOG2_E_U128: u128 = 1_442_695_040_888_963_407u128;
pub(crate) const E_U128: u128 = 2_718_281_828_459_045_235u128;
