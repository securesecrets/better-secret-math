use ethnum::U256;
use rstest::*;
use crate::{ud60x18::{
    constants::{ E, PI, MAX_WHOLE_UD60X18, MAX_UD60X18 },
    log2
}, common::exp10};

#[test]
fn test_too_small() {
    assert!(log2(exp10(18) - 1).is_err());
}

#[rstest]
#[case(exp10(18), U256::ZERO)]
#[case(2 * exp10(18), exp10(18))]
#[case(4 * exp10(18), exp10(18) * 2)]
#[case(8 * exp10(18), exp10(18) * 3)]
#[case(16 * exp10(18), exp10(18) * 4)]
#[case(18446744073709551616 * exp10(18), exp10(18) * 64)]
fn test_power_of_two(#[case] actual: U256, #[case] expected: U256) {
    let actual = log2(actual).unwrap();
    assert_eq!(actual, expected);
}

#[rstest]
#[case(1_125 * exp10(15), U256::new(0_169_925_001_442_312_346))]
#[case(E, U256::new(1_442_695_040_888_963_394))]
#[case(PI, U256::new(1_651_496_129_472_318_782))]
#[case(exp10(24), U256::new(19_931_568_569_324_174_075))]
#[case(MAX_WHOLE_UD60X18, U256::new(196_205_294_292_027_477_728))]
#[case(MAX_UD60X18, U256::new(196_205_294_292_027_477_728))]
fn test_not_power_of_two(#[case] actual: U256, #[case] expected: U256) {
    let actual = log2(actual).unwrap();
    assert_eq!(actual, expected);
}