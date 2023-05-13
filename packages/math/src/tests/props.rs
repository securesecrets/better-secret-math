use cosmwasm_std::Uint256;
use ethnum::{U256, I256};
use proptest::{
    strategy::Strategy,
    proptest
};

use crate::common::muldiv;

fn arb_xyz(max_x: u128, max_y: u128, max_z: u128) -> impl Strategy<Value = (U256, U256, U256)> {
    (0..max_x, 0..max_y, 1..max_z).prop_map(|(x, y, z)| {
        let x = U256::from(x);
        let y = U256::from(y);
        let z = U256::from(z);
        (x, y, z)
    })
}

// Proptest muldiv (use cosmwasm_std for reference).
proptest! {
    #[test]
    fn proptest_muldiv(order in arb_xyz(100000000, 10000000, 10000000)) {
        let (x, y, z) = order;
        let xy = x * y;
        let xyz = xy / z;
        let muldiv_xyz = muldiv(x, y, z).unwrap();
        assert_eq!(xyz, muldiv_xyz);
    }

    #[test]
    fn proptest_muldiv_vs_checked_multiply_ratio(order in arb_xyz(143254 * 10u128.pow(13), 153749859331053729885, 220254119896034847314)) {
        let (x, y, z) = order;
        let (x_uint256, y_uint256, z_uint256) = (Uint256::from_u128(x.as_u128()), Uint256::from_u128(y.as_u128()), Uint256::from_u128(z.as_u128()));
        let muldiv_xyz = muldiv(x, y, z).unwrap();
        let checked_multiply_ratio_xyz = x_uint256.checked_multiply_ratio(y_uint256, z_uint256).unwrap();
        assert!(muldiv_xyz == U256::from(checked_multiply_ratio_xyz));
    }

    #[test]
    fn proptest_i256_sub(
        x in -100000i128..100000i128,
        y in -100000i128..100000i128,
    ) {
        let a = I256::from(x);
        let b = I256::from(y);
        println!("a: {}, b: {}", a, b);
        let c = a - b;
        let z = x - y;
        assert_eq!(c.as_i128(), z);
    }
}