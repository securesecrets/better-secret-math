use super::*;
use crate::ud60x18::{sqrt, E, PI};

const MAX_PERMITTED: U256 =
    U256::from_words(0x12725DD1D243ABA0E7, 0x5FE645CC4873F9E65AFE688C928E1F21);

#[test]
fn test_zero() {
    assert_eq!(sqrt(U256::ZERO).unwrap(), U256::ZERO);
}

#[test]
fn test_overflow() {
    assert!(sqrt(MAX_PERMITTED + U256::ONE).is_err());
}

#[rstest]
#[case(U256::ONE, exp10(9))]
#[case(exp10(15), U256::new(31622776601683793))]
#[case(exp10(18), exp10(18))]
#[case(exp10(18) * 2, U256::new(1414213562373095048))]
#[case(E, U256::new(1648721270700128146))]
#[case(exp10(18) * 3, U256::new(1732050807568877293))]
#[case(PI, U256::new(1772453850905516027))]
#[case(exp10(18) * 4, exp10(18) * 2)]
#[case(exp10(18) * 16, exp10(18) * 4)]
#[case(exp10(35), U256::new(316227766016837933199889354))]
#[case(
    U256::from_words(0x24, 0xB3C72EE9BBE79DB150B1FB85AE82BCC1),
    U256::new(111754781727598977910452220959)
)]
#[case(
    U256::from_words(0x50D22BFE589, 0x831BDE847D2871FE888CD97600C2B3D3),
    U256::new(43473210166640613973238162807779776)
)]
#[case(exp10(58), exp10(38))]
#[case(exp10(58) * 5, U256::new(223606797749978969640917366873127623544))]
#[case(MAX_PERMITTED, U256::new(340282366920938463463374607431768211455))]
fn test_sqrt(#[case] x: U256, #[case] expected: U256) {
    let actual = sqrt(x).unwrap();
    assert_eq!(actual, expected);
}
