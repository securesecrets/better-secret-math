use ethnum::U256;
use rstest::*;
use crate::{ud60x18::{
    constants::{ E, PI, MAX_WHOLE_UD60X18, MAX_UD60X18 },
    frac
}, common::exp10};

#[test]
fn test_zero() {
    let actual = frac(U256::ZERO);
    assert_eq!(actual, U256::ZERO);
}

#[rstest]
#[case(exp10(17), exp10(17))]
#[case(exp10(17) * 5, exp10(17) * 5)]
#[case(exp10(18), U256::ZERO)]
#[case(1_125 * exp10(15), 0_125 * exp10(15))]
#[case(exp10(18) * 2, U256::ZERO)]
#[case(PI, U256::new(0_141592653589793238))]
#[case(4_2 * exp10(17), 0_2 * exp10(17))]
#[case(exp10(24), U256::ZERO)]
#[case(MAX_WHOLE_UD60X18, U256::ZERO)]
#[case(MAX_UD60X18, U256::new(0_584007913129639935))]
fn test_frac(#[case] actual: U256, #[case] expected: U256) {
    let actual = frac(actual);
    assert_eq!(actual, expected);
}