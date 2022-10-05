#[cfg(test)]
mod tests {
    use better_secret_math::U256;
    use btr_macros::support_interface;
    use cosmwasm_std::Uint128;

    #[derive(support_interface)]
    pub struct Derive {
        pub string: String,
        pub other: u8,
    }

    impl Default for Derive {
        fn default() -> Self {
            Self {
                string: "".to_string(),
                other: 0,
            }
        }
    }

    #[test]
    fn into() {
        let assert = DeriveInterface {
            other: 0,
            string: "".into(),
        };

        let from: DeriveInterface = Derive::default().into();

        assert_eq!(assert.other, from.other);
        assert_eq!(assert.string, from.string);
    }

    #[test]
    fn struct_generation() {
        use cosmwasm_std::Decimal256;
        // Test that is builds
        let assert = DeriveInterface {
            other: 10,
            string: "test".into(),
        };

        // Test that interface replacement works
        #[derive(support_interface)]
        pub struct AttributeTest {
            #[has_interface]
            pub test: Derive,
        }

        let assert = AttributeTestInterface {
            test: DeriveInterface {
                other: 10,
                string: "test".into(),
            },
        };
    }

    #[test]
    fn import_generation() {
        #[derive(support_interface)]
        #[no_shd]
        pub struct Imports {
            pub a: U256,
            pub b: u128,
            pub c: u64,
        }

        //use cosmwasm_std::{Uint128, Uint256, Uint64};
        let assert = ImportsInterface {
            a: Uint256::from(10u8),
            b: Uint128::from(10u8),
            c: Uint64::from(10u8),
        };

        let from: ImportsInterface = Imports {
            a: U256::from(10u8),
            b: 10,
            c: 10,
        }
        .into();

        assert_eq!(assert.b, from.b);
    }

    #[test]
    fn newtype_struct_generation() {
        #[derive(support_interface)]
        pub struct Test(String);

        let assert = TestInterface("test".into());

        #[derive(support_interface)]
        pub struct AttributeTest(String, #[has_interface] Derive);

        let assert = AttributeTestInterface(
            "test".into(),
            DeriveInterface {
                other: 10,
                string: "test".into(),
            },
        );

        #[derive(support_interface)]
        #[no_shd]
        pub struct TestInto(u128, #[has_interface] Derive);

        let assert = TestIntoInterface(
            Uint128::new(100),
            DeriveInterface {
                string: "".into(),
                other: 0,
            },
        );

        let into: TestIntoInterface = TestInto(100, Derive::default()).into();

        assert_eq!(assert.1.string, into.1.string);
        assert_eq!(assert.0, into.0);
    }
}
