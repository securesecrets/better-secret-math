use crate::{
    common::exp10,
    ud60x18::{
        constants::{MAX_UD60X18, MAX_WHOLE_UD60X18, PI},
        floor,
    },
};
use ethnum::U256;
use rstest::*;

#[test]
fn test_exp_zero() {
    let actual = floor(U256::ZERO);
    assert_eq!(actual, U256::ZERO);
}

#[rstest]
#[case(exp10(17), U256::ZERO)]
#[case(exp10(17) * 5, U256::ZERO)]
#[case(exp10(18), exp10(18))]
#[case(1_125 * exp10(15), exp10(18))]
#[case(exp10(18) * 2, exp10(18) * 2)]
#[case(PI, exp10(18) * 3)]
#[case(4_2 * exp10(17), exp10(18) * 4)]
#[case(exp10(24), exp10(24))]
#[case(MAX_WHOLE_UD60X18, MAX_WHOLE_UD60X18)]
#[case(MAX_UD60X18, MAX_WHOLE_UD60X18)]
fn test_exp(#[case] actual: U256, #[case] expected: U256) {
    let actual = floor(actual);
    assert_eq!(actual, expected);
}
