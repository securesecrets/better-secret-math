use super::*;
use crate::ud60x18::{
    mul, E, MAX_SCALED_UD60X18, MAX_UD60X18, MAX_WHOLE_UD60X18, PI, SQRT_MAX_UD60X18,
};

#[rstest]
#[case("0", MAX_UD60X18, "0")]
#[case(MAX_UD60X18, "0", "0")]
fn test_one_zero(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = mul(x, y).unwrap();
    assert_eq!(actual, expected);
}

#[rstest]
#[case(SQRT_MAX_UD60X18 + 1, SQRT_MAX_UD60X18 + 1)]
fn test_overflow(#[case] x: U256, #[case] y: U256) {
    assert!(mul(x, y).is_err());
}

#[rstest]
#[case("1", "1", U256::ZERO)]
#[case("6", exp10(17), U256::ZERO)]
#[case(exp10(9), exp10(9), U256::ONE)]
#[case(2_098 * exp10(15), 1_119 * exp10(15), 2_347662 * exp10(12))]
#[case(PI, E, "8539734222673567063")]
#[case(183 * exp10(17), 12_04 * exp10(16), 220_332 * exp10(15))]
#[case(314_271 * exp10(15), 18_819 * exp10(16), 5_914_265_949 * exp10(13))]
#[case(9_817 * exp10(18), 2_348 * exp10(18), 23_050_316 * exp10(18))]
#[case(12_983_989 * exp10(15), 78_299 * exp10(16), 1_016_633_354_711 * exp10(13))]
#[case(exp10(24), exp10(20), exp10(26))]
#[case(
    SQRT_MAX_UD60X18,
    SQRT_MAX_UD60X18,
    "115792089237316195423570985008687907853269984664959999305615707080986380425072"
)]
#[case(MAX_WHOLE_UD60X18, "1", MAX_SCALED_UD60X18)]
#[case(MAX_UD60X18 - (5 * exp10(17)), "1", MAX_SCALED_UD60X18)]
fn test_mul(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = mul(x, y).unwrap();
    assert_eq!(actual, expected);
}
