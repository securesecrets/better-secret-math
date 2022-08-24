#[cfg(test)]
mod tests {
    use btr_macros::support_interface;

    #[derive(support_interface)]
    pub struct DeriveTest {
        pub string: String,
        pub other: u64,
    }

    #[derive(support_interface)]
    pub struct AttributeTest {
        #[has_interface]
        pub test: DeriveTest,
    }

    #[test]
    fn support_interface_generation() {
        // Test that is builds
        let assert = DeriveTestInterface {
            other: 10u64,
            string: "test".into(),
        };
    }

    #[test]
    fn has_interface_generation() {
        //Test that is builds
        let assert = AttributeTestInterface {
            test: DeriveTestInterface {
                other: 10u64,
                string: "test".into(),
            },
        };
    }
}
