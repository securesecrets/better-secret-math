use crate::{
    common::exp10,
    ud60x18::{
        constants::{E, MAX_UD60X18, MAX_WHOLE_UD60X18, PI},
        log10,
    },
};
use ethnum::U256;
use rstest::*;

#[test]
fn test_too_small() {
    assert!(log10(exp10(18) - 1).is_err());
}

#[rstest]
#[case(exp10(18), U256::ZERO)]
#[case(exp10(19), exp10(18))]
#[case(exp10(20), exp10(18) * 2)]
#[case(exp10(24), exp10(18) * 6)]
#[case(exp10(67), exp10(18) * 49)]
#[case(exp10(75), exp10(18) * 57)]
#[case(exp10(76), exp10(18) * 58)]
fn test_power_of_tens(#[case] actual: U256, #[case] expected: U256) {
    let actual = log10(actual).unwrap();
    assert_eq!(actual, expected);
}

#[rstest]
#[case(E, U256::new(0_434294481903251823))]
#[case(PI, U256::new(0_497149872694133849))]
#[case(4 * exp10(18), U256::new(0_602059991327962390))]
#[case(16 * exp10(18), U256::new(1_204119982655924781))]
#[case(32 * exp10(18), U256::new(1_505149978319905976))]
#[case(42_12 * exp10(16), U256::new(1_624488362513448905))]
#[case(1010_892143 * exp10(12), U256::new(3_004704821071980110))]
#[case(4_409_341_881 * exp10(14), U256::new(5_644373773418177966))]
#[case(1_000_000_000_000_000_000_000_000_000_001 * exp10(6), U256::new(17_999_999_999_999_999_999))]
#[case(MAX_WHOLE_UD60X18, U256::new(59_063678889979185987))]
#[case(MAX_UD60X18, U256::new(59_063678889979185987))]
fn test_not_power_of_tens(#[case] actual: U256, #[case] expected: U256) {
    let actual = log10(actual).unwrap();
    assert_eq!(actual, expected);
}
