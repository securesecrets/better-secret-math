//! For advanced fixed-point math that works with U256 numbers which are considered to have 18 trailing decimals. We call this number representation unsigned 60.18-decimal fixed-point, since there can be up to 60 digits in the integer part and up to 18 decimals in the fractional part. The numbers are bound by the minimum and the maximum values permitted by the CosmWasm type U256.
//!

use super::{
    asm::gt, core, tens::*, MAX_UD60x18, MAX_WHOLE_UD60x18, SCALE_u128, DOUBLE_SCALE, HALF_SCALE,
    LOG2_E, SCALE,
};
use crate::{
    asm::{self},
    core::{most_significant_bit, muldiv, muldiv_fp},
};
use cosmwasm_std::{Decimal256, DivideByZeroError, StdError, StdResult, Uint256};
use ethnum::{AsU256, U256};

/// This pub fn will never be run. It's just here so the code from PRBMathUD 60x18 maintains its original form.
fn phantom_sub(x: i32, y: i32) -> U256 {
    x.as_u256() - y.as_u256()
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
    // 2^192 doesn't fit within the 192.64-bit format used internally in this pub fn.
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
/// See https://en.wikipedia.org/wiki/Floor_and_ceiling_pub fns.
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
pub fn assert_with_precision(actual: U256, expected: U256, error: U256) {
    use crate::core::abs_diff;

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

pub fn log2(x: U256) -> StdResult<U256> {
    if x < SCALE {
        return Err(StdError::generic_err("PRBMathUD60x18 LogInputTooSmall"));
    }
    // Calculate the integer part of the logarithm and add it to the result and finally calculate y = x * 2^(-n).
    let n = most_significant_bit(x / SCALE);

    // The integer part of the logarithm as an unsigned 60.18-decimal fixed-point number. The operation can't overflow
    // because n is maximum 255 and SCALE is 1e18.
    let mut result = n * SCALE;

    // This is y = x * 2^(-n).
    let mut y = x >> n;

    // If y = 1, the fractional part is zero.
    if y == SCALE {
        return Ok(result);
    }

    // Calculate the fractional part via the iterative approximation.
    // The "delta >>= 1" part is equivalent to "delta /= 2", but shifting bits is faster.
    let mut delta = HALF_SCALE;
    while delta > 0 {
        y = (y * y) / SCALE;

        // Is y^2 > 2 and so in the range [2,4)?
        if y >= 2 * SCALE {
            // Add the 2^(-m) factor to the logarithm.
            result += delta;

            // Corresponds to z/2 on Wikipedia.
            y >>= 1;
        }
        delta >>= 1;
    }
    Ok(result)
}

/// @notice Calculates the common logarithm of x.
///
/// @dev First checks if x is an exact power of ten and it stops if yes. If it's not, calculates the common
/// logarithm based on the insight that log/ log2(U256::new(10).
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
        return Err(StdError::generic_err("Log too small."));
    }
    let mut result: U256;
    // Note that the "mul" in this block is the assembly multiplication operation, not the "mul" pub fn defined
    // in this contract.
    // prettier-ignore
    match x {
        QUINTILLIONTH => result = asm::mul(SCALE, phantom_sub(0, 18)),
        HUN_QUADTH => result = asm::mul(SCALE, phantom_sub(1, 18)),
        TEN_QUADTH => result = asm::mul(SCALE, phantom_sub(2, 18)),
        QUADTH => result = asm::mul(SCALE, phantom_sub(3, 18)),
        HUN_TRILTH => result = asm::mul(SCALE, phantom_sub(4, 18)),
        TEN_TRILTH => result = asm::mul(SCALE, phantom_sub(5, 18)),
        TRILTH => result = asm::mul(SCALE, phantom_sub(6, 18)),
        HUN_BILTH => result = asm::mul(SCALE, phantom_sub(7, 18)),
        TEN_BILTH => result = asm::mul(SCALE, phantom_sub(8, 18)),
        BILTH => result = asm::mul(SCALE, phantom_sub(9, 18)),
        HUN_MILTH => result = asm::mul(SCALE, phantom_sub(10, 18)),
        TEN_MILTH => result = asm::mul(SCALE, phantom_sub(11, 18)),
        MILTH => result = asm::mul(SCALE, phantom_sub(12, 18)),
        HUN_THOUSANDTH => result = asm::mul(SCALE, phantom_sub(13, 18)),
        TEN_THOUSANDTH => result = asm::mul(SCALE, phantom_sub(14, 18)),
        THOUSANDTH => result = asm::mul(SCALE, phantom_sub(15, 18)),
        HUNDREDTH => result = asm::mul(SCALE, phantom_sub(16, 18)),
        TENTH => result = asm::mul(SCALE, phantom_sub(17, 18)),
        ONE => result = U256::ZERO,
        TEN => result = SCALE,
        HUNDRED => result = asm::mul(SCALE, U256::new(2u128)),
        THOUSAND => result = asm::mul(SCALE, U256::new(3u128)),
        TEN_THOUSAND => result = asm::mul(SCALE, U256::new(4u128)),
        HUN_THOUSAND => result = asm::mul(SCALE, U256::new(5u128)),
        MIL => result = asm::mul(SCALE, U256::new(6u128)),
        TEN_MIL => result = asm::mul(SCALE, U256::new(7u128)),
        HUN_MIL => result = asm::mul(SCALE, U256::new(8u128)),
        BIL => result = asm::mul(SCALE, U256::new(9u128)),
        TEN_BIL => result = asm::mul(SCALE, U256::new(10u128)),
        HUN_BIL => result = asm::mul(SCALE, U256::new(11u128)),
        TRIL => result = asm::mul(SCALE, U256::new(12u128)),
        TEN_TRIL => result = asm::mul(SCALE, U256::new(13u128)),
        HUN_TRIL => result = asm::mul(SCALE, U256::new(14u128)),
        QUAD => result = asm::mul(SCALE, U256::new(15u128)),
        TEN_QUAD => result = asm::mul(SCALE, U256::new(16u128)),
        HUN_QUAD => result = asm::mul(SCALE, U256::new(17u128)),
        QUIN => result = asm::mul(SCALE, U256::new(18u128)),
        TEN_QUIN => result = asm::mul(SCALE, U256::new(19u128)),
        HUN_QUIN => result = asm::mul(SCALE, U256::new(20u128)),
        SEX => result = asm::mul(SCALE, U256::new(21u128)),
        TEN_SEX => result = asm::mul(SCALE, U256::new(22u128)),
        HUN_SEX => result = asm::mul(SCALE, U256::new(23u128)),
        SEPT => result = asm::mul(SCALE, U256::new(24u128)),
        TEN_SEPT => result = asm::mul(SCALE, U256::new(25u128)),
        HUN_SEPT => result = asm::mul(SCALE, U256::new(26u128)),
        OCT => result = asm::mul(SCALE, U256::new(27u128)),
        TEN_OCT => result = asm::mul(SCALE, U256::new(28u128)),
        HUN_OCT => result = asm::mul(SCALE, U256::new(29u128)),
        NON => result = asm::mul(SCALE, U256::new(30u128)),
        TEN_NON => result = asm::mul(SCALE, U256::new(31u128)),
        HUN_NON => result = asm::mul(SCALE, U256::new(32u128)),
        DECI => result = asm::mul(SCALE, U256::new(33u128)),
        TEN_DECI => result = asm::mul(SCALE, U256::new(34u128)),
        HUN_DECI => result = asm::mul(SCALE, U256::new(35u128)),
        UND => result = asm::mul(SCALE, U256::new(36u128)),
        TEN_UND => result = asm::mul(SCALE, U256::new(37u128)),
        HUN_UND => result = asm::mul(SCALE, U256::new(38u128)),
        DUOD => result = asm::mul(SCALE, U256::new(39u128)),
        TEN_DUOD => result = asm::mul(SCALE, U256::new(40u128)),
        HUN_DUOD => result = asm::mul(SCALE, U256::new(41u128)),
        TRE => result = asm::mul(SCALE, U256::new(42u128)),
        TEN_TRE => result = asm::mul(SCALE, U256::new(43u128)),
        HUN_TRE => result = asm::mul(SCALE, U256::new(44u128)),
        QUATT => result = asm::mul(SCALE, U256::new(45u128)),
        TEN_QUATT => result = asm::mul(SCALE, U256::new(46u128)),
        HUN_QUATT => result = asm::mul(SCALE, U256::new(47u128)),
        QUIND => result = asm::mul(SCALE, U256::new(48u128)),
        TEN_QUIND => result = asm::mul(SCALE, U256::new(49u128)),
        HUN_QUIND => result = asm::mul(SCALE, U256::new(50u128)),
        SEXD => result = asm::mul(SCALE, U256::new(51u128)),
        TEN_SEXD => result = asm::mul(SCALE, U256::new(52u128)),
        HUN_SEXD => result = asm::mul(SCALE, U256::new(53u128)),
        SEPTD => result = asm::mul(SCALE, U256::new(54u128)),
        TEN_SEPTD => result = asm::mul(SCALE, U256::new(55u128)),
        HUN_SEPTD => result = asm::mul(SCALE, U256::new(56u128)),
        OCTOD => result = asm::mul(SCALE, U256::new(57u128)),
        TEN_OCTOD => result = asm::mul(SCALE, U256::new(58u128)),
        HUN_OCTOD => result = asm::mul(SCALE, U256::new(59u128)),
        _ => result = MAX_UD60x18,
    }

    if result == MAX_UD60x18 {
        result = (log2(x)? * SCALE) / 3_321928094887362347;
    }
    Ok(result)
}

/// @notice Divides two unsigned 60.18-decimal fixed-point numbers, returning a new unsigned 60.18-decimal fixed-point number.
///
/// @dev Uses mulDiv to enable overflow-safe multiplication and division.
///
/// Requirements:
/// - The denominator cannot be zero.
///
/// @param x The numerator as an unsigned 60.18-decimal fixed-point number.
/// @param y The denominator as an unsigned 60.18-decimal fixed-point number.
/// @param result The quotient as an unsigned 60.18-decimal fixed-point number.
pub fn div(x: U256, y: U256) -> StdResult<U256> {
    muldiv(x, SCALE, y)
}

/// @notice Yields the excess beyond the floor of x.
/// @dev Based on the odd pub fn definition https://en.wikipedia.org/wiki/Fractional_part.
/// @param x The unsigned 60.18-decimal fixed-point number to get the fractional part of.
/// @param result The fractional part of x as an unsigned 60.18-decimal fixed-point number.
pub fn frac(x: U256) -> U256 {
    x % SCALE
}

/// @notice Calculates geometric mean of x and y, i.e. sqrt(x * y), rounding down.
///
/// @dev Requirements:
/// - x * y must fit within MAX_UD60x18, lest it overflows.
///
/// @param x The first operand as an unsigned 60.18-decimal fixed-point number.
/// @param y The second operand as an unsigned 60.18-decimal fixed-point number.
/// @return result The result as an unsigned 60.18-decimal fixed-point number.
pub fn gm(x: U256, y: U256) -> StdResult<U256> {
    if x == 0 {
        return Ok(U256::ZERO);
    }

    // Checking for overflow this way is faster than letting Solidity do it.
    let xy = x * y;
    if xy / x != y {
        return Err(StdError::generic_err(format!(
            "PRBMathUD60x18__GmOverflow {} {}",
            x, y
        )));
    }

    // We don't need to multiply by the SCALE here because the x*y product had already picked up a factor of SCALE
    // during multiplication. See the comments within the "sqrt" pub fn.
    Ok(sqrt(xy)?)
}

/// @notice Calculates the natural logarithm of x.
///
/// @dev Based on the insight that ln(x) = log2(x) / log2(e).
///
/// Requirements:
/// - All from "log2".
///
/// Caveats:
/// - All from "log2".
/// - This doesn't return exactly 1 for 2.718281828459045235, for that we would need more fine-grained precision.
///
/// @param x The unsigned 60.18-decimal fixed-point number for which to calculate the natural logarithm.
/// @return result The natural logarithm as an unsigned 60.18-decimal fixed-point number.
pub fn ln(x: U256) -> StdResult<U256> {
    // Do the fixed-point multiplication inline to save gas. This is overflow-safe because the maximum value that log2(x)
    // can return is 196205294292027477728.
    Ok((log2(x)? * SCALE) / LOG2_E)
}

/// @notice Multiplies two unsigned 60.18-decimal fixed-point numbers together, returning a new unsigned 60.18-decimal
/// fixed-point number.
/// @dev See the documentation for the "PRBMath.mulDivFixedPoint" pub fn.
/// @param x The multiplicand as an unsigned 60.18-decimal fixed-point number.
/// @param y The multiplier as an unsigned 60.18-decimal fixed-point number.
/// @return result The product as an unsigned 60.18-decimal fixed-point number.
pub fn mul(x: U256, y: U256) -> StdResult<U256> {
    muldiv_fp(x, y)
}

/// @notice Returns PI as an unsigned 60.18-decimal fixed-point number.
pub fn pi() -> U256 {
    U256::new(3_141592653589793238u128)
}

/// @notice Raises x to the power of y.
///
/// @dev Based on the insight that x^y = 2^(log2(x) * y).
///
/// Requirements:
/// - All from "exp2", "log2" and "mul".
///
/// Caveats:
/// - All from "exp2", "log2" and "mul".
/// - Assumes 0^0 is 1.
///
/// @param x Number to raise to given power y, as an unsigned 60.18-decimal fixed-point number.
/// @param y Exponent to raise x to, as an unsigned 60.18-decimal fixed-point number.
/// @return result x raised to power y, as an unsigned 60.18-decimal fixed-point number.
pub fn pow(x: U256, y: U256) -> StdResult<U256> {
    if x == 0 {
        if y == 0 {
            return Ok(SCALE);
        } else {
            return Ok(U256::ZERO);
        }
    } else {
        Ok(exp2(mul(log2(x)?, y)?)?)
    }
}

/// @notice Raises x (unsigned 60.18-decimal fixed-point number) to the power of y (basic unsigned integer) using the
/// famous algorithm "exponentiation by squaring".
///
/// @dev See https://en.wikipedia.org/wiki/Exponentiation_by_squaring
///
/// Requirements:
/// - The result must fit within MAX_UD60x18.
///
/// Caveats:
/// - All from "mul".
/// - Assumes 0^0 is 1.
///
/// @param x The base as an unsigned 60.18-decimal fixed-point number.
/// @param y The exponent as an uint256.
/// @return result The result as an unsigned 60.18-decimal fixed-point number.
pub fn powu(x: U256, y: U256) -> StdResult<U256> {
    // Calculate the first iteration of the loop in advance.
    let mut result = if y & 1 > 0 { x } else { SCALE };
    let mut x = x;
    // Equivalent to "for(y /= 2; y > 0; y /= 2)" but faster.
    let mut new_y = y >> 1;
    while new_y > 0u128 {
        x = muldiv_fp(x, x)?;

        // Equivalent to "y % 2 == 1" but faster.
        if y & 1 > 0 {
            result = muldiv_fp(result, x)?;
        }
        new_y >>= 1;
    }
    Ok(result)
}

/// @notice Returns 1 as an unsigned 60.18-decimal fixed-point number.
pub fn scale() -> U256 {
    SCALE
}

/// @notice Calculates the square root of x, rounding down.
/// @dev Uses the Babylonian method https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method.
///
/// Requirements:
/// - x must be less than MAX_UD60x18 / SCALE.
///
/// @param x The unsigned 60.18-decimal fixed-point number for which to calculate the square root.
/// @return result The result as an unsigned 60.18-decimal fixed-point .
pub fn sqrt(x: U256) -> StdResult<U256> {
    if x > MAX_UD60x18 / SCALE {
        return Err(StdError::generic_err(format!(
            "PRBMathUD60x18__SqrtOverflow {}",
            x
        )));
    }
    // Multiply x by the SCALE to account for the factor of SCALE that is picked up when multiplying two unsigned
    // 60.18-decimal fixed-point numbers together (in this case, those two numbers are both the square root).
    Ok(core::sqrt(x * SCALE))
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
        let pow = pow.into();
        let result: U256 = result.into();
        assert_eq!(result, exp2(pow).unwrap());
    }

    #[rstest]
    #[case("4.0", "16.0")]
    fn test_pow_sqrt(#[case] x: Decimal256, #[case] xpow2: Decimal256) {
        let x: U256 = x.into();
        let xpow2: U256 = xpow2.into();
        let two = 2 * SCALE;
        assert_eq!(pow(x, two).unwrap(), xpow2);
        assert_eq!(sqrt(xpow2).unwrap(), x);
    }

    #[rstest]
    #[case("2324323.0", "2323442.23", "5400430214360.29")]
    fn test_mul(#[case] x: Decimal256, #[case] y: Decimal256, #[case] xy: Decimal256) {
        assert_eq!(xy, x.checked_mul(y).unwrap());
        let xy: U256 = xy.into();
        assert_eq!(xy, muldiv(x.into(), y.into(), SCALE).unwrap());
        assert_eq!(xy, mul(x.into(), y.into()).unwrap());
    }
}
