use ethnum::U256;
use rstest::*;
use crate::{ud60x18::{
    constants::{ E, PI, MAX_WHOLE_UD60X18, MAX_UD60X18 },
    frac
}, common::exp10};

#[test]
fn test_zero() {
    let actual = frac(U256::ZERO);
    assert_eq!(actual, U256::ZERO);
}

// sets.push(set({ x: 0.1e18, expected: 0.1e18 }));
// sets.push(set({ x: 0.5e18, expected: 0.5e18 }));
// sets.push(set({ x: 1e18, expected: 0 }));
// sets.push(set({ x: 1.125e18, expected: 0.125e18 }));
// sets.push(set({ x: 2e18, expected: 0 }));
// sets.push(set({ x: PI, expected: 0.141592653589793238e18 }));
// sets.push(set({ x: 4.2e18, expected: 0.2e18 }));
// sets.push(set({ x: 1e24, expected: 0 }));
// sets.push(set({ x: MAX_WHOLE_UD60x18, expected: 0 }));
// sets.push(set({ x: MAX_UD60x18, expected: 0.584007913129639935e18 }));

#[rstest]
#[case(exp10(17), exp10(17))]
#[case(exp10(17) * 5, exp10(17) * 5)]
#[case(exp10(18), U256::ZERO)]
#[case(1_125 * exp10(15), 0_125 * exp10(15))]
#[case(exp10(18) * 2, U256::ZERO)]
#[case(PI, U256::new(0_141592653589793238))]
#[case(4_2 * exp10(17), 0_2 * exp10(17))]
#[case(exp10(24), U256::ZERO)]
#[case(MAX_WHOLE_UD60X18, U256::ZERO)]
#[case(MAX_UD60X18, U256::new(0_584007913129639935))]
fn test_frac(#[case] actual: U256, #[case] expected: U256) {
    let actual = frac(actual);
    assert_eq!(actual, expected);
}