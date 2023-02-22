use super::*;
use crate::ud60x18::{avg, MAX_UD60X18, MAX_WHOLE_UD60X18};

#[rstest]
#[case("0", "0", "0")]
fn test_avg_both_zero(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = avg(x, y);
    assert_eq!(actual, expected);
}

#[rstest]
#[case(U256::ZERO, 3 * exp10(18), 15 * exp10(17))]
#[case(3 * exp10(18), U256::ZERO, 15 * exp10(17))]
fn test_avg_one_zero(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = avg(x, y);
    assert_eq!(actual, expected);
}

#[rstest]
#[case("2", "4", "3")]
#[case(2 * exp10(18), 2 * exp10(18), 2 * exp10(18))]
#[case(4 * exp10(18), 8 * exp10(18), 6 * exp10(18))]
#[case(100 * exp10(18), 200 * exp10(18), 150 * exp10(18))]
#[case(1 * exp10(24), 1 * exp10(25), 55 * exp10(23))]
fn test_avg_both_even(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = avg(x, y);
    assert_eq!(actual, expected);
}

#[rstest]
#[case("1", "3", "2")]
#[case(exp10(18) + 1, exp10(18) + 1, exp10(18) + 1)]
#[case(3 * exp10(18) + 1, 7 * exp10(18) + 1, 5 * exp10(18) + 1)]
#[case(99 * exp10(18) + 1, 199 * exp10(18) + 1, 149 * exp10(18) + 1)]
#[case(1 * exp10(24) + 1, 1 * exp10(25) + 1, 55 * exp10(23) + 1)]
#[case(MAX_UD60X18, MAX_UD60X18, MAX_UD60X18)]
fn test_avg_both_odd(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = avg(x, y);
    assert_eq!(actual, expected);
}

#[rstest]
#[case("1", "2", "1")]
#[case(exp10(18) + 1, 2 * exp10(18), 15 * exp10(17))]
#[case(3 * exp10(18) + 1, 8 * exp10(18), 55 * exp10(17))]
#[case(99 * exp10(18), 200 * exp10(18), 1495 * exp10(17))]
#[case(exp10(24) + 1, exp10(25) + exp10(18), (55 * exp10(23)) + (5 * exp10(17)))]
#[case(
    MAX_UD60X18,
    MAX_WHOLE_UD60X18,
    U256::from_words(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFBF297F2D7F1FFFF)
)]
fn test_avg_one_odd_one_even(#[case] x: U256, #[case] y: U256, #[case] expected: U256) {
    let actual = avg(x, y);
    assert_eq!(actual, expected);
}
