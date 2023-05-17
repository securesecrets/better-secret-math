use crate::{
    common::exp10,
    ud60x18::{
        constants::{MAX_UD60X18, MAX_WHOLE_UD60X18, PI},
        inv,
    },
};
use ethnum::U256;
use rstest::*;

#[test]
fn test_err() {
    assert!(inv(U256::ZERO).is_err());
}

// sets.push(set({ x: 0.000000000000000001e18, expected: 1e36 }));
// sets.push(set({ x: 0.00001e18, expected: 100_000e18 }));
// sets.push(set({ x: 0.05e18, expected: 20e18 }));
// sets.push(set({ x: 0.1e18, expected: 10e18 }));
// sets.push(set({ x: 1e18, expected: 1e18 }));
// sets.push(set({ x: 2e18, expected: 0.5e18 }));
// sets.push(set({ x: PI, expected: 0.318309886183790671e18 }));
// sets.push(set({ x: 4e18, expected: 0.25e18 }));
// sets.push(set({ x: 22e18, expected: 0.045454545454545454e18 }));
// sets.push(set({ x: 100.135e18, expected: 0.009_986_518_200_429_420e18 }));
// sets.push(set({ x: 772.05e18, expected: 0.001295252898128359e18 }));
// sets.push(set({ x: 2503e18, expected: 0.000399520575309628e18 }));
// sets.push(set({ x: 1e36, expected: 0.000000000000000001e18 }));
// sets.push(set({ x: 1e36 + 1, expected: 0 }));
// sets.push(set({ x: MAX_WHOLE_UD60x18, expected: 0 }));
// sets.push(set({ x: MAX_UD60x18, expected: 0 }));

#[rstest]
#[case(U256::ONE, exp10(36))]
#[case(0_00001 * exp10(13), 100_000 * exp10(18))]
#[case(0_05 * exp10(16), 20 * exp10(18))]
#[case(0_1 * exp10(17), 10 * exp10(18))]
#[case(exp10(18), exp10(18))]
#[case(2 * exp10(18), 0_5 * exp10(17))]
#[case(PI, U256::new(318_309_886_183_790_671))]
#[case(4 * exp10(18), 0_25 * exp10(16))]
#[case(22 * exp10(18), U256::new(45_454_545_454_545_454))]
#[case(100_135 * exp10(15), U256::new(0_009_986_518_200_429_420))]
#[case(77_205 * exp10(16), U256::new(1_295_252_898_128_359))]
#[case(2503 * exp10(18), U256::new(399_520_575_309_628))]
#[case(exp10(36), U256::ONE)]
#[case(exp10(36) + 1, U256::ZERO)]
#[case(MAX_WHOLE_UD60X18, U256::ZERO)]
#[case(MAX_UD60X18, U256::ZERO)]
fn test_ln(#[case] actual: U256, #[case] expected: U256) {
    let actual = inv(actual).unwrap();
    assert_eq!(actual, expected);
}
