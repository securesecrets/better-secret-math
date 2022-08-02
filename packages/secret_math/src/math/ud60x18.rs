//! For advanced fixed-point math that works with U256 numbers which are considered to have 18 trailing decimals. We call this number representation unsigned 60.18-decimal fixed-point, since there can be up to 60 digits in the integer part and up to 18 decimals in the fractional part. The numbers are bound by the minimum and the maximum values permitted by the CosmWasm type U256.
//!
//! Based off https://github.com/paulrberg/prb-math/blob/main/contracts/PRBMathSD59x18.sol.

use crate::asm::mul;

use super::{
    asm::gt, core, MAX_UD60x18, MAX_WHOLE_UD60x18, SCALE_u128, DOUBLE_SCALE, HALF_SCALE, LOG2_E,
    SCALE,
};
use cosmwasm_std::{Decimal256, DivideByZeroError, StdError, StdResult, Uint256};
use ethnum::U256;

fn sub(x: U256, y: U256) -> I256 {
    x.as_i256() - y.as_i256()
}

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

    /// @notice Calculates the common logarithm of x.
    ///
    /// @dev First checks if x is an exact power of ten and it stops if yes. If it's not, calculates the common
    /// logarithm based on the insight that logU256::new(10(x) = log2(x) / log2(U256::new(10).
    ///
    /// Requirements:
    /// - All from "log2".
    ///
    /// Caveats:
    /// - All from "log2".
    ///
    /// @param x The unsigned 60.18-decimal fixed-point number for which to calculate the common logarithm.
    /// @return result The common logarithm as an unsigned 60.18-decimal fixed-point number.
    pub fn log10(x: U256) -> StdResult<U256> {
        if x < SCALE {
            return Err(StdError::generic_err("Log too small."))
        }
        let mut result: U256;
        // Note that the "mul" in this block is the assembly multiplication operation, not the "mul" function defined
        // in this contract.
        // prettier-ignore
            match x {
                U256::ONE => result = mul(SCALE, sub(0.into(), 18.into())),
                U256::new(10u128) =>  result = mul(SCALE, sub(1.into(), 18.into())),
                U256::new(100u128) =>  result = mul(SCALE, sub(2, 18.into())),
                U256::new(1000) =>  result = mul(SCALE, sub(3, 18.into())),
                U256::new(10000) =>  result = mul(SCALE, sub(4, 18.into())),
                U256::new(100000) =>  result = mul(SCALE, sub(5, 18.into())),
                U256::new(1000000) =>  result = mul(SCALE, sub(6, 18.into())),
                U256::new(10000000) =>  result = mul(SCALE, sub(7, 18.into())),
                U256::new(100000000) =>  result = mul(SCALE, sub(8, 18.into())),
                U256::new(1000000000) =>  result = mul(SCALE, sub(9, 18.into())),
                U256::new(10000000000) =>  result = mul(SCALE, sub(10, 18.into())),
                U256::new(100000000000) =>  result = mul(SCALE, sub(11, 18.into())),
                U256::new(1000000000000) =>  result = mul(SCALE, sub(12, 18.into())),
                U256::new(10000000000000) =>  result = mul(SCALE, sub(13, 18.into())),
                U256::new(100000000000000) =>  result = mul(SCALE, sub(14, 18.into())),
                U256::new(1000000000000000) =>  result = mul(SCALE, sub(15, 18.into())),
                U256::new(10000000000000000) =>  result = mul(SCALE, sub(16, 18.into())),
                U256::new(100000000000000000) =>  result = mul(SCALE, sub(17, 18.into())),
                U256::new(1000000000000000000) =>  result = 0,
                U256::new(10000000000000000000) =>  result = SCALE,
                U256::new(100000000000000000000) =>  result = mul(SCALE, 2),
                U256::new(1000000000000000000000) =>  result = mul(SCALE, 3),
                U256::new(10000000000000000000000) =>  result = mul(SCALE, 4),
                U256::new(100000000000000000000000) =>  result = mul(SCALE, 5),
                U256::new(1000000000000000000000000) =>  result = mul(SCALE, 6),
                U256::new(10000000000000000000000000) =>  result = mul(SCALE, 7),
                U256::new(100000000000000000000000000) =>  result = mul(SCALE, 8),
                U256::new(1000000000000000000000000000) =>  result = mul(SCALE, 9),
                U256::new(10000000000000000000000000000) =>  result = mul(SCALE, 10),
                U256::new(100000000000000000000000000000) =>  result = mul(SCALE, 11),
                U256::new(1000000000000000000000000000000) =>  result = mul(SCALE, 12),
                U256::new(10000000000000000000000000000000) =>  result = mul(SCALE, 13),
                U256::new(100000000000000000000000000000000) =>  result = mul(SCALE, 14),
                U256::new(1000000000000000000000000000000000) =>  result = mul(SCALE, 15),
                U256::new(10000000000000000000000000000000000) =>  result = mul(SCALE, 16),
                U256::new(100000000000000000000000000000000000) =>  result = mul(SCALE, 17),
                U256::new(1000000000000000000000000000000000000) =>  result = mul(SCALE, 18),
                U256::new(10000000000000000000000000000000000000) =>  result = mul(SCALE, 19),
                U256::new(100000000000000000000000000000000000000) =>  result = mul(SCALE, 20),
                U256::new(1000000000000000000000000000000000000000) =>  result = mul(SCALE, 21),
                U256::new(10000000000000000000000000000000000000000) =>  result = mul(SCALE, 22),
                U256::new(100000000000000000000000000000000000000000) =>  result = mul(SCALE, 23),
                U256::new(1000000000000000000000000000000000000000000) =>  result = mul(SCALE, 24),
                U256::new(10000000000000000000000000000000000000000000) =>  result = mul(SCALE, 25),
                U256::new(100000000000000000000000000000000000000000000) =>  result = mul(SCALE, 26),
                U256::new(1000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 27),
                U256::new(10000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 28),
                U256::new(100000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 29),
                U256::new(1000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 30),
                U256::new(10000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 31),
                U256::new(100000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 32),
                U256::new(1000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 33),
                U256::new(10000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 34),
                U256::new(100000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 35),
                U256::new(1000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 36),
                U256::new(10000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 37),
                U256::new(100000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 38),
                U256::new(1000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 39),
                U256::new(10000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 40),
                U256::new(100000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 41),
                U256::new(1000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 42),
                U256::new(10000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 43),
                U256::new(100000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 44),
                U256::new(1000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 45),
                U256::new(10000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 46),
                U256::new(100000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 47),
                U256::new(1000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 48),
                U256::new(10000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 49),
                U256::new(100000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 50),
                U256::new(1000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 51),
                U256::new(10000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 52),
                U256::new(100000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 53),
                U256::new(1000000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 54),
                U256::new(10000000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 55),
                U256::new(100000000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 56),
                U256::new(1000000000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 57),
                U256::new(10000000000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 58),
                U256::new(100000000000000000000000000000000000000000000000000000000000000000000000000000) =>  result = mul(SCALE, 59),
                _ => result = MAX_UD60x18,
            }

        if result == MAX_UD60x18 {
            result = (log2(x) * SCALE) / 3_321928094887362347;
        }
        Ok(result)
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
