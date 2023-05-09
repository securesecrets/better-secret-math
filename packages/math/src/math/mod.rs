//! Math library for advanced fixed-point math that works with numbers which are considered to have 18 trailing decimals.
//! Uses U256 and I256 for better performance.
pub mod asm;
pub mod common;
mod rebase;
pub mod sd59x18;
pub(crate) mod tens;
pub mod ud60x18;
pub use rebase::*;

#[cfg(test)]
mod tests;

pub const UNIT_U128: u128 = 1_000_000_000_000_000_000u128;
pub const HALF_UNIT_U128: u128 = 500_000_000_000_000_000u128;
pub const LOG2_E_U128: u128 = 1_442_695_040_888_963_407u128;
pub const E_U128: u128 = 2_718_281_828_459_045_235u128;
