use cosmwasm_std::{StdResult, StdError};
use ethnum::U256;

use crate::common::{exp10, muldiv, bankers_round};

pub trait TokenMath {
    const NORMALIZED_PRECISION: u8;
    const BANKERS_ROUNDING_ENABLED: bool;
    /// Amount (token decimal precision) -> Amount (normalized decimal precision).
    fn normalize_amount_from_any_utokens(amount: impl Into<U256>, token_decimals: u8) -> StdResult<U256> {
        let amount: U256 = amount.into();
        if token_decimals == Self::NORMALIZED_PRECISION {
            Ok(amount)
        } else {
            muldiv(amount, exp10(Self::NORMALIZED_PRECISION.into()), exp10(token_decimals))
        }
    }

    /// Amount (normalized decimal precision) -> Amount (token decimal precision).
    fn denormalize_amount_to_any_utokens(amount: impl Into<U256>, token_decimals: u8) -> StdResult<U256> {
        let normalized_amount: U256 = amount.into();
        if token_decimals == Self::NORMALIZED_PRECISION {
            Ok(normalized_amount)
        } else {
            if Self::BANKERS_ROUNDING_ENABLED {
                if token_decimals > Self::NORMALIZED_PRECISION {
                    return Err(StdError::generic_err(format!("Token decimals {} must be <= normalized precision {}", token_decimals, Self::NORMALIZED_PRECISION)));
                };
                let precision_diff = Self::NORMALIZED_PRECISION - token_decimals;
                Ok(bankers_round(normalized_amount.into(), precision_diff) / exp10(precision_diff))
            } else {
                Ok(normalized_amount / exp10(Self::NORMALIZED_PRECISION - token_decimals))
            }
        }
    }

    /// Amount (normalized decimal precision) -> Amount (normalized decimals, but excess precision truncated or rounded)
    fn normalize_amount_to_any_token_precision(amount: impl Into<U256>, token_decimals: u8) -> StdResult<U256> {
        let amount: U256 = amount.into();
        if token_decimals == Self::NORMALIZED_PRECISION {
            Ok(amount)
        } else {
            if token_decimals > Self::NORMALIZED_PRECISION {
                return Err(StdError::generic_err(format!("Token decimals {} must be <= normalized precision {}", token_decimals, Self::NORMALIZED_PRECISION)));
            };
            let precision_diff = Self::NORMALIZED_PRECISION - token_decimals;
            if Self::BANKERS_ROUNDING_ENABLED {
                Ok(bankers_round(amount, precision_diff))
            } else {
                let truncated = amount / exp10(precision_diff);
                Ok(truncated * exp10(precision_diff))
            }
        }
    }

    fn token_decimals(&self) -> u8;
    /// Amount (token decimal precision) -> Amount (normalized decimal precision).
    fn normalize_amount_from_utokens(&self, amount: impl Into<U256>) -> StdResult<U256> {
        Self::normalize_amount_from_any_utokens(amount, self.token_decimals())
    }

    /// Amount (normalized decimal precision) -> Amount (token decimal precision).
    fn denormalize_amount_to_utokens(&self, amount: impl Into<U256>) -> StdResult<U256> {
        Self::denormalize_amount_to_any_utokens(amount, self.token_decimals())
    }

    /// Amount (normalized decimal precision) -> Amount (normalized decimals, but excess precision truncated or rounded)
    fn normalize_amount_to_token_precision(&self, amount: impl Into<U256>) -> StdResult<U256> {
        Self::normalize_amount_to_any_token_precision(amount, self.token_decimals())
    }
}

pub trait PriceMath {
    const PRICE_PRECISION: u8;
    fn price(&self) -> U256;

    /// Gets the value for some amount using the price.
    fn calc_value_from_amount(&self, amount: impl Into<U256> + Copy) -> StdResult<U256> {
        let price_precision = exp10(Self::PRICE_PRECISION);
        let amount: U256 = amount.into();
        muldiv(amount, self.price(), price_precision)
    }

    /// Gets the amount equivalent to the provided value divided by the unit price.
    fn calc_amount_from_value(
        &self,
        value: impl Into<U256> + Copy,
        value_precision: u8,
        amount_precision: u8,
    ) -> StdResult<U256> {
        let price_precision = exp10(Self::PRICE_PRECISION);
        let value_precision = exp10(value_precision);
        let amount_precision = exp10(amount_precision);
        let value: U256 = value.into();

        let normalized_value = muldiv(value, price_precision, value_precision)?;
        muldiv(normalized_value, amount_precision, self.price())
    }

}