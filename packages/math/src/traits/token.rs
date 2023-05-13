use cosmwasm_std::StdResult;
use ethnum::U256;

use crate::common::{exp10, muldiv, bankers_round};

pub trait TokenMath {
    fn token_decimals(&self) -> u8;
    /// Normalizes the asset amount from being based off asset decimals -> 18 decimals.
    fn normalize(&self, amount: impl Into<U256>) -> StdResult<U256> {
        let amount: U256 = amount.into();
        let decimals = self.token_decimals();
        if decimals == 18 {
            Ok(amount)
        } else {
            muldiv(amount, exp10(18), exp10(decimals as u16))
        }
    }

    /// Normalized amount -> actual token amount.
    fn denormalize(&self, normalized_amount: impl Into<U256>) -> StdResult<U256> {
        let normalized_amount: U256 = normalized_amount.into();
        let decimals = self.token_decimals();
        if decimals == 18 {
            Ok(normalized_amount)
        } else {
            Ok(normalized_amount / exp10((18 - decimals) as u16))
        }
    }

    /// Normalized amount -> normalized token amount based off token precision.
    fn to_token_precision(&self, amount: impl Into<U256>, round: bool) -> StdResult<U256> {
        let amount: U256 = amount.into();
        let decimals = self.token_decimals();
        if decimals == 18 {
            Ok(amount)
        } else {
            let precision_diff = 18 - decimals;
            if round {
                Ok(bankers_round(amount, precision_diff))
            } else {
                let truncated = amount / exp10(precision_diff as u16);
                Ok(truncated * exp10((precision_diff) as u16))
            }
        }
    }
}