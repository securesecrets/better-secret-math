use cosmwasm_std::{StdError, StdResult};
use ethnum::{I256, U256};

#[derive(thiserror::Error, Debug)]
pub enum AsmError {
    #[error("UD60x18 Addition overflow: {0} + {1}")]
    AddOverflow(U256, U256),
    #[error("UD60x18 Subtraction underflow: {0} - {1}")]
    SubUnderflow(U256, U256),
    #[error("UD60x18 Mul overflow: {0} * {1}")]
    MulOverflow(U256, U256),
}

impl Into<StdError> for AsmError {
    fn into(self) -> StdError {
        StdError::generic_err(self.to_string())
    }
}

/// Assembly math operations
pub struct Asm;

impl Asm {
    /// Computes (x + y) % k where the addition is performed with arbitrary precision and does not wrap around at 2^256
    pub fn addmod(x: U256, y: U256, k: U256) -> StdResult<U256> {
        match x.checked_add(y) {
            Some(sum) => Ok(sum % k),
            None => Err(AsmError::AddOverflow(x, y).into()),
        }
    }

    /// Computes (x * y) % k where the addition is performed with arbitrary precision and does not wrap around at 2^256
    pub fn mulmod(x: U256, y: U256, k: U256) -> StdResult<U256> {
        match x.checked_mul(y) {
            Some(product) => Ok(product % k),
            None => Err(AsmError::MulOverflow(x, y).into()),
        }
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

    pub fn u_sub(x: U256, y: U256) -> StdResult<U256> {
        match x.checked_sub(y) {
            Some(diff) => Ok(diff),
            None => Err(AsmError::SubUnderflow(x, y).into()),
        }
    }

    pub fn sub(x: U256, y: U256) -> StdResult<I256> {
        match x.as_i256().checked_sub(y.as_i256()) {
            Some(diff) => Ok(diff),
            None => Err(AsmError::SubUnderflow(x, y).into()),
        }
    }

    pub fn sgt(x: I256, y: I256) -> I256 {
        if x > y {
            I256::ONE
        } else {
            I256::ZERO
        }
    }

    pub fn or(x: U256, y: U256) -> U256 {
        x | y
    }

    pub fn mul(x: U256, y: U256) -> StdResult<U256> {
        match x.checked_mul(y) {
            Some(product) => Ok(product),
            None => Err(AsmError::MulOverflow(x, y).into()),
        }
    }

    pub fn add(x: U256, y: U256) -> StdResult<U256> {
        match x.checked_add(y) {
            Some(sum) => Ok(sum),
            None => Err(AsmError::AddOverflow(x, y).into()),
        }
    }

    pub fn div(x: U256, y: U256) -> U256 {
        x / y
    }
}
