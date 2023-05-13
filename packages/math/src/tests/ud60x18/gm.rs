use ethnum::U256;
use rstest::*;
use crate::{ud60x18::{
    constants::{ PI, SQRT_MAX_UD60X18, E, MAX_WHOLE_UD60X18, MAX_UD60X18 },
    gm
}, common::exp10};

const SQRT_MAX_UINT256: U256 = U256::new(340282366920938463463374607431768211455u128);

#[rstest]
#[case(U256::ZERO, PI, U256::ZERO)]
#[case(PI, U256::ZERO, U256::ZERO)]
fn test_edge(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    assert_eq!(gm(x, y).unwrap(), expected);
}

#[test]
fn test_error() {
    assert!(gm(SQRT_MAX_UD60X18 + exp10(18), SQRT_MAX_UD60X18 + exp10(18)).is_err());
}

#[rstest]
#[case(exp10(18), exp10(18), exp10(18))]
#[case(exp10(18), 4 * exp10(18), 2 * exp10(18))]
#[case(2 * exp10(18), 8 * exp10(18), 4 * exp10(18))]
#[case(E, 89_01 * exp10(16), U256::new(15_554879155787087514))]
#[case(PI, 8_2 * exp10(17), U256::new(5_075535416036056441))]
#[case(32_247 * exp10(16), 67_477 * exp10(16), U256::new(466_468736251423392217))]
#[case(24_048 * exp10(17), 7899_210662 * exp10(12), U256::new(4358_442588812843362311))]
#[case(SQRT_MAX_UINT256, SQRT_MAX_UINT256, SQRT_MAX_UINT256)]
#[case(MAX_WHOLE_UD60X18, U256::ONE, SQRT_MAX_UINT256)]
#[case(MAX_UD60X18, U256::ONE, SQRT_MAX_UINT256)]
fn test_gm(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    assert_eq!(gm(x, y).unwrap(), expected);
}