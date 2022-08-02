//! Math library for advanced fixed-point math that works with numbers which are considered to have 18 trailing decimals.
//! Uses U256 and I256 for better performance.
pub mod asm;
pub mod core;
mod rebase;
pub mod sd59x18;
pub(crate) mod tens;
pub mod ud60x18;

use ethnum::U256;

pub use rebase::*;

/// 10^36 or 1e36
pub const DOUBLE_SCALE: U256 = U256::new(1_000_000_000_000_000_000_000_000_000_000_000_000u128);
/// 10^18 or 1e18
pub const SCALE: U256 = U256::new(1_000_000_000_000_000_000u128);
pub const SCALE_u128: u128 = 1_000_000_000_000_000_000u128;

/// Half of 10^18.
pub const HALF_SCALE: U256 = U256::new(500_000_000_000_000_000u128);
pub const HALF_SCALE_u128: u128 = 500_000_000_000_000_000u128;

/// log2(e) as an unsigned 18 decimal fixed-point number.
pub const LOG2_E: U256 = U256::new(1_442_695_040_888_963_407u128);
pub const LOG2_E_u128: u128 = 1_442_695_040_888_963_407u128;

/// The mathematical constant e - Euler's number.
pub const E: U256 = U256::new(2_718_281_828_459_045_235u128);
pub const E_u128: u128 = 2_718_281_828_459_045_235u128;

/// @dev The maximum whole value an unsigned 60.18-decimal fixed-point number can have.
pub const MAX_WHOLE_UD60x18: U256 = U256::from_words(
    0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFFF7E52FE5AFE40000,
);

/// @dev The maximum decimal value an unsigned 60.18-decimal fixed-point number can have.
pub const MAX_UD60x18: U256 = U256::MAX;
