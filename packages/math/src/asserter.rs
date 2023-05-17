use cosmwasm_std::{Decimal256, Uint128, Uint256};
use ethnum::U256;

use crate::common::{abs_diff, bankers_round, exp10, muldiv18};
pub struct MathAsserter;

impl MathAsserter {
    /// Assert a and b are within error distance of one another
    /// a, b, and error must be normalized to 10^18
    pub fn close_u128(a: u128, b: u128, error: u128) {
        // Get absolute different of a and b
        let diff = a.abs_diff(b);
        // Ensure diff is within inputted margin of error
        let error_diff = if a < b {
            muldiv18(U256::from(a), exp10(18) + U256::from(error))
                .unwrap()
                .as_u128()
                - a
        } else {
            a - muldiv18(U256::from(a), exp10(18) - U256::from(error))
                .unwrap()
                .as_u128()
        };
        assert!(diff <= error_diff);
    }

    /// Employs bankers rounding on the (x - n)th decimal of actual where x is actual's decimal precision.
    ///
    /// So if x is 18, n is 1, it will perform bankers rounding to the 17th decimal and check if expected and actual are the same afterwards.
    pub fn close_u256(expected: impl Into<U256> + Copy, actual: impl Into<U256> + Copy, n: u8) {
        let actual: U256 = actual.into();
        let expected: U256 = expected.into();
        assert_eq!(expected, bankers_round(actual, n));
    }

    /// Asserts that expected and actual are equal after dividing actual by 10^n.
    pub fn close_trim_u256(
        expected: impl Into<U256> + Copy,
        actual: impl Into<U256> + Copy,
        n: u8,
    ) {
        let actual: U256 = actual.into();
        let expected: U256 = expected.into();
        assert_eq!(expected, actual / exp10((n).into()));
    }

    /// Asserts that expected and actual are equal after dividing actual by 10^(18 - n).
    pub fn close_trim_u256x18(
        expected: impl Into<U256> + Copy,
        actual: impl Into<U256> + Copy,
        n: u8,
    ) {
        let actual: U256 = actual.into();
        let expected: U256 = expected.into();
        assert_eq!(expected, actual / exp10((18 - n).into()));
    }

    // Asserts that expected and actual are within 17 decimal precision of each other using bankers rounding on the actual value.
    pub fn bigint(expected: impl Into<U256> + Copy, actual: impl Into<U256> + Copy) {
        Self::close_u256(expected, actual, 1);
    }

    /// Asserts the actual value is equal to expected after truncating some amount of its decimals.
    pub fn close_uint256(expected: u128, actual: Uint256, decimals: u32) {
        assert_eq!(
            Uint256::from_u128(expected),
            actual / Uint256::from_u128(10u128.pow(decimals))
        );
    }

    /// Asserts the actual value is equal to expected after truncating some amount of its decimals.
    pub fn close_uint128(expected: u128, actual: Uint128, decimals: u32) {
        assert_eq!(
            Uint128::new(expected),
            actual / Uint128::new(10u128.pow(decimals))
        );
    }

    pub fn get_deviation(
        expected: impl Into<U256> + Copy,
        actual: impl Into<U256> + Copy,
    ) -> Decimal256 {
        let expected = expected.into();
        if expected == U256::ZERO {
            return Decimal256::zero();
        }
        let actual = actual.into();
        let diff = abs_diff(expected, actual);
        Decimal256::from_ratio(diff, expected)
    }

    pub fn within_deviation(
        expected: impl Into<U256> + Copy,
        actual: impl Into<U256> + Copy,
        deviation: impl Into<U256> + Copy,
    ) {
        let actual_deviation = Self::get_deviation(expected, actual);
        let deviation: Decimal256 = deviation.into().into();
        assert!(actual_deviation <= deviation);
    }
}

/// Check if a is within some deviation of b.
#[macro_export]
macro_rules! assert_within_precision {
    ($a:expr, $b:expr, $tolerance:expr) => {{
        let a: better_secret_math::U256 = $a.into();
        let b: better_secret_math::U256 = $b.into();
        let tolerance: U256 = $tolerance.into();

        let diff = better_secret_math::common::abs_diff(a, b);
        if b == U256::ZERO && a == U256::ZERO {
            assert!(true);
        } else {
            let actual_deviation = better_secret_math::ud60x18::div(diff, b).unwrap();
            assert!(
                actual_deviation <= tolerance,
                "Expected {:?} to be within {:?} of {:?} (Actual: {:?}",
                a,
                tolerance,
                b,
                actual_deviation
            );
        }
    }};
}
