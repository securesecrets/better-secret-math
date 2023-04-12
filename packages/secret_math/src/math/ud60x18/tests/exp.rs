use ethnum::U256;
use rstest::*;
use crate::{ud60x18::{
    constants::{ E, PI },
    exp
}, common::exp10};

const MAX_PERMITTED: U256 = U256::new(133_084258667509499440u128);

#[test]
fn test_exp_zero() {
    let actual = exp(U256::ZERO).unwrap();
    assert_eq!(actual, exp10(18));
}

#[test]
fn test_greater_than_max() {
    assert!(exp(MAX_PERMITTED + exp10(18)).is_err());
}

#[rstest]
#[case(U256::ONE, exp10(18))]
#[case(exp10(18), U256::new(2_718281828459045234u128))]
#[case(exp10(18) * 2, U256::new(7_389056098930650223u128))]
#[case(E, U256::new(15_154262241479264171u128))]
#[case(exp10(18) * 3, U256::new(20_085536923187667724u128))]
#[case(PI, U256::new(23_140692632779268962u128))]
#[case(exp10(18) * 4, U256::new(54_598150033144239019u128))]
#[case(exp10(13) * U256::new(11_89215u128), U256::new(146115_107851442195738190u128))]
#[case(exp10(18) * 16, U256::new(8886110_520507872601090007u128))]
#[case(exp10(16) * U256::new(20_82u128), U256::new(1101567497_354306722521735975u128))]
#[case(exp10(12) * U256::new(33_333333u128), U256::new(299559147061116_199277615819889397u128))]
fn test_exp(#[case] actual: U256, #[case] expected: U256) {
    let actual = exp(actual).unwrap();
    assert_eq!(actual, expected);
}