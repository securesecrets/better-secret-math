use cosmwasm_std::{StdError, StdResult};
use ethnum::{I256, U256};

use crate::asm::Asm;
use crate::common::muldiv18;

use super::common;
use super::{HALF_UNIT_U128, LOG2_E_U128, UNIT_U128};

const UNIT: I256 = I256::new(UNIT_U128 as i128);
const HALF_UNIT: I256 = I256::new(HALF_UNIT_U128 as i128);
const LOG2_E: I256 = I256::new(LOG2_E_U128 as i128);
const DOUBLE_UNIT: I256 = I256::new(1_000_000_000_000_000_000_000_000_000_000_000_000i128);

const MAX_SD59X18: I256 = I256::MAX;

/// @The maximum whole value a signed 59.18-decimal fixed-point number can have.
const MAX_WHOLE_SD59X18: I256 = I256::from_words(
    170141183460469231731687303715884105727i128,
    -792003956564819968,
);

/// @dev The minimum value a signed 59.18-decimal fixed-point number can have.
const MIN_SD59X18: I256 = I256::MIN;

/// @dev The minimum whole value a signed 59.18-decimal fixed-point number can have.
const MIN_WHOLE_SD59X18: I256 = I256::from_words(
    -170141183460469231731687303715884105728i128,
    792003956564819968,
);

pub fn pow(x: I256, y: I256) -> StdResult<I256> {
    if x == 0 {
        if y == 0 {
            return Ok(UNIT);
        } else {
            return Ok(I256::ZERO);
        }
    } else {
        exp2(mul(log2(x)?, y)?)
    }
}

/// @notice Multiplies two signed 59.18-decimal fixed-point numbers together, returning a new signed 59.18-decimal
/// fixed-point number.
///
/// @dev Variant of "mulDiv" that works with signed numbers and employs constant folding, i.e. the denominator is
/// always 1e18.
///
/// Requirements:
/// - All from "PRBMath.mulDivFixedPoint".
/// - None of the inputs can be MIN_SD59X18
/// - The result must fit within MAX_SD59X18.
///
/// Caveats:
/// - The body is purposely left uncommented; see the NatSpec comments in "PRBMath.mulDiv" to understand how this works.
///
/// @param x The multiplicand as a signed 59.18-decimal fixed-point number.
/// @param y The multiplier as a signed 59.18-decimal fixed-point number.
/// @return result The product as a signed 59.18-decimal fixed-point number.
pub fn mul(x: I256, y: I256) -> StdResult<I256> {
    if x == MIN_SD59X18 || y == MIN_SD59X18 {
        return Err(StdError::generic_err(format!(
            "PRBMathSD59X18_MulInputTooSmall {}",
            x
        )));
    }

    let ax: U256;
    let ay: U256;
    ax = if x < 0 { (-x).as_u256() } else { x.as_u256() };
    ay = if y < 0 { (-y).as_u256() } else { y.as_u256() };

    let r_abs = muldiv18(ax, ay)?;
    if r_abs > MAX_SD59X18.as_u256() {
        return Err(StdError::generic_err(format!(
            "PRBMathSD59X18__MulOverflow {}",
            r_abs
        )));
    }

    let sx = Asm::sgt(x, Asm::sub(U256::ZERO, U256::ONE));
    let sy = Asm::sgt(y, Asm::sub(U256::ZERO, U256::ONE));

    let result = if sx ^ sy == 1 {
        r_abs.as_i256() * I256::MINUS_ONE
    } else {
        r_abs.as_i256()
    };
    Ok(result)
}

/// @notice Calculates the natural exponent of x.
///
/// @dev Based on the insight that e^x = 2^(x * log2(e)).
///
/// Requirements:
/// - All from "log2".
/// - x must be less than 133.084258667509499441.
///
/// Caveats:
/// - All from "exp2".
/// - For any x less than -41.446531673892822322, the result is zero.
///
/// @param x The exponent as a signed 59.18-decimal fixed-point number.
/// @return result The result as a signed 59.18-decimal fixed-point number.
pub fn exp(x: I256) -> StdResult<I256> {
    // Without this check, the value passed to "exp2" would be less than -59.794705707972522261.
    if x < -41_446531673892822322 {
        return Ok(I256::ZERO);
    }

    // Without this check, the value passed to "exp2" would be greater than 192.
    if x >= 133_084258667509499441 {
        return Err(StdError::generic_err(format!(
            "PRBMathSD59X18__ExpInputTooBig {}",
            x
        )));
    }

    // Do the fixed-point multiplication inline to save gas.
    let double_scale_product = x * LOG2_E;
    Ok(exp2((double_scale_product + HALF_UNIT) / UNIT)?)
}

/// @notice Calculates the binary exponent of x using the binary fraction method.
///
/// @dev See https://ethereum.stackexchange.com/q/79903/24693.
///
/// Requirements:
/// - x must be 192 or less.
/// - The result must fit within MAX_SD59X18.
///
/// Caveats:
/// - For any x less than -59.794705707972522261, the result is zero.
///
/// @param x The exponent as a signed 59.18-decimal fixed-point number.
/// @return result The result as a signed 59.18-decimal fixed-point number.
pub fn exp2(x: I256) -> StdResult<I256> {
    // This works because 2^(-x) = 1/2^x.
    if x < 0 {
        // 2^59.794705707972522262 is the maximum number whose inverse does not truncate down to zero.
        if x < -59_794705707972522261 {
            return Ok(I256::ZERO);
        }

        // Do the fixed-point inversion inline to save gas. The numerator is UNIT * UNIT.
        Ok(DOUBLE_UNIT / exp2(x * I256::MINUS_ONE)?)
    } else {
        // 2^192 doesn't fit within the 192.64-bit format used internally in this function.
        if x >= 192 * UNIT {
            return Err(StdError::generic_err(format!(
                "PRBMathSD59X18__Exp2InputTooBig {}",
                x
            )));
        }

        // Convert x to the 192.64-bit fixed-point format.
        let x192x64 = (x.as_u256() << 64) / UNIT.as_u256();

        // Safe to convert the result to int256 directly because the maximum input allowed is 192.
        Ok(common::exp2(x192x64).as_i256())
    }
}

/// Calculates the natural logarithm of x.
///
/// Based on the insight that ln(x) = log2(x) / log2(e).
///
/// Requirements:
/// - All from "log2".
///
/// Caveats:
/// - All from "log2".
/// - This doesn't return exactly 1 for 2718281828459045235, for that we would need more fine-grained precision.
///
/// x - The signed 59.18-decimal fixed-point number for which to calculate the natural logarithm.
///
/// returns the natural logarithm as a signed 59.18-decimal fixed-point number.
pub fn ln(x: I256) -> StdResult<I256> {
    // Do the fixed-point multiplication inline to save gas. This is overflow-safe because the maximum value that log2(x)
    // can return is 195205294292027477728.
    Ok((log2(x)? * UNIT) / LOG2_E)
}

/// @notice Calculates the binary logarithm of x.
///
/// @dev Based on the iterative approximation algorithm.
/// https://en.wikipedia.org/wiki/Binary_logarithm#Iterative_approximation
///
/// Requirements:
/// - x must be greater than zero.
///
/// Caveats:
/// - The results are not perfectly accurate to the last decimal, due to the lossy precision of the iterative approximation.
///
/// @param x The signed 59.18-decimal fixed-point number for which to calculate the binary logarithm.
/// @return result The binary logarithm as a signed 59.18-decimal fixed-point number.
pub fn log2(mut x: I256) -> StdResult<I256> {
    if x <= 0 {
        return Err(StdError::generic_err("LogInputTooSmall"));
    }
    let sign: I256;
    // This works because log2(x) = -log2(1/x).
    if x >= UNIT {
        sign = I256::ONE;
    } else {
        sign = I256::MINUS_ONE;
        // Do the fixed-point inversion inline to save gas. The numerator is UNIT * UNIT.
        x = I256::new(1000000000000000000000000000000000000i128) / x;
    }

    // Calculate the integer part of the logarithm and add it to the result and finally calculate y = x * 2^(-n).
    let a = (x / UNIT).as_u256();
    let n = super::common::most_significant_bit(a).as_i256();

    // The integer part of the logarithm as a signed 59.18-decimal fixed-point number. The operation can't overflow
    // because n is maximum 255, UNIT is 1e18 and sign is either 1 or -1.
    let mut result = n * UNIT;

    // This is y = x * 2^(-n).
    let mut y = x >> n;

    // If y = 1, the fractional part is zero.
    if y == UNIT {
        return Ok(result * sign);
    }

    // Calculate the fractional part via the iterative approximation.
    // The "delta >>= 1" part is equivalent to "delta /= 2", but shifting bits is faster.
    let mut delta = HALF_UNIT;
    while delta > 0 {
        y = (y * y) / UNIT;

        // Is y^2 > 2 and so in the range [2,4)?
        if y >= 2 * UNIT {
            // Add the 2^(-m) factor to the logarithm.
            result += delta;

            // Corresponds to z/2 on Wikipedia.
            y >>= 1;
        }
        delta >>= 1;
    }
    result *= sign;
    Ok(result)
}

/// @notice Calculates the square root of x, rounding down.
/// @dev Uses the Babylonian method https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method.
///
/// Requirements:
/// - x cannot be negative.
/// - x must be less than MAX_SD59X18 / UNIT.
///
/// @param x The signed 59.18-decimal fixed-point number for which to calculate the square root.
/// @return result The result as a signed 59.18-decimal fixed-point .
pub fn sqrt(x: I256) -> StdResult<I256> {
    if x < 0 {
        return Err(StdError::generic_err(format!(
            "PRBMathSD59X18__SqrtNegativeInput {}",
            x
        )));
    }
    if x > MAX_SD59X18 / UNIT {
        return Err(StdError::generic_err(format!(
            "PRBMathSD59X18__SqrtOverflow {}",
            x
        )));
    }
    // Multiply x by the UNIT to account for the factor of UNIT that is picked up when multiplying two signed
    // 59.18-decimal fixed-point numbers together (in this case, those two numbers are both the square root).
    Ok(common::sqrt((x * UNIT).as_u256()).as_i256())
}

/// Gets the scale as a signed int 256
pub fn scale() -> I256 {
    UNIT
}

#[cfg(test)]
mod test {
    use super::*;
    use ethnum::I256;

    #[test]
    fn test() {
        let int2 = I256::from_str_prefixed(
            "-57896044618658097711785492504343953926634992332820282019728000000000000000000",
        )
        .unwrap();
        let int = I256::from_str_prefixed(
            "57896044618658097711785492504343953926634992332820282019728000000000000000000",
        )
        .unwrap();
        assert_eq!(MIN_WHOLE_SD59X18, int2);
        assert_eq!(MAX_WHOLE_SD59X18, int);
    }
}
