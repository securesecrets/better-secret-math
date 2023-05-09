use super::*;
use crate::ud60x18::{
    powu, E, MAX_UD60X18, MAX_WHOLE_UD60X18, PI, SQRT_MAX_UD60X18,
};

const MAX_PERMITTED: U256 = U256::from_words(0x0DE0B6B3A763FFFFFFFFFFFFFFFFFFFF, 0xffffffffffffffffffffffffffffffff);

#[test]
fn test_pow_base_and_exp_zero() {
    let actual = powu(U256::ZERO, U256::ZERO).unwrap();
    assert_eq!(actual, exp10(18));
}

#[rstest]
#[case(U256::new(1))]
#[case(U256::new(2))]
#[case(U256::new(3))]
fn test_pow_base_zero_exp_not_zero(#[case] x: U256) {
    assert_eq!(powu(U256::ZERO, x).unwrap(), U256::ZERO);
}


#[rstest]
#[case(exp10(18),  exp10(18))]
#[case(PI,  exp10(18))]
#[case(MAX_UD60X18 - U256::ONE,  exp10(18))]
fn test_exp_zero(#[case] x: U256, #[case] expected: U256) {
    assert_eq!(powu(x, U256::ZERO).unwrap(), expected);
}

#[test]
fn test_overflow() {
    assert!(powu(MAX_WHOLE_UD60X18, U256::new(2)).is_err());
}

#[rstest]
#[case(exp10(15), U256::new(3), exp10(9))]
#[case(exp10(17), U256::new(2), exp10(16))]
#[case(exp10(18), U256::new(1), exp10(18))]
#[case(2 * exp10(18), U256::new(5), 32 * exp10(18))]
#[case(2 * exp10(18), U256::new(100), 1267650600228_229401496703205376 * exp10(18))]
#[case(E, U256::new(2), U256::new(7_389056098930650225))]
#[case(PI, U256::new(3), U256::new(31_006276680299820158))]
#[case(5_491 * exp10(15), U256::new(19), U256::new(113077820843204_476043049664958463))]
#[case(exp10(20), U256::new(4), exp10(26))]
#[case(47_877 * exp10(16), U256::new(20), U256::from_words(0x3A052F18B3FC80FECE7D840DD81B, 0xEF1E8BCE3C4B6374029648608DDDF3AC))]
#[case(6_452_166 * exp10(15), U256::new(7), U256::from_words(0x14DFE9, 0x874D69C88BA34ADFE2FE104953FA09D0))]
#[case(exp10(24), U256::new(3), exp10(36))]
#[case(U256::new(38685626227668133590597631999999999999), U256::new(3), U256::from_words(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF2, 0xCE4CDF1D90732CDEF20133F625121A7D))]
#[case(SQRT_MAX_UD60X18, U256::new(2), U256::from_words(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFF768FA0BEC94B5A68CE97F5770))]
#[case(MAX_WHOLE_UD60X18, U256::new(1), MAX_WHOLE_UD60X18)]
#[case(MAX_UD60X18, U256::new(1), MAX_UD60X18)]
fn test_pow(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    assert_eq!(powu(x, y).unwrap(), expected);
}