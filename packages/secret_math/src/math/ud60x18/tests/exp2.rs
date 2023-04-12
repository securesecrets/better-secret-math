use ethnum::U256;
use rstest::*;
use crate::{ud60x18::{
    constants::{ E, PI },
    exp2
}, common::exp10};

const MAX_PERMITTED: U256 = U256::new(192_000_000_000_000_000_000u128 - 1u128);

#[test]
fn test_zero() {
    let actual = exp2(U256::ZERO).unwrap();
    assert_eq!(actual, exp10(18));
}

#[test]
fn test_greater_than_max() {
    assert!(exp2(MAX_PERMITTED + exp10(18)).is_err());
}

#[rstest]
#[case(U256::ONE, exp10(18))]
#[case(exp10(3), U256::new(1_000000000000000693u128))]
#[case(exp10(14) * 3212, U256::new(1_249369313012024883u128))]
#[case(exp10(18), exp10(18) * 2)]
#[case(exp10(18) * 2, exp10(18) * 4)]
#[case(E, U256::new(6_580885991017920969u128))]
#[case(PI, U256::new(8_824977827076287621u128))]
#[case(exp10(18) * 4, exp10(18) * 16)]
#[case(exp10(13) * 11_89215, U256::new(3800_964933301542754377u128))]
#[case(exp10(18) * 16, exp10(18) * 65536)]
#[case(exp10(16) * 20_82, U256::new(1851162_354076939434682641u128))]
#[case(exp10(12) * 33_333333, U256::new(10822636909_120553492168423503u128))]
#[case(exp10(18) * 64, exp10(18) * 18_446744073709551616)]
fn test_exp2(#[case] actual: U256, #[case] expected: U256) {
    let actual = exp2(actual).unwrap();
    assert_eq!(actual, expected);
}