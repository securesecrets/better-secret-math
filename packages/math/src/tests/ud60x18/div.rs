use super::*;
use crate::ud60x18::{div, MAX_SCALED_UD60X18, MAX_UD60X18, MAX_WHOLE_UD60X18, PI};

#[rstest]
#[case(U256::ONE, U256::ZERO)]
#[case(MAX_SCALED_UD60X18 + 1, "1")]
fn test_div_error(#[case] x: U256, #[case] y: U256) {
    assert!(div(x, y).is_err());
}

#[rstest]
#[case(U256::ZERO, U256::ONE, U256::ZERO)]
#[case(U256::ZERO, exp10(18), U256::ZERO)]
#[case(U256::ZERO, PI, U256::ZERO)]
#[case(U256::ZERO, exp10(24), U256::ZERO)]
fn test_div_by_zero(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = div(x, y).unwrap();
    assert_eq!(actual, expected);
}

#[rstest]
#[case("1", MAX_UD60X18, U256::ZERO)]
#[case("1", exp10(18) + U256::ONE, U256::ZERO)]
#[case("1", exp10(18), "1")]
#[case(exp10(13), exp10(13), exp10(18))]
#[case(exp10(13), 2 * exp10(13), 5 * exp10(17))]
#[case(5 * exp10(16), 2 * exp10(16), 25 * exp10(17))]
#[case(exp10(17), exp10(16), exp10(19))]
#[case(2 * exp10(18), 2 * exp10(18), exp10(18))]
#[case(2 * exp10(18), 5 * exp10(18), 4 * exp10(17))]
#[case(4 * exp10(18), 2 * exp10(18), 2 * exp10(18))]
#[case(22 * exp10(18), 7 * exp10(18), "3142857142857142857")]
#[case(100_135 * exp10(15), 100_134 * exp10(15), "1000009986617931971")]
#[case(77_205 * exp10(16), 19_998 * exp10(16), "3860636063606360636")]
#[case(2503 * exp10(18), 91_888_211 * exp10(16), "2723962054283546" )]
#[case(exp10(24), exp10(18), exp10(24))]
#[case(MAX_SCALED_UD60X18, "1", MAX_WHOLE_UD60X18)]
fn test_div(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = div(x, y).unwrap();
    assert_eq!(actual, expected);
}
