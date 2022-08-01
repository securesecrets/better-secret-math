//! For advanced fixed-point math that works with U256 numbers which are considered to have 18 trailing decimals. We call this number representation unsigned 60.18-decimal fixed-point, since there can be up to 60 digits in the integer part and up to 18 decimals in the fractional part. The numbers are bound by the minimum and the maximum values permitted by the CosmWasm type U256.
//!
//! Based off https://github.com/paulrberg/prb-math/blob/main/contracts/PRBMathSD59x18.sol.

use super::{
    asm::gt, core, MAX_UD60x18, MAX_WHOLE_UD60x18, SCALE_u128, DOUBLE_SCALE, HALF_SCALE, LOG2_E,
    SCALE,
};
use cosmwasm_std::{Decimal256, DivideByZeroError, StdError, StdResult, Uint256};
use ethnum::U256;

/// Calculates the binary exponent of x using the binary fraction method.
///
/// See https://ethereum.stackexchange.com/q/79903/24693.
///
/// Requirements:
/// - x must be 192 or less.
/// - the result must fit within 60.18-bit format.
///
pub fn exp2(x: U256) -> StdResult<U256> {
    // 2^192 doesn't fit within the 192.64-bit format used internally in this function.
    if x >= U256::new(192_000_000_000_000_000_000u128) {
        return Err(StdError::generic_err(format!("Exp2InputTooBig {}", x)));
    }
    let x192x64 = (x << 64) / SCALE;
    Ok(core::exp2(x192x64))
}

/// Calculates the natural exponent of x.
///
/// Based on insight that e^x = 2^(x * log2(e)).
///
/// Requirements:
/// - All from "log2"
/// - x must be less than 133.084258667509499441.
pub fn exp(x: U256) -> StdResult<U256> {
    if x >= U256::new(133_084258667509499441u128) {
        return Err(StdError::generic_err(format!("ExpInputTooBig {}", x)));
    }
    let double_scale_product = x * LOG2_E;
    exp2((double_scale_product + HALF_SCALE) / SCALE)
}

/// Yields the least unsigned value greater than or equal to x.
///
/// x must be less than or equal to U256::MAX();
pub fn ceil(x: U256) -> StdResult<U256> {
    if x > MAX_WHOLE_UD60x18 {
        return Err(StdError::generic_err(format!(
            "{} must be less than or equal to U256::MAX",
            x
        )));
    }
    let remainder = x % SCALE;
    let delta = SCALE - remainder;
    let factor = gt(remainder, U256::ZERO);
    let x = x + (delta * factor);
    Ok(x)
}

/// Yields the greatest unsigned 60.18 decimal fixed-point number less than or equal to x.
/// Optimized for fractional value inputs, because for every whole value there are (1e18 - 1) fractional counterparts.
/// See https://en.wikipedia.org/wiki/Floor_and_ceiling_functions.
pub fn floor(x: U256) -> U256 {
    let remainder = x % SCALE;
    let factor = gt(remainder, U256::ZERO);
    x - (remainder * factor)
}

/// @notice Calculates 1 / x, rounding toward zero.
///
/// @param x The unsigned 60.18-decimal fixed-point number for which to calculate the inverse.
/// @return result The inverse as an unsigned 60.18-decimal fixed-point number
pub fn inv(x: U256) -> StdResult<U256> {
    if x == 0 {
        return Err(StdError::DivideByZero {
            source: DivideByZeroError {
                operand: "/".to_string(),
            },
        });
    }
    let res = DOUBLE_SCALE / x;
    Ok(res)
}

/// Converts some U256 in 60.18-decimal form to a Decimal256.
pub fn to_dec(x: U256) -> Decimal256 {
    Decimal256::new(Uint256::from_be_bytes(x.to_be_bytes()))
}

/// Converts some U256 in 60.18-decimal form to a Uint256 in 60.18 decimal form.
pub fn to_uint(x: U256) -> Uint256 {
    Uint256::from_be_bytes(x.to_be_bytes())
}

/// Converts some U256 in standard form to 60.18-decimal form to a Uint256 in 60.18 decimal form.
pub fn from_u256(x: U256) -> StdResult<U256> {
    if x > MAX_UD60x18 / SCALE {
        return Err(StdError::generic_err(format!(
            "PRBMathUD60x18__FromUintOverflow {}",
            x
        )));
    }
    Ok(x * SCALE)
}

/// Converts some Decimal256 into a 60.18-decimal U256.
pub fn from_dec(x: Decimal256) -> StdResult<U256> {
    if x > Decimal256::MAX {
        return Err(StdError::generic_err(format!(
            "PRBMathUD60x18__FromUintOverflow {}",
            x
        )));
    }
    Ok(U256::from_be_bytes(x.atomics().to_be_bytes()))
}

/// @notice Converts a number from basic integer form to unsigned 60.18-decimal fixed-point representation.
///
/// @dev Requirements:
/// - x must be less than or equal to MAX_UD60x18 divided by SCALE.
///
/// @param x The basic integer to convert.
/// @param result The same number in unsigned 60.18-decimal fixed-point representation.
pub fn from_uint(x: Uint256) -> StdResult<U256> {
    if x > Decimal256::MAX.atomics() / Uint256::from_u128(SCALE_u128) {
        return Err(StdError::generic_err(format!(
            "PRBMathUD60x18__FromUintOverflow {}",
            x
        )));
    }
    let x = U256::from_be_bytes(x.to_be_bytes());
    Ok(x * SCALE)
}

/// Asserts that 2 unsigned 60.18-decimal fixed-point values are within some decimal precision error.
pub fn assert_with_precision(actual: U256, expected: U256, error: U256 ) {
    use crate::core::{abs_diff, muldiv};

    if error > U256::ONE * SCALE {
        panic!("Error precision cannot be 1.")
    }
    let err = abs_diff(actual, expected);
    let acceptable = muldiv(expected, error, SCALE).unwrap();

    assert!(
        err <= acceptable,
        "Check failed - actual: {}, expected: {}, exceeds acceptable error {}",
        actual,
        expected,
        error
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("3.0", "8.0")]
    #[case("8.0", "256.0")]
    #[case("2.5", "5.656854249492380195")]
    fn test_exp2(#[case] pow: Decimal256, #[case] result: Decimal256) {
        let pow = from_dec(pow).unwrap();
        let result = from_dec(result).unwrap();
        assert_eq!(result, exp2(pow).unwrap());
    }
}
