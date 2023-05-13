use ethnum::I256;

pub const UNIT: I256 = I256::new(crate::UNIT_U128 as i128);
pub const HALF_UNIT: I256 = I256::new(crate::HALF_UNIT_U128 as i128);
pub const LOG2_E: I256 = I256::new(crate::LOG2_E_U128 as i128);
pub const DOUBLE_UNIT: I256 = I256::new(1_000_000_000_000_000_000_000_000_000_000_000_000i128);

pub const MAX_SD59X18: I256 = I256::MAX;

/// @The maximum whole value a signed 59.18-decimal fixed-point number can have.
pub const MAX_WHOLE_SD59X18: I256 = I256::from_words(
    170141183460469231731687303715884105727i128,
    -792003956564819968,
);

/// @dev The minimum value a signed 59.18-decimal fixed-point number can have.
pub const MIN_SD59X18: I256 = I256::MIN;

/// @dev The minimum whole value a signed 59.18-decimal fixed-point number can have.
pub const MIN_WHOLE_SD59X18: I256 = I256::from_words(
    -170141183460469231731687303715884105728i128,
    792003956564819968,
);
