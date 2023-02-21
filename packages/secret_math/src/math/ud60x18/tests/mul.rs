use super::*;
use crate::{ud60x18::mul, MAX_UD60x18, SQRT_MAX_UD60x18};

#[rstest]
#[case("0", MAX_UD60x18, "0")]
#[case(MAX_UD60x18, "0", "0")]
fn test_one_zero(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = mul(x, y).unwrap();
    assert_eq!(actual, expected);
}

#[rstest]
#[case(SQRT_MAX_UD60x18 + 1, SQRT_MAX_UD60x18 + 1)]
fn test_overflow(#[case] x: U256, #[case] y: U256) {
    assert!(mul(x, y).is_err());
}

// #[rstest]
// #[case("0", MAX_UD60x18, "0")]
// #[case(MAX_UD60x18, "0", "0")]
// fn test_one_zero(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
//     let actual = mul(x, y).unwrap();
//     assert_eq!(actual, expected);
// }
