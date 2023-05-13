use crate::common::exp10;
use ethnum::{U256};

pub const EXP_MAX_INPUT: U256 = U256::new(133_084258667509499440u128);
/// 192e18 - 1
pub const EXP2_MAX_INPUT: U256 = U256::new(1_919_999_999_999_999_999u128);

/// 10^18 or 1e18
pub const UNIT: U256 = exp10(18);
/// 10^36 or 1e36
pub const UNIT_SQUARED: U256 = exp10(36);
/// Largest power of two divisor of UNIT.
pub const UNIT_LPOTD: U256 = U256::new(262144u128);
/// UNIT inverted mod 2^256.
pub const UNIT_INVERSE: U256 = U256::from_words(
    229681740086561209518615317264092320238,
    298919117238935307856972083127780443753,
);

/// 2^128
pub const TWO_TO_128: U256 = U256::from_words(1, 0);
/// 2^255
pub const TWO_TO_255: U256 = U256::from_words(0x80000000000000000000000000000000, 0);

/// Half of 10^18.
pub const HALF_UNIT: U256 = U256::new(500_000_000_000_000_000u128);

/// log2(e) as an unsigned 18 decimal fixed-point number.
pub const LOG2_E: U256 = U256::new(1_442_695_040_888_963_407u128);

pub const PI: U256 = U256::new(3_141_592_653_589_793_238u128);

/// The mathematical constant e - Euler's number.
pub const E: U256 = U256::new(2_718_281_828_459_045_235u128);

/// @dev The maximum whole value an unsigned 60.18-decimal fixed-point number can have.
pub const MAX_WHOLE_UD60X18: U256 = U256::from_words(
    0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFFF7E52FE5AFE40000,
);

/// @dev The maximum decimal value an unsigned 60.18-decimal fixed-point number can have.
pub const MAX_UD60X18: U256 = U256::MAX;

pub const MAX_UNITD_UD60X18: U256 =
    U256::from_words(0x12725DD1D243ABA0E75FE645CC4873F9, 0xE65AFE688C928E1F21);

/// 115792089237316195423570985008687907853269_984665640564039457
pub const MAX_SCALED_UD60X18: U256 =
    U256::from_words(0x12725DD1D243ABA0E7, 0x5FE645CC4873F9E65AFE688C928E1F21);

/// 340282366920938463463374607431_768211455999999999
pub const SQRT_MAX_UD60X18: U256 = U256::from_words(0x3B9AC9FF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
