use cosmwasm_std::{StdError, StdResult};
use ethnum::{I256};

use super::{HALF_SCALE_u128, LOG2_E_u128, SCALE_u128};

const SCALE: I256 = I256::new(SCALE_u128 as i128);
const HALF_SCALE: I256 = I256::new(HALF_SCALE_u128 as i128);
const LOG2_E: I256 = I256::new(LOG2_E_u128 as i128);

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
    Ok((log2(x)? * SCALE) / LOG2_E)
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
    if x >= SCALE {
        sign = I256::ONE;
    } else {
        sign = I256::MINUS_ONE;
        // Do the fixed-point inversion inline to save gas. The numerator is SCALE * SCALE.
        x = I256::new(1000000000000000000000000000000000000i128) / x;
    }

    // Calculate the integer part of the logarithm and add it to the result and finally calculate y = x * 2^(-n).
    let a = (x / SCALE).as_u256();
    let n = super::core::most_significant_bit(a).as_i256();

    // The integer part of the logarithm as a signed 59.18-decimal fixed-point number. The operation can't overflow
    // because n is maximum 255, SCALE is 1e18 and sign is either 1 or -1.
    let mut result = n * SCALE;

    // This is y = x * 2^(-n).
    let mut y = x >> n;

    // If y = 1, the fractional part is zero.
    if y == SCALE {
        return Ok(result * sign);
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
    result *= sign;
    Ok(result)
}
