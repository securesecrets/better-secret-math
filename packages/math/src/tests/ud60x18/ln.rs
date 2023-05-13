use ethnum::U256;
use rstest::*;
use crate::{ud60x18::{
    constants::{ E, PI, MAX_WHOLE_UD60X18, MAX_UD60X18 },
    ln
}, common::exp10};

#[test]
fn test_too_small() {
    assert!(ln(exp10(18) - 1).is_err());
}

#[rstest]
#[case(exp10(18), U256::ZERO)]
#[case(1_125 * exp10(15), U256::new(117783035656383442u128))]
#[case(exp10(18) * 2, U256::new(693147180559945309u128))]
#[case(E, U256::new(999999999999999990u128))]
#[case(PI, U256::new(1144729885849400163u128))]
#[case(4 * exp10(18), U256::new(1386294361119890619u128))]
#[case(8 * exp10(18), U256::new(2079441541679835928u128))]
#[case(exp10(24), U256::new(13815510557964274099u128))]
#[case(MAX_WHOLE_UD60X18, U256::new(135_999146549453176925u128))]
#[case(MAX_UD60X18, U256::new(135_999146549453176925u128))]
fn test_ln(#[case] actual: U256, #[case] expected: U256) {
    let actual = ln(actual).unwrap();
    assert_eq!(actual, expected);
}