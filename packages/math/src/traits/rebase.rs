use btr_macros::borsh_serde;
use cosmwasm_std::{Uint256, StdResult};
use ethnum::U256;

use crate::common::{muldiv, checked_add, checked_sub};

pub trait Rebase {
    fn elastic_uint256(&self) -> Uint256;
    fn base_uint256(&self) -> Uint256;
    fn elastic(&self) -> U256;
    fn base(&self) -> U256;
    fn set_elastic(&mut self, elastic: U256);
    fn set_base(&mut self, base: U256);
    fn into_rebase(&self) -> SimpleRebase {
        SimpleRebase::new(self.elastic(), self.base())
    }
    /// Calculates the base value in relationship to `elastic` and self
    fn to_base(&self, elastic: impl Into<U256> + Copy, round_up: bool) -> StdResult<U256> {
        let elastic = elastic.into();
        let mut base: U256;
        if self.elastic() == 0 {
            base = elastic;
        } else {
            base = muldiv(elastic, self.base(), self.elastic())?;
            if round_up && muldiv(base, self.elastic(), self.base())? < elastic {
                base += U256::ONE;
            }
        }
        Ok(base)
    }

    /// Calculates the elastic value in relationship to `base` and self
    fn to_elastic(&self, base: impl Into<U256> + Copy, round_up: bool) -> StdResult<U256> {
        let base = base.into();
        let mut elastic: U256;
        if self.base() == 0 {
            elastic = base;
        } else {
            elastic = muldiv(base, self.elastic(), self.base())?;
            if round_up && muldiv(elastic, self.base(), self.elastic())? < base {
                elastic += U256::ONE;
            }
        }
        Ok(elastic)
    }

    /// Add `elastic` to `self` and update `total.base`
    fn add_elastic(&mut self, elastic: impl Into<U256> + Copy, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        let elastic: U256 = elastic.into();
        self.set_elastic(checked_add(self.elastic(), elastic)?);
        self.set_base(checked_add(self.base(), base)?);
        Ok((self, base))
    }

    /// Sub `elastic` from `self` and update `total.base`
    fn sub_elastic(&mut self, elastic: impl Into<U256> + Copy, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        let elastic: U256 = elastic.into();
        self.set_elastic(checked_sub(self.elastic(), elastic)?);
        // The amount we are subtracting from elastic and base are proportional in this function
        // so if we pass the checked_sub above, we don't need to check again.
        self.set_base(self.base() - base);
        Ok((self, base))
    }

    /// Add `base` to `total` and update `self.elastic()`
    fn add_base(&mut self, base: impl Into<U256> + Copy, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.set_elastic(checked_add(self.elastic(), elastic)?);
        let base: U256 = base.into();
        self.set_base(checked_add(self.base(), base)?);
        Ok((self, elastic))
    }

    /// Sub `base` from `total` and update `self.elastic()`
    fn sub_base(&mut self, base: impl Into<U256> + Copy, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.set_elastic(checked_sub(self.elastic(), elastic)?);
        // The amount we are subtracting from elastic and base are proportional in this function
        // so if we pass the checked_sub above, we don't need to check again.
        let base: U256 = base.into();
        self.set_base(self.base() - base);
        Ok((self, elastic))
    }
}

#[borsh_serde]
#[derive(Default)]
pub struct SimpleRebase {
    pub elastic: U256,
    pub base: U256,
}

impl SimpleRebase {
    pub fn new(elastic: U256, base: U256) -> Self {
        Self { elastic, base }
    }
}

impl Rebase for SimpleRebase {
    fn elastic_uint256(&self) -> Uint256 {
        self.elastic.into()
    }

    fn base_uint256(&self) -> Uint256 {
        self.base.into()
    }

    fn elastic(&self) -> U256 {
        self.elastic
    }

    fn base(&self) -> U256 {
        self.base
    }

    fn set_elastic(&mut self, elastic: U256) {
        self.elastic = elastic;
    }

    fn set_base(&mut self, base: U256) {
        self.base = base;
    }
}
