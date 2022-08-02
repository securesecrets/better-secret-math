use ethnum::{U256, I256};

/// Computes (x + y) % k where the addition is performed with arbitrary precision and does not wrap around at 2^256
pub fn addmod(x: U256, y: U256, k: U256) -> U256 {
    (x + y) % k
}

/// Computes (x * y) % k where the addition is performed with arbitrary precision and does not wrap around at 2^256
pub fn mulmod(x: U256, y: U256, k: U256) -> U256 {
    (x * y) % k
}

/// Compares the first and second operands and returns a value of 1 (true) if the first operand is greater than or equal the second, else a value of 0 (false).
pub fn gt(x: U256, y: U256) -> U256 {
    if x > y {
        U256::ONE
    } else {
        U256::ZERO
    }
}

/// Compares the first and second operands and returns a value of 0 (false) if the first operand is greater than or equal the second, else a value of 1 (true).
pub fn lt(x: U256, y: U256) -> U256 {
    if x < y {
        U256::ONE
    } else {
        U256::ZERO
    }
}

pub fn u_sub(x: U256, y: U256) -> U256 {
    x - y
}

pub fn sub(x: U256, y: U256) -> I256 {
    x.as_i256() - y.as_i256()
}

pub fn sgt(x: I256, y: I256) -> I256 {
    if x > y { I256::ONE } else { I256::ZERO }
}

pub fn or(x: U256, y: U256) -> U256 {
    x | y
}

pub fn mul(x: U256, y: U256) -> U256 {
    x * y
}

pub fn add(x: U256, y: U256) -> U256 {
    x + y
}

pub fn div(x: U256, y: U256) -> U256 {
    x / y
}
