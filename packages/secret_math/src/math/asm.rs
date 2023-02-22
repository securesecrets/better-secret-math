use ethnum::{I256, U256};
use primitive_types::{U128, U512};

pub fn u256_to_u512(u: &U256) -> U512 {
    U512::from_little_endian(&u.to_le_bytes())
}

pub fn u512_to_u256(r: U512) -> U256 {
    if r <= U512::zero() {
        U256::ZERO
    } else if r > U512::from_little_endian(&U256::MAX.to_le_bytes()) {
        U256::MAX
    } else {
        let lo = U128([r.0[0], r.0[1]]).as_u128();
        let hi = U128([r.0[2], r.0[3]]).as_u128();
        U256::from_words(hi, lo)
    }
}

/// Assembly math operations.
/// Mirrors how unchecked arithmetic behaves in Solidity (it uses wrapping arithmetic).
pub struct Asm;

impl Asm {
    /// Computes (x + y) % k where the addition is performed with arbitrary precision and does not wrap around at 2^256.
    pub fn addmod(x: U256, y: U256, k: U256) -> U256 {
        if k == U256::ZERO {
            return U256::ZERO;
        }

        if let Some(z) = x.checked_add(y) {
            return z % k;
        }

        let x = u256_to_u512(&x);
        let y = u256_to_u512(&y);
        let k = u256_to_u512(&k);
        let z = (x + y) % k;
        u512_to_u256(z)
    }

    /// Computes (x * y) % k where the addition is performed with arbitrary precision and does not wrap around at 2^256.
    pub fn mulmod(x: U256, y: U256, k: U256) -> U256 {
        if k == U256::ZERO {
            return U256::ZERO;
        }

        if let Some(z) = x.checked_mul(y) {
            return z % k;
        }

        let x = u256_to_u512(&x);
        let y = u256_to_u512(&y);
        let k = u256_to_u512(&k);
        let z = (x * y) % k;
        u512_to_u256(z)
    }

    /// Compares the first and second operands and returns a value of 1 (true) if the first operand is greater than or equal the second, else a value of 0 (false).
    #[inline]
    pub fn gt(x: U256, y: U256) -> U256 {
        if x > y {
            U256::ONE
        } else {
            U256::ZERO
        }
    }

    /// Compares the first and second operands and returns a value of 0 (false) if the first operand is greater than or equal the second, else a value of 1 (true).
    #[inline]
    pub fn lt(x: U256, y: U256) -> U256 {
        if x < y {
            U256::ONE
        } else {
            U256::ZERO
        }
    }

    #[inline]
    pub fn u_sub(x: U256, y: U256) -> U256 {
        x.wrapping_sub(y)
    }

    #[inline]
    pub fn sub(x: U256, y: U256) -> I256 {
        x.as_i256().wrapping_sub(y.as_i256())
    }

    #[inline]
    pub fn sgt(x: I256, y: I256) -> I256 {
        if x > y {
            I256::ONE
        } else {
            I256::ZERO
        }
    }

    #[inline]
    /// Performs assembly shl instruction which shifts y left by x bits.
    pub fn shl(x: U256, y: U256) -> U256 {
        y << x
    }

    #[inline]
    pub fn or(x: U256, y: U256) -> U256 {
        x | y
    }

    #[inline]
    pub fn mul(x: U256, y: U256) -> U256 {
        x.wrapping_mul(y)
    }

    #[inline]
    pub fn add(x: U256, y: U256) -> U256 {
        x.wrapping_add(y)
    }

    #[inline]
    pub fn div(x: U256, y: U256) -> U256 {
        x.wrapping_div(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ud60x18::constants::TWO_TO_255;
    use rstest::*;

    #[rstest]
    #[case("3", "4", "5", "2")]
    #[case(U256::MAX, "1", U256::MAX, U256::ZERO)]
    #[case(
        TWO_TO_255,
        TWO_TO_255,
        U256::MAX,
        "28948022309329048855892746252171976963317496166410141009864396001978282409984"
    )]
    fn test_mulmod(#[case] x: U256, #[case] y: U256, #[case] z: U256, #[case] expected: U256) {
        assert_eq!(Asm::mulmod(x, y, z), expected);
    }

    #[rstest]
    #[case("3", "4", "5", "2")]
    #[case(U256::MAX, "1", "10", "6")]
    #[case(U256::MAX, U256::MAX, U256::MAX, U256::ZERO)]
    #[case(TWO_TO_255, TWO_TO_255, U256::MAX, "1")]
    fn test_addmod(#[case] x: U256, #[case] y: U256, #[case] z: U256, #[case] expected: U256) {
        assert_eq!(Asm::addmod(x, y, z), expected);
    }

    #[rstest]
    #[case("7", "128")]
    #[case("6", "64")]
    fn test_shl(#[case] x: U256, #[case] expected: U256) {
        assert_eq!(Asm::shl(x, U256::ONE), expected);
    }
}
