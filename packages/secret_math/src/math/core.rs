//! Common mathematical functions used in ud60x18 and sd59x18. Note that this shared library does not always assume the unsigned 60.18-decimal fixed-point representation. When it does not, it is explicitly mentioned in the documentation.
//! Forks methods from here - https://github.com/paulrberg/prb-math/blob/main/contracts/PRBMath.sol.
use super::asm::*;
pub use super::tens::exp10;
use cosmwasm_std::{OverflowError, OverflowOperation, StdError, StdResult};
use std::ops::Not;

use ethnum::U256;
const SCALE: U256 = U256::new(1_000_000_000_000_000_000u128);
/// Largest power of two divisor of SCALE.
const SCALE_LPOTD: U256 = U256::new(262144u128);

/// SCALE inverted mod 2^256.
const SCALE_INVERSE: U256 = U256::from_words(
    229681740086561209518615317264092320238,
    298919117238935307856972083127780443753,
);

/// Finds whether or not some Uint256 is odd.
pub fn is_odd(x: U256) -> bool {
    x & 1 == 1
}

pub fn checked_add(x: U256, y: U256) -> StdResult<U256> {
    let (a, b) = x.overflowing_add(y);
    if b {
        Err(StdError::Overflow {
            source: OverflowError::new(OverflowOperation::Add, x, y),
        })
    } else {
        Ok(a)
    }
}

pub fn checked_sub(x: U256, y: U256) -> StdResult<U256> {
    if y > x {
        Err(StdError::Overflow {
            source: OverflowError::new(OverflowOperation::Sub, x, y),
        })
    } else {
        Ok(x - y)
    }
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
    if x > y {
        x - y
    } else {
        y - x
    }
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
    if x == 0 {
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
    result >>= U256::new(191u128) - (x >> 64);
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
/// - Applies bankers rounding on the last place to smooth out errors over time.
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
    if prod1 == 0 {
        result = prod0 / denominator;
        return Ok(result);
    }

    // Make sure the result is less than 2^256. Also prevents denominator == 0.
    if prod1 >= denominator {
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
    Ok(bankers_round(result, 1))
}

/// @notice Calculates floor(x*y÷1e18) with full precision.
///
/// @dev Variant of "mulDiv" with constant folding, i.e. in which the denominator is always 1e18. Before returning the
/// final result, we add 1 if (x * y) % SCALE >= HALF_SCALE. Without this, 6.6e-19 would be truncated to 0 instead of
/// being rounded to 1e-18.  See "Listing 6" and text above it at https://accu.org/index.php/journals/1717.
///
/// Requirements:
/// - The result must fit within uint256.
///
/// Caveats:
/// - The body is purposely left uncommented; see the NatSpec comments in "PRBMath.mulDiv" to understand how this works.
/// - It is assumed that the result can never be type(uint256).max when x and y solve the following two equations:
///     1. x * y = type(uint256).max * SCALE
///     2. (x * y) % SCALE >= SCALE / 2
///
/// @param x The multiplicand as an unsigned 60.18-decimal fixed-point number.
/// @param y The multiplier as an unsigned 60.18-decimal fixed-point number.
/// @return result The result as an unsigned 60.18-decimal fixed-point number.
pub fn muldiv_fp(x: U256, y: U256) -> StdResult<U256> {
    let mm = mulmod(x, y, !U256::ZERO);
    let prod0 = mul(x, y);
    let prod1 = u_sub(u_sub(mm, prod0), lt(mm, prod0));

    if prod1 >= SCALE {
        return Err(StdError::generic_err(format!(
            "PRBMath__MulDivFixedPointOverflow {}",
            prod1
        )));
    }

    let remainder = mulmod(x, y, SCALE);
    let round_up_unit = gt(remainder, U256::new(499999999999999999u128));

    if prod1 == 0 {
        let result = (prod0 / SCALE) + round_up_unit;
        return Ok(result);
    }

    Ok(bankers_round(
        add(
            mul(
                or(
                    div(u_sub(prod0, remainder), SCALE_LPOTD),
                    mul(
                        u_sub(prod1, gt(remainder, prod0)),
                        add(div(u_sub(U256::ZERO, SCALE_LPOTD), SCALE_LPOTD), U256::ONE),
                    ),
                ),
                SCALE_INVERSE,
            ),
            round_up_unit,
        ),
        1,
    ))
}

/// Performs bankers rounding using the nth digit.
pub fn bankers_round(x: U256, digit: u8) -> U256 {
    let n = nth_digit(x, digit);
    let round_up = match n {
        5 => {
            let next_digit = nth_digit(x, digit + 1);
            // Checks if the next digit is the nearest even integer.
            next_digit % 2 != 0
        }
        _ => n > 5,
    };
    let precision = exp10(digit as u16);

    (x + if round_up { precision } else { U256::ZERO }) / precision * precision
}

/// Where x is a positive integer. Supports up to 32 digits.
pub fn nth_digit(x: U256, digit: u8) -> u8 {
    ((x / exp10((digit - 1) as u16)) % 10).as_u8()
}

#[cfg(test)]
mod test {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("99958", 2, 5)]
    #[case("99958", 1, 8)]
    #[case("99958", 5, 9)]
    fn test_nth_digit(#[case] x: U256, #[case] digit: u8, #[case] expected: u8) {
        let n = nth_digit(x, digit);
        assert_eq!(n, expected);
    }

    #[rstest]
    #[case("99958", 2, "100000")]
    #[case("99955", 2, "100000")]
    #[case("99945", 2, "99900")]
    #[case("735", 1, "740")]
    #[case("745", 1, "740")]
    #[case("755", 1, "760")]
    #[case("765", 1, "760")]
    fn test_bankers_rounding(#[case] x: U256, #[case] digit: u8, #[case] expected: U256) {
        assert_eq!(bankers_round(x, digit), expected);
    }

    #[test]
    fn test_const() {
        let int2 = U256::from_str_prefixed(
            "78156646155174841979727994598816262306175212592076161876661508869554232690281",
        )
        .unwrap();
        assert_eq!(SCALE_INVERSE, int2);
    }

    #[test]
    fn test_checked_err() {
        let max = U256::MAX;
        let one = U256::ONE;
        assert!(checked_add(max, one).is_err());
        assert!(checked_sub(one, max).is_err());
    }

    #[rstest]
    #[case("12000", "12000", "12000")]
    #[case("11", "13", "12")]
    fn test_avg(#[case] x: U256, #[case] y: U256, #[case] xavgy: U256) {
        assert_eq!(avg(x, y), xavgy);
    }

    #[rstest]
    #[case("20", "10", "10")]
    #[case("1", "9999", "9998")]
    fn test_abs_diff(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
        assert_eq!(abs_diff(x, y), expected);
    }

    #[rstest]
    #[case("19318389123", "1319320194941", "219031831291", "116362725698")]
    fn test_muldiv(#[case] x: U256, #[case] y: U256, #[case] denom: U256, #[case] expected: U256) {
        assert_eq!(muldiv(x, y, denom).unwrap(), expected);
    }

    #[rstest]
    #[case("12443", "443", "12000", "12886")]
    fn test_checked_ok(#[case] x: U256, #[case] y: U256, #[case] xsuby: U256, #[case] xaddy: U256) {
        assert_eq!(checked_add(x, y).unwrap(), xaddy);
        assert_eq!(checked_sub(x, y).unwrap(), xsuby);
    }

    #[rstest]
    #[case("0", false)]
    #[case("2323421", true)]
    #[case("232323232320", false)]
    fn test_is_odd(#[case] x: U256, #[case] expected: bool) {
        let actual = is_odd(x);
        assert_eq!(actual, expected);
    }
}
