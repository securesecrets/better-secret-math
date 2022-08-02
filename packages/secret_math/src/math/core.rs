//! Common mathematical functions used in ud60x18 and sd59x18. Note that this shared library does not always assume the unsigned 60.18-decimal fixed-point representation. When it does not, it is explicitly mentioned in the documentation.
//! Forks methods from here - https://github.com/paulrberg/prb-math/blob/main/contracts/PRBMath.sol.
use core::panic;
use std::ops::Not;

use super::asm::*;
use cosmwasm_std::{OverflowError, OverflowOperation, StdError, StdResult};
use super::tens::*;

use ethnum::U256;
const SCALE: U256 = U256::new(1_000_000_000_000_000_000u128);

/// Finds whether or not some Uint256 is odd.
pub fn is_odd(x: U256) -> bool {
    x & 1 == 1
}

/// Calculates the arithmetic average of x and y, rounding down.
pub fn avg(x: U256, y: U256) -> U256 {
    // This can never overflow.
    let mut result = (x >> 1) + (y >> 1);
    // If both numbers are odd, the 0.5 remainder gets truncated twice so we add it back.
    if is_odd(x) && is_odd(y) {
        result += U256::ONE;
    }
    result
}

/// Takes the absolute difference of two unsigned ints.
pub fn abs_diff(x: U256, y: U256) -> U256 {
    if x > y { x - y } else { y - x }
}

/// Calculates the square root of x, rounding down.
/// Uses the Babylonian method https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method.
///
/// Caveats:
/// - This function does not work with fixed-point numbers.
///
/// @param x The uint256 number for which to calculate the square root.
/// @return result The result as an uint256.
pub fn sqrt(x: U256) -> U256 {
    if (x == 0) {
        return U256::ZERO;
    }

    // Set the initial guess to the least power of two that is greater than or equal to sqrt(x).
    let mut x_aux = x;
    let mut result = U256::ONE;
    // Can't panic.
    if x_aux >= U256::from_str_hex("0x100000000000000000000000000000000").unwrap() {
        x_aux >>= 128;
        result <<= 64;
    }
    if x_aux >= 0x10000000000000000 {
        x_aux >>= 64;
        result <<= 32;
    }
    if x_aux >= 0x100000000 {
        x_aux >>= 32;
        result <<= 16;
    }
    if x_aux >= 0x10000 {
        x_aux >>= 16;
        result <<= 8;
    }
    if x_aux >= 0x100 {
        x_aux >>= 8;
        result <<= 4;
    }
    if x_aux >= 0x10 {
        x_aux >>= 4;
        result <<= 2;
    }
    if x_aux >= 0x4 {
        result <<= 1;
    }

    // The operations can never overflow because the result is max 2^127 when it enters this block.
    result = (result + x / result) >> 1;
    result = (result + x / result) >> 1;
    result = (result + x / result) >> 1;
    result = (result + x / result) >> 1;
    result = (result + x / result) >> 1;
    result = (result + x / result) >> 1;
    result = (result + x / result) >> 1; // Seven iterations should be enough
    let rounded_down_result = x / result;
    let result = if result <= rounded_down_result {
        rounded_down_result
    } else {
        result
    };
    result
}

/// Calculates the binary exponent of x (2^x) using the binary fraction method.
///
/// Has to use 192.64-bit fixed-point numbers so x is the exponent as an unsigned 192.64-bit fixed-point number.
///
/// The result is an unsigned 60.18-decimal fixed-point number.
pub(crate) fn exp2(x: U256) -> U256 {
    // Guaranteed not to panic.
    let mut result =
        U256::from_str_hex("0x800000000000000000000000000000000000000000000000").unwrap();

    if x & U256::new(0x8000000000000000u128) > 0 {
        result = (result * U256::new(0x16A09E667F3BCC909)) >> 64;
    }
    if x & U256::new(0x4000000000000000) > 0 {
        result = (result * U256::new(0x1306FE0A31B7152DF)) >> 64;
    }
    if x & U256::new(0x2000000000000000) > 0 {
        result = (result * U256::new(0x1172B83C7D517ADCE)) >> 64;
    }
    if x & U256::new(0x1000000000000000) > 0 {
        result = (result * U256::new(0x10B5586CF9890F62A)) >> 64;
    }
    if x & U256::new(0x800000000000000) > 0 {
        result = (result * U256::new(0x1059B0D31585743AE)) >> 64;
    }
    if x & U256::new(0x400000000000000) > 0 {
        result = (result * U256::new(0x102C9A3E778060EE7)) >> 64;
    }
    if x & U256::new(0x200000000000000) > 0 {
        result = (result * U256::new(0x10163DA9FB33356D8)) >> 64;
    }
    if x & U256::new(0x100000000000000) > 0 {
        result = (result * U256::new(0x100B1AFA5ABCBED61)) >> 64;
    }
    if x & U256::new(0x80000000000000) > 0 {
        result = (result * U256::new(0x10058C86DA1C09EA2)) >> 64;
    }
    if x & U256::new(0x40000000000000) > 0 {
        result = (result * U256::new(0x1002C605E2E8CEC50)) >> 64;
    }
    if x & U256::new(0x20000000000000) > 0 {
        result = (result * U256::new(0x100162F3904051FA1)) >> 64;
    }
    if x & U256::new(0x10000000000000) > 0 {
        result = (result * U256::new(0x1000B175EFFDC76BA)) >> 64;
    }
    if x & U256::new(0x8000000000000) > 0 {
        result = (result * U256::new(0x100058BA01FB9F96D)) >> 64;
    }
    if x & U256::new(0x4000000000000) > 0 {
        result = (result * U256::new(0x10002C5CC37DA9492)) >> 64;
    }
    if x & U256::new(0x2000000000000) > 0 {
        result = (result * U256::new(0x1000162E525EE0547)) >> 64;
    }
    if x & U256::new(0x1000000000000) > 0 {
        result = (result * U256::new(0x10000B17255775C04)) >> 64;
    }
    if x & U256::new(0x800000000000) > 0 {
        result = (result * U256::new(0x1000058B91B5BC9AE)) >> 64;
    }
    if x & U256::new(0x400000000000) > 0 {
        result = (result * U256::new(0x100002C5C89D5EC6D)) >> 64;
    }
    if x & U256::new(0x200000000000) > 0 {
        result = (result * U256::new(0x10000162E43F4F831)) >> 64;
    }
    if x & U256::new(0x100000000000) > 0 {
        result = (result * U256::new(0x100000B1721BCFC9A)) >> 64;
    }
    if x & U256::new(0x80000000000) > 0 {
        result = (result * U256::new(0x10000058B90CF1E6E)) >> 64;
    }
    if x & U256::new(0x40000000000) > 0 {
        result = (result * U256::new(0x1000002C5C863B73F)) >> 64;
    }
    if x & U256::new(0x20000000000) > 0 {
        result = (result * U256::new(0x100000162E430E5A2)) >> 64;
    }
    if x & U256::new(0x10000000000) > 0 {
        result = (result * U256::new(0x1000000B172183551)) >> 64;
    }
    if x & U256::new(0x8000000000) > 0 {
        result = (result * U256::new(0x100000058B90C0B49)) >> 64;
    }
    if x & U256::new(0x4000000000) > 0 {
        result = (result * U256::new(0x10000002C5C8601CC)) >> 64;
    }
    if x & U256::new(0x2000000000) > 0 {
        result = (result * U256::new(0x1000000162E42FFF0)) >> 64;
    }
    if x & U256::new(0x1000000000) > 0 {
        result = (result * U256::new(0x10000000B17217FBB)) >> 64;
    }
    if x & U256::new(0x800000000) > 0 {
        result = (result * U256::new(0x1000000058B90BFCE)) >> 64;
    }
    if x & U256::new(0x400000000) > 0 {
        result = (result * U256::new(0x100000002C5C85FE3)) >> 64;
    }
    if x & U256::new(0x200000000) > 0 {
        result = (result * U256::new(0x10000000162E42FF1)) >> 64;
    }
    if x & U256::new(0x100000000) > 0 {
        result = (result * U256::new(0x100000000B17217F8)) >> 64;
    }
    if x & U256::new(0x80000000) > 0 {
        result = (result * U256::new(0x10000000058B90BFC)) >> 64;
    }
    if x & U256::new(0x40000000) > 0 {
        result = (result * U256::new(0x1000000002C5C85FE)) >> 64;
    }
    if x & U256::new(0x20000000) > 0 {
        result = (result * U256::new(0x100000000162E42FF)) >> 64;
    }
    if x & U256::new(0x10000000) > 0 {
        result = (result * U256::new(0x1000000000B17217F)) >> 64;
    }
    if x & U256::new(0x8000000) > 0 {
        result = (result * U256::new(0x100000000058B90C0)) >> 64;
    }
    if x & U256::new(0x4000000) > 0 {
        result = (result * U256::new(0x10000000002C5C860)) >> 64;
    }
    if x & U256::new(0x2000000) > 0 {
        result = (result * U256::new(0x1000000000162E430)) >> 64;
    }
    if x & U256::new(0x1000000) > 0 {
        result = (result * U256::new(0x10000000000B17218)) >> 64;
    }
    if x & U256::new(0x800000) > 0 {
        result = (result * U256::new(0x1000000000058B90C)) >> 64;
    }
    if x & U256::new(0x400000) > 0 {
        result = (result * U256::new(0x100000000002C5C86)) >> 64;
    }
    if x & U256::new(0x200000) > 0 {
        result = (result * U256::new(0x10000000000162E43)) >> 64;
    }
    if x & U256::new(0x100000) > 0 {
        result = (result * U256::new(0x100000000000B1721)) >> 64;
    }
    if x & U256::new(0x80000) > 0 {
        result = (result * U256::new(0x10000000000058B91)) >> 64;
    }
    if x & U256::new(0x40000) > 0 {
        result = (result * U256::new(0x1000000000002C5C8)) >> 64;
    }
    if x & U256::new(0x20000) > 0 {
        result = (result * U256::new(0x100000000000162E4)) >> 64;
    }
    if x & U256::new(0x10000) > 0 {
        result = (result * U256::new(0x1000000000000B172)) >> 64;
    }
    if x & U256::new(0x8000) > 0 {
        result = (result * U256::new(0x100000000000058B9)) >> 64;
    }
    if x & U256::new(0x4000) > 0 {
        result = (result * U256::new(0x10000000000002C5D)) >> 64;
    }
    if x & U256::new(0x2000) > 0 {
        result = (result * U256::new(0x1000000000000162E)) >> 64;
    }
    if x & U256::new(0x1000) > 0 {
        result = (result * U256::new(0x10000000000000B17)) >> 64;
    }
    if x & U256::new(0x800) > 0 {
        result = (result * U256::new(0x1000000000000058C)) >> 64;
    }
    if x & U256::new(0x400) > 0 {
        result = (result * U256::new(0x100000000000002C6)) >> 64;
    }
    if x & U256::new(0x200) > 0 {
        result = (result * U256::new(0x10000000000000163)) >> 64;
    }
    if x & U256::new(0x100) > 0 {
        result = (result * U256::new(0x100000000000000B1)) >> 64;
    }
    if x & U256::new(0x80) > 0 {
        result = (result * U256::new(0x10000000000000059)) >> 64;
    }
    if x & U256::new(0x40) > 0 {
        result = (result * U256::new(0x1000000000000002C)) >> 64;
    }
    if x & U256::new(0x20) > 0 {
        result = (result * U256::new(0x10000000000000016)) >> 64;
    }
    if x & U256::new(0x10) > 0 {
        result = (result * U256::new(0x1000000000000000B)) >> 64;
    }
    if x & U256::new(0x8) > 0 {
        result = (result * U256::new(0x10000000000000006)) >> 64;
    }
    if x & U256::new(0x4) > 0 {
        result = (result * U256::new(0x10000000000000003)) >> 64;
    }
    if x & U256::new(0x2) > 0 {
        result = (result * U256::new(0x10000000000000001)) >> 64;
    }
    if x & U256::new(0x1) > 0 {
        result = (result * U256::new(0x10000000000000001)) >> 64;
    }

    result *= SCALE;
    result >>= (U256::new(191u128) - (x >> 64));
    result
}

/// @notice Finds the zero-based index of the first one in the binary representation of x.
/// @dev See the note on msb in the "Find First Set" Wikipedia article https://en.wikipedia.org/wiki/Find_first_set
/// @param x The uint256 number for which to find the index of the most significant bit.
/// @return msb The index of the most significant bit as an uint256.
pub(crate) fn most_significant_bit(mut x: U256) -> U256 {
    let mut msb = U256::ZERO;
    let two = U256::from(2u128);

    if x >= two.pow(128) {
        x >>= 128;
        msb += 128;
    }
    if x >= two.pow(64) {
        x >>= 64;
        msb += 64;
    }
    if x >= two.pow(32) {
        x >>= 32;
        msb += 32;
    }
    if x >= two.pow(16) {
        x >>= 16;
        msb += 16;
    }
    if x >= two.pow(8) {
        x >>= 8;
        msb += 8;
    }
    if x >= two.pow(4) {
        x >>= 4;
        msb += 4;
    }
    if x >= two.pow(2) {
        x >>= 2;
        msb += 2;
    }
    if x >= two.pow(1) {
        // No need to shift x any more.
        msb += 1;
    }
    msb
}

/// @notice Calculates floor(x*y÷denominator) with full precision.
///
/// @dev Credit to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv.
///
/// Requirements:
/// - The denominator cannot be zero.
/// - The result must fit within uint256.
///
/// Caveats:
/// - This function does not work with fixed-point numbers.
///
/// @param x The multiplicand as an uint256.
/// @param y The multiplier as an uint256.
/// @param denominator The divisor as an uint256.
/// @return result The result as an uint256.
pub fn muldiv(x: U256, y: U256, mut denominator: U256) -> StdResult<U256> {
    // 512-bit multiply [prod1 prod0] = x * y. Compute the product mod 2^256 and mod 2^256 - 1, then use
    // use the Chinese Remainder Theorem to reconstruct the 512 bit result. The result is stored in two 256
    // variables such that product = prod1 * 2^256 + prod0.
    let mut prod0: U256; // Least significant 256 bits of the product
    let mut prod1: U256; // Most significant 256 bits of the product
    let result: U256;

    let mm = mulmod(x, y, U256::ZERO.not());
    prod0 = mul(x, y);
    prod1 = u_sub(u_sub(mm, prod0), lt(mm, prod0));

    // Handle non-overflow cases, 256 by 256 division.
    if (prod1 == 0) {
        result = prod0 / denominator;
        return Ok(result);
    }

    // Make sure the result is less than 2^256. Also prevents denominator == 0.
    if (prod1 >= denominator) {
        return Err(StdError::Overflow {
            source: OverflowError {
                operation: OverflowOperation::Mul,
                operand1: prod1.to_string(),
                operand2: denominator.to_string(),
            },
        });
    }

    ///////////////////////////////////////////////
    // 512 by 256 division.
    ///////////////////////////////////////////////

    // Make division exact by u_subtracting the remainder from [prod1 prod0].
    // Compute remainder using mulmod.
    let remainder = mulmod(x, y, denominator);

    // u_subtract 256 bit number from 512 bit number.
    prod1 = u_sub(prod1, gt(remainder, prod0));
    prod0 = u_sub(prod0, remainder);

    // Factor powers of two out of denominator and compute largest power of two divisor of denominator. Always >= 1.
    // See https://cs.stackexchange.com/q/138556/92363.
    // Does not overflow because the denominator cannot be zero at this stage in the function.
    let mut lpotdod = denominator & (denominator.not() + 1);
    // Divide denominator by lpotdod.
    denominator = div(denominator, lpotdod);

    // Divide [prod1 prod0] by lpotdod.
    prod0 = div(prod0, lpotdod);

    // Flip lpotdod such that it is 2^256 / lpotdod. If lpotdod is zero, then it becomes one.
    lpotdod = add(div(u_sub(U256::ZERO, lpotdod), lpotdod), U256::ONE);

    // Shift in bits from prod1 into prod0.
    prod0 |= prod1 * lpotdod;

    // Invert denominator mod 2^256. Now that denominator is an odd number, it has an inverse modulo 2^256 such
    // that denominator * inv = 1 mod 2^256. Compute the inverse by starting with a seed that is correct for
    // four bits. That is, denominator * inv = 1 mod 2^4.
    let mut inverse = (3 * denominator) ^ 2;

    // Use the Newton-Raphson iteration to improve the precision. Thanks to Hensel's lifting lemma, this also works
    // in modular arithmetic, doubling the correct bits in each step.
    inverse *= 2 - denominator * inverse; // inverse mod 2^8
    inverse *= 2 - denominator * inverse; // inverse mod 2^16
    inverse *= 2 - denominator * inverse; // inverse mod 2^32
    inverse *= 2 - denominator * inverse; // inverse mod 2^64
    inverse *= 2 - denominator * inverse; // inverse mod 2^128
    inverse *= 2 - denominator * inverse; // inverse mod 2^256

    // Because the division is now exact we can divide by multiplying with the modular inverse of denominator.
    // This will give us the correct result modulo 2^256. Since the preconditions guarantee that the outcome is
    // less than 2^256, this is the final result. We don't need to compute the high bits of the result and prod1
    // is no longer required.
    result = prod0 * inverse;
    Ok(result)
}

    /// Gets the result of 10^x in constant time. Used for precision calculations (i.e. normalizing different token amounts
    /// based off their decimals).
    ///
    /// @param x - integer between 0 and 18
    ///
    /// @return result The common logarithm as an unsigned 60.18-decimal fixed-point number.
    pub fn e10(x: u32) -> U256 {
        // Note that the "mul" in this block is the assembly multiplication operation, not the "mul" function defined
        // in this contract.
        // prettier-ignore
            match x {
                0 => QUINTILLIONTH,
                1 => HUN_QUADTH,
                2 => TEN_QUADTH,
                3 => QUADTH,
                4 => HUN_TRILTH,
                5 => TEN_TRILTH,
                6 => TRILTH,
                7 => HUN_BILTH,
                8 => TEN_BILTH,
                9 => BILTH,
                10 => HUN_MILTH,
                11 => TEN_MILTH,
                12 => MILTH,
                13 => HUN_THOUSANDTH,
                14 => TEN_THOUSANDTH,
                15 => THOUSANDTH,
                16 => HUNDREDTH,
                17 => TENTH,
                18 => ONE,
                19 => TEN,
                _ => panic!("Not using this correctly :|"),
            }
        }

