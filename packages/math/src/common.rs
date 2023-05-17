//! Common mathematical functions used in ud60x18 and sd59x18. Note that this shared library does not always assume the unsigned 60.18-decimal fixed-point representation. When it does not, it is explicitly mentioned in the documentation.
//! Forks methods from here - https://github.com/paulrberg/prb-math/blob/main/contracts/PRBMath.sol.
pub use super::tens::exp10;
use crate::{
    asm::{u256_to_u512, u512_to_u256, Asm},
    ud60x18::constants::*,
};
use cosmwasm_std::{DivideByZeroError, OverflowError, OverflowOperation, StdError, StdResult};
use primitive_types::U512;
use std::ops::Not;

use ethnum::U256;

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

/// Takes the absolute difference of two unsigned ints.
pub fn abs_diff(x: U256, y: U256) -> U256 {
    if x > y {
        x - y
    } else {
        y - x
    }
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
    let precision = exp10(digit);

    (x + if round_up { precision } else { U256::ZERO }) / precision * precision
}

/// Where x is a positive integer. Supports up to 32 digits.
pub fn nth_digit(x: U256, digit: u8) -> u8 {
    ((x / exp10(digit - 1)) % 10).as_u8()
}

/// @notice Finds the zero-based index of the first one in the binary representation of x.
/// @dev See the note on msb in the "Find First Set" Wikipedia article https://en.wikipedia.org/wiki/Find_first_set
/// @param x The uint256 number for which to find the index of the most significant bit.
/// @return msb The index of the most significant bit as an uint256.
pub fn msb(mut x: U256) -> U256 {
    let mut result = U256::ZERO;

    let mut factor = Asm::shl(
        U256::new(7u128),
        Asm::gt(x, U256::new(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
    );
    x >>= factor;
    result = Asm::or(result, factor);

    factor = Asm::shl(U256::new(6u128), Asm::gt(x, U256::new(0xFFFFFFFFFFFFFFFF)));
    x >>= factor;
    result = Asm::or(result, factor);

    factor = Asm::shl(U256::new(5u128), Asm::gt(x, U256::new(0xFFFFFFFF)));
    x >>= factor;
    result = Asm::or(result, factor);

    factor = Asm::shl(U256::new(4u128), Asm::gt(x, U256::new(0xFFFF)));
    x >>= factor;
    result = Asm::or(result, factor);

    factor = Asm::shl(U256::new(3u128), Asm::gt(x, U256::new(0xFF)));
    x >>= factor;
    result = Asm::or(result, factor);

    factor = Asm::shl(U256::new(2u128), Asm::gt(x, U256::new(0xF)));
    x >>= factor;
    result = Asm::or(result, factor);

    factor = Asm::shl(U256::new(1u128), Asm::gt(x, U256::new(0x3)));
    x >>= factor;
    result = Asm::or(result, factor);

    factor = Asm::gt(x, U256::new(0x1));
    Asm::or(result, factor)
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
pub fn muldiv(x: U256, y: U256, denominator: U256) -> StdResult<U256> {
    if denominator == 0 {
        return Err(StdError::DivideByZero {
            source: DivideByZeroError {
                operand: "better_secret_math::muldiv".to_string(),
            },
        });
    }

    // 512-bit multiply [prod1 prod0] = x * y. Compute the product mod 2^256 and mod 2^256 - 1, then use
    // use the Chinese Remainder Theorem to reconstruct the 512 bit result. The result is stored in two 256
    // variables such that product = prod1 * 2^256 + prod0.
    // Least significant 256 bits of the product
    // Most significant 256 bits of the product
    let result: U256;

    let mm = Asm::mulmod(x, y, U256::ZERO.not());
    let prod0: U256 = Asm::mul(x, y);
    let prod1: U256 = Asm::u_sub(Asm::u_sub(mm, prod0), Asm::lt(mm, prod0));

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

    let lo = prod0.to_le_bytes();
    let hi = prod1.to_le_bytes();
    let lo_hi = [lo, hi].concat();
    let xy = U512::from_little_endian(&lo_hi);
    let denominator = u256_to_u512(&denominator);
    result = u512_to_u256(&xy / denominator);
    Ok(result)
}

/// @notice Calculates floor(x*y÷1e18) with full precision.
///
/// @dev Variant of "mulDiv" with constant folding, i.e. in which the denominator is always 1e18. Before returning the
/// final result, we add 1 if (x * y) % UNIT >= HALF_UNIT. Without this, 6.6e-19 would be truncated to 0 instead of
/// being rounded to 1e-18.  See "Listing 6" and text above it at https://accu.org/index.php/journals/1717.
///
/// Requirements:
/// - The result must fit within uint256.
///
/// Caveats:
/// - The body is purposely left uncommented; see the NatSpec comments in "PRBMath.mulDiv" to understand how this works.
/// - It is assumed that the result can never be type(uint256).max when x and y solve the following two equations:
///     1. x * y = type(uint256).max * UNIT
///     2. (x * y) % UNIT >= UNIT / 2
///
/// @param x The multiplicand as an unsigned 60.18-decimal fixed-point number.
/// @param y The multiplier as an unsigned 60.18-decimal fixed-point number.
/// @return result The result as an unsigned 60.18-decimal fixed-point number.
pub fn muldiv18(x: U256, y: U256) -> StdResult<U256> {
    let mm = Asm::mulmod(x, y, !U256::ZERO);
    let prod0 = Asm::mul(x, y);
    let prod1 = Asm::u_sub(Asm::u_sub(mm, prod0), Asm::lt(mm, prod0));

    if prod1 >= UNIT {
        return Err(StdError::generic_err(format!(
            "PRBMath__MulDiv18Overflow {}",
            prod1
        )));
    }

    let remainder = Asm::mulmod(x, y, UNIT);

    if prod1 == 0 {
        return Ok(prod0 / UNIT);
    }

    Ok(Asm::mul(
        Asm::or(
            Asm::div(Asm::u_sub(prod0, remainder), UNIT_LPOTD),
            Asm::mul(
                Asm::u_sub(prod1, Asm::gt(remainder, prod0)),
                Asm::add(
                    Asm::div(Asm::u_sub(U256::ZERO, UNIT_LPOTD), UNIT_LPOTD),
                    U256::ONE,
                ),
            ),
        ),
        UNIT_INVERSE,
    ))
}

/// Calculates the binary exponent of x (2^x) using the binary fraction method.
/// Has to use 192.64-bit fixed-point numbers so x is the exponent as an unsigned 192.64-bit fixed-point number.
/// See https://ethereum.stackexchange.com/a/96594/24693.
/// The result is an unsigned 60.18-decimal fixed-point number.
pub fn exp2(x: U256) -> U256 {
    // Start from 0.5 in the 192.64-bit fixed-point format.
    // Guaranteed not to panic.
    let mut result =
        U256::from_str_hex("0x800000000000000000000000000000000000000000000000").unwrap();

    // Multiply the result by root(2, 2^-i) when the bit at position i is 1. None of the intermediary results overflows
    // because the initial result is 2^191 and all magic factors are less than 2^65.
    if x & 0xFF00000000000000 > 0 {
        if x & 0x8000000000000000 > 0 {
            result = (result * 0x16A09E667F3BCC909) >> 64;
        }
        if x & 0x4000000000000000 > 0 {
            result = (result * 0x1306FE0A31B7152DF) >> 64;
        }
        if x & 0x2000000000000000 > 0 {
            result = (result * 0x1172B83C7D517ADCE) >> 64;
        }
        if x & 0x1000000000000000 > 0 {
            result = (result * 0x10B5586CF9890F62A) >> 64;
        }
        if x & 0x800000000000000 > 0 {
            result = (result * 0x1059B0D31585743AE) >> 64;
        }
        if x & 0x400000000000000 > 0 {
            result = (result * 0x102C9A3E778060EE7) >> 64;
        }
        if x & 0x200000000000000 > 0 {
            result = (result * 0x10163DA9FB33356D8) >> 64;
        }
        if x & 0x100000000000000 > 0 {
            result = (result * 0x100B1AFA5ABCBED61) >> 64;
        }
    }

    if x & 0xFF000000000000 > 0 {
        if x & 0x80000000000000 > 0 {
            result = (result * 0x10058C86DA1C09EA2) >> 64;
        }
        if x & 0x40000000000000 > 0 {
            result = (result * 0x1002C605E2E8CEC50) >> 64;
        }
        if x & 0x20000000000000 > 0 {
            result = (result * 0x100162F3904051FA1) >> 64;
        }
        if x & 0x10000000000000 > 0 {
            result = (result * 0x1000B175EFFDC76BA) >> 64;
        }
        if x & 0x8000000000000 > 0 {
            result = (result * 0x100058BA01FB9F96D) >> 64;
        }
        if x & 0x4000000000000 > 0 {
            result = (result * 0x10002C5CC37DA9492) >> 64;
        }
        if x & 0x2000000000000 > 0 {
            result = (result * 0x1000162E525EE0547) >> 64;
        }
        if x & 0x1000000000000 > 0 {
            result = (result * 0x10000B17255775C04) >> 64;
        }
    }

    if x & 0xFF0000000000 > 0 {
        if x & 0x800000000000 > 0 {
            result = (result * 0x1000058B91B5BC9AE) >> 64;
        }
        if x & 0x400000000000 > 0 {
            result = (result * 0x100002C5C89D5EC6D) >> 64;
        }
        if x & 0x200000000000 > 0 {
            result = (result * 0x10000162E43F4F831) >> 64;
        }
        if x & 0x100000000000 > 0 {
            result = (result * 0x100000B1721BCFC9A) >> 64;
        }
        if x & 0x80000000000 > 0 {
            result = (result * 0x10000058B90CF1E6E) >> 64;
        }
        if x & 0x40000000000 > 0 {
            result = (result * 0x1000002C5C863B73F) >> 64;
        }
        if x & 0x20000000000 > 0 {
            result = (result * 0x100000162E430E5A2) >> 64;
        }
        if x & 0x10000000000 > 0 {
            result = (result * 0x1000000B172183551) >> 64;
        }
    }

    if x & 0xFF00000000 > 0 {
        if x & 0x8000000000 > 0 {
            result = (result * 0x100000058B90C0B49) >> 64;
        }
        if x & 0x4000000000 > 0 {
            result = (result * 0x10000002C5C8601CC) >> 64;
        }
        if x & 0x2000000000 > 0 {
            result = (result * 0x1000000162E42FFF0) >> 64;
        }
        if x & 0x1000000000 > 0 {
            result = (result * 0x10000000B17217FBB) >> 64;
        }
        if x & 0x800000000 > 0 {
            result = (result * 0x1000000058B90BFCE) >> 64;
        }
        if x & 0x400000000 > 0 {
            result = (result * 0x100000002C5C85FE3) >> 64;
        }
        if x & 0x200000000 > 0 {
            result = (result * 0x10000000162E42FF1) >> 64;
        }
        if x & 0x100000000 > 0 {
            result = (result * 0x100000000B17217F8) >> 64;
        }
    }

    if x & 0xFF000000 > 0 {
        if x & 0x80000000 > 0 {
            result = (result * 0x10000000058B90BFC) >> 64;
        }
        if x & 0x40000000 > 0 {
            result = (result * 0x1000000002C5C85FE) >> 64;
        }
        if x & 0x20000000 > 0 {
            result = (result * 0x100000000162E42FF) >> 64;
        }
        if x & 0x10000000 > 0 {
            result = (result * 0x1000000000B17217F) >> 64;
        }
        if x & 0x8000000 > 0 {
            result = (result * 0x100000000058B90C0) >> 64;
        }
        if x & 0x4000000 > 0 {
            result = (result * 0x10000000002C5C860) >> 64;
        }
        if x & 0x2000000 > 0 {
            result = (result * 0x1000000000162E430) >> 64;
        }
        if x & 0x1000000 > 0 {
            result = (result * 0x10000000000B17218) >> 64;
        }
    }

    if x & 0xFF0000 > 0 {
        if x & 0x800000 > 0 {
            result = (result * 0x1000000000058B90C) >> 64;
        }
        if x & 0x400000 > 0 {
            result = (result * 0x100000000002C5C86) >> 64;
        }
        if x & 0x200000 > 0 {
            result = (result * 0x10000000000162E43) >> 64;
        }
        if x & 0x100000 > 0 {
            result = (result * 0x100000000000B1721) >> 64;
        }
        if x & 0x80000 > 0 {
            result = (result * 0x10000000000058B91) >> 64;
        }
        if x & 0x40000 > 0 {
            result = (result * 0x1000000000002C5C8) >> 64;
        }
        if x & 0x20000 > 0 {
            result = (result * 0x100000000000162E4) >> 64;
        }
        if x & 0x10000 > 0 {
            result = (result * 0x1000000000000B172) >> 64;
        }
    }

    if x & 0xFF00 > 0 {
        if x & 0x8000 > 0 {
            result = (result * 0x100000000000058B9) >> 64;
        }
        if x & 0x4000 > 0 {
            result = (result * 0x10000000000002C5D) >> 64;
        }
        if x & 0x2000 > 0 {
            result = (result * 0x1000000000000162E) >> 64;
        }
        if x & 0x1000 > 0 {
            result = (result * 0x10000000000000B17) >> 64;
        }
        if x & 0x800 > 0 {
            result = (result * 0x1000000000000058C) >> 64;
        }
        if x & 0x400 > 0 {
            result = (result * 0x100000000000002C6) >> 64;
        }
        if x & 0x200 > 0 {
            result = (result * 0x10000000000000163) >> 64;
        }
        if x & 0x100 > 0 {
            result = (result * 0x100000000000000B1) >> 64;
        }
    }

    if x & 0xFF > 0 {
        if x & 0x80 > 0 {
            result = (result * 0x10000000000000059) >> 64;
        }
        if x & 0x40 > 0 {
            result = (result * 0x1000000000000002C) >> 64;
        }
        if x & 0x20 > 0 {
            result = (result * 0x10000000000000016) >> 64;
        }
        if x & 0x10 > 0 {
            result = (result * 0x1000000000000000B) >> 64;
        }
        if x & 0x8 > 0 {
            result = (result * 0x10000000000000006) >> 64;
        }
        if x & 0x4 > 0 {
            result = (result * 0x10000000000000003) >> 64;
        }
        if x & 0x2 > 0 {
            result = (result * 0x10000000000000001) >> 64;
        }
        if x & 0x1 > 0 {
            result = (result * 0x10000000000000001) >> 64;
        }
    }

    // We're doing two things at the same time:
    //
    //   1. Multiply the result by 2^n + 1, where "2^n" is the integer part and the one is added to account for
    //      the fact that we initially set the result to 0.5. This is accomplished by subtracting from 191
    //      rather than 192.
    //   2. Convert the result to the unsigned 60.18-decimal fixed-point format.
    //
    // This works because 2^(191-ip) = 2^ip / 2^191, where "ip" is the integer part "2^n".
    result *= UNIT;
    result >>= U256::new(191u128) - (x >> 64);
    result
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

    // For our first guess, we get the biggest power of 2 which is smaller than the square root of x.
    //
    // We know that the "msb" (most significant bit) of x is a power of 2 such that we have:
    //
    // $$
    // msb(x) <= x <= 2*msb(x)$
    // $$
    //
    // We write $msb(x)$ as $2^k$ and we get:
    //
    // $$
    // k = log_2(x)
    // $$
    //
    // Thus we can write the initial inequality as:
    //
    // $$
    // 2^{log_2(x)} <= x <= 2*2^{log_2(x)+1} \\
    // sqrt(2^k) <= sqrt(x) < sqrt(2^{k+1}) \\
    // 2^{k/2} <= sqrt(x) < 2^{(k+1)/2} <= 2^{(k/2)+1}
    // $$
    //
    // Consequently, $2^{log_2(x) /2}` is a good first approximation of sqrt(x) with at least one correct bit.
    let mut x_aux = x;
    let mut result = U256::ONE;
    // Can't panic.
    if x_aux >= TWO_TO_128 {
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
    if result >= rounded_down_result {
        rounded_down_result
    } else {
        result
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

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
        assert_eq!(UNIT_INVERSE, int2);
    }

    #[test]
    fn test_checked_err() {
        let max = U256::MAX;
        let one = U256::ONE;
        assert!(checked_add(max, one).is_err());
        assert!(checked_sub(one, max).is_err());
    }

    #[rstest]
    #[case("20", "10", "10")]
    #[case("1", "9999", "9998")]
    fn test_abs_diff(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
        assert_eq!(abs_diff(x, y), expected);
    }

    #[rstest]
    #[case("19318389123", "1319320194941", "219031831291", "116362725698")]
    #[case(U256::MAX, U256::MAX, U256::MAX, U256::MAX)]
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

    #[rstest]
    #[case(TWO_TO_128, "340282366920938463463374607431768211456")]
    #[case(SQRT_MAX_UD60X18, "340282366920938463463374607431768211455999999999")]
    #[case(
        MAX_SCALED_UD60X18,
        "115792089237316195423570985008687907853269984665640564039457"
    )]
    #[case(
        TWO_TO_255,
        "57896044618658097711785492504343953926634992332820282019728792003956564819968"
    )]
    fn test_constants(#[case] x: U256, #[case] expected: &str) {
        assert_eq!(x.to_string(), expected);
    }
}
