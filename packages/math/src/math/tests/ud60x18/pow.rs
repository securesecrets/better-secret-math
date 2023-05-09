use super::*;
use crate::ud60x18::{
    pow, E, PI,
};

const MAX_PERMITTED: U256 = U256::from_words(0x0DE0B6B3A763FFFFFFFFFFFFFFFFFFFF, 0xffffffffffffffffffffffffffffffff);

#[test]
fn test_pow_base_and_exp_zero() {
    let actual = pow(U256::ZERO, U256::ZERO).unwrap();
    assert_eq!(actual, exp10(18));
}

#[rstest]
#[case(U256::ZERO, exp10(18), U256::ZERO)]
#[case(U256::ZERO, E, U256::ZERO)]
#[case(U256::ZERO, PI, U256::ZERO)]
fn test_pow_base_zero_exp_not_zero(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    assert_eq!(pow(x, y).unwrap(), expected);
}

#[test]
fn test_base_less_than_one_error() {
    assert!(pow(exp10(18) - U256::ONE, PI).is_err());
}

#[rstest]
#[case(exp10(18),  exp10(18))]
#[case(E,  exp10(18))]
#[case(PI,  exp10(18))]
fn test_exp_zero(#[case] x: U256, #[case] expected: U256) {
    assert_eq!(pow(x, U256::ZERO).unwrap(), expected);
}

#[rstest]
#[case(exp10(18))]
#[case(E)]
#[case(PI)]
fn test_exp_one(#[case] x: U256) {
    assert_eq!(pow(x, exp10(18)).unwrap(), x);
}

#[test]
fn test_exp_greater_than_max_permitted() {
    assert!(pow(MAX_PERMITTED + U256::ONE, exp10(18) + U256::ONE).is_err());
}

#[rstest]
#[case(exp10(18), 2 * exp10(18), exp10(18))]
#[case(exp10(18), PI, exp10(18))]
#[case(2 * exp10(18), 3 * exp10(18) / 2, U256::new(2_828427124746190097))]
#[case(E, 1_66976 * exp10(13), U256::new(5_310893029888037560))]
#[case(E, E, U256::new(15_154262241479263793))]
#[case(PI, PI, U256::new(36_462159607207910473))]
#[case(11 * exp10(18), 285 * exp10(17), U256::from_words(0x53C746CA, 0xB1E16FC03F12E7D3B3A11766CCE3D17F))]
#[case(32_15 * exp10(16), 23_99 * exp10(16), U256::from_words(0xEFF20459D7CA9, 0xC2708EF303EB4156D71D9DE63EBEAC6D))]
#[case(406 * exp10(18), exp10(16) * 25, U256::new(4_488812947719016318))]
#[case(1729 * exp10(18), exp10(16) * 98, U256::new(1489_495149922256917866))]
#[case(33441 * exp10(18), 2_1891 * exp10(14), U256::new(8018621589_681923269491820156))]
#[case(U256::new(340282366920938463463374607431768211455) * exp10(18), exp10(18) + U256::ONE, U256::from_words(0xDE0B6B3A7640047, 0x650BEA3C25747159DBA58E859AF5DD9D))]
#[case(MAX_PERMITTED, exp10(18) - U256::ONE, U256::from_words(0xDE0B6B3A763FF6A373AF8903E173F01, 0x79D866D0B6D0DFCBFFF816EF566C0000))]
fn test_pow(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    assert_eq!(pow(x, y).unwrap(), expected);
}