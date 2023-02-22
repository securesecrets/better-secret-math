use crate::{
    common::{checked_add, checked_sub, muldiv},
    make_borsh,
};
use cosmwasm_std::StdResult;
use cosmwasm_std::Uint256;
use ethnum::U256;

make_borsh! {
    #[derive(Default, Eq)]
    Rebase {
        elastic: Uint256, U256, "";
        base: Uint256, U256, ""
    }
}

impl Rebase {
    pub fn init() -> Self {
        Rebase {
            elastic: Uint256::zero(),
            base: Uint256::zero(),
        }
    }

    /// Calculates the base value in relationship to `elastic` and self
    pub fn to_base(&self, elastic: Uint256, round_up: bool) -> StdResult<Uint256> {
        let mut base: Uint256;
        if self.elastic.is_zero() {
            base = elastic;
        } else {
            base = elastic.multiply_ratio(self.base, self.elastic);
            if round_up && base.multiply_ratio(self.elastic, self.base) < elastic {
                base = base.checked_add(Uint256::from(1u128))?;
            }
        }
        Ok(base)
    }

    /// Calculates the elastic value in relationship to `base` and self
    pub fn to_elastic(&self, base: Uint256, round_up: bool) -> StdResult<Uint256> {
        let mut elastic: Uint256;
        if self.base.is_zero() {
            elastic = base;
        } else {
            elastic = base.multiply_ratio(self.elastic, self.base);
            if round_up && elastic.multiply_ratio(self.base, self.elastic) < base {
                elastic = elastic.checked_add(Uint256::from(1u128))?;
            }
        }
        Ok(elastic)
    }

    /// Add `elastic` to `self` and update `total.base`
    pub fn add_elastic(
        &mut self,
        elastic: Uint256,
        round_up: bool,
    ) -> StdResult<(&mut Self, Uint256)> {
        let base = self.to_base(elastic, round_up)?;
        self.elastic = self.elastic.checked_add(elastic)?;
        self.base = self.base.checked_add(base)?;
        Ok((self, base))
    }

    /// Sub `elastic` from `self` and update `total.base`
    pub fn sub_elastic(
        &mut self,
        elastic: Uint256,
        round_up: bool,
    ) -> StdResult<(&mut Self, Uint256)> {
        let base = self.to_base(elastic, round_up)?;
        self.elastic = self.elastic.checked_sub(elastic)?;
        self.base = self.base.checked_sub(base)?;
        Ok((self, base))
    }

    /// Add `base` to `total` and update `self.elastic`
    pub fn add_base(&mut self, base: Uint256, round_up: bool) -> StdResult<(&mut Self, Uint256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.elastic = self.elastic.checked_add(elastic)?;
        self.base = self.base.checked_add(base)?;
        Ok((self, elastic))
    }

    /// Sub `base` from `total` and update `self.elastic`
    pub fn sub_base(&mut self, base: Uint256, round_up: bool) -> StdResult<(&mut Self, Uint256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.elastic = self.elastic.checked_sub(elastic)?;
        self.base = self.base.checked_sub(base)?;
        Ok((self, elastic))
    }

    /// Add `elastic` and `base` to self.
    pub fn add_self(&mut self, elastic: Uint256, base: Uint256) -> StdResult<&mut Self> {
        self.elastic = self.elastic.checked_add(elastic)?;
        self.base = self.base.checked_add(base)?;
        Ok(self)
    }

    /// Subtract `elastic` and `base` from self.
    pub fn sub_self(&mut self, elastic: Uint256, base: Uint256) -> StdResult<&mut Self> {
        self.elastic = self.elastic.checked_sub(elastic)?;
        self.base = self.base.checked_sub(base)?;
        Ok(self)
    }
}

impl BtrRebase {
    pub fn init() -> Self {
        BtrRebase {
            elastic: U256::ZERO,
            base: U256::ZERO,
        }
    }

    /// Calculates the base value in relationship to `elastic` and self
    pub fn to_base(&self, elastic: U256, round_up: bool) -> StdResult<U256> {
        let mut base: U256;
        if self.elastic == 0 {
            base = elastic;
        } else {
            base = muldiv(elastic, self.base, self.elastic)?;
            if round_up && muldiv(base, self.elastic, self.base)? < elastic {
                base += U256::ONE;
            }
        }
        Ok(base)
    }

    /// Calculates the elastic value in relationship to `base` and self
    pub fn to_elastic(&self, base: U256, round_up: bool) -> StdResult<U256> {
        let mut elastic: U256;
        if self.base == 0 {
            elastic = base;
        } else {
            elastic = muldiv(base, self.elastic, self.base)?;
            if round_up && muldiv(elastic, self.base, self.elastic)? < base {
                elastic += U256::ONE;
            }
        }
        Ok(elastic)
    }

    /// Add `elastic` to `self` and update `total.base`
    pub fn add_elastic(&mut self, elastic: U256, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        self.elastic = checked_add(self.elastic, elastic)?;
        self.base = checked_add(self.base, base)?;
        Ok((self, base))
    }

    /// Sub `elastic` from `self` and update `total.base`
    pub fn sub_elastic(&mut self, elastic: U256, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        self.elastic = checked_sub(self.elastic, elastic)?;
        // The amount we are subtracting from elastic and base are proportional in this function
        // so if we pass the checked_sub above, we don't need to check again.
        self.base -= base;
        Ok((self, base))
    }

    /// Add `base` to `total` and update `self.elastic`
    pub fn add_base(&mut self, base: U256, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.elastic = checked_add(self.elastic, elastic)?;
        self.base = checked_add(self.base, base)?;
        Ok((self, elastic))
    }

    /// Sub `base` from `total` and update `self.elastic`
    pub fn sub_base(&mut self, base: U256, round_up: bool) -> StdResult<(&mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.elastic = checked_sub(self.elastic, elastic)?;
        // The amount we are subtracting from elastic and base are proportional in this function
        // so if we pass the checked_sub above, we don't need to check again.
        self.base -= base;
        Ok((self, elastic))
    }
}

#[test]
fn test_rebase_math() {
    let mut total_borrowed = Rebase::init();
    let value = Uint256::from(100u128);
    total_borrowed.add_base(value, false).unwrap();
    assert_eq!(value, total_borrowed.elastic);
    assert_eq!(value, total_borrowed.base);
}

#[test]
fn test_rebase_math_2() {
    use std::ops::Div;

    let mut total_borrowed = Rebase::init();
    total_borrowed
        .add_base(Uint256::from(320u128), false)
        .unwrap();
    assert_eq!(
        Uint256::from(1u128),
        total_borrowed.elastic.div(total_borrowed.base)
    );
    total_borrowed.elastic = total_borrowed
        .elastic
        .checked_add(Uint256::from(160u128))
        .unwrap();
    assert_eq!(
        Uint256::from(30u128),
        total_borrowed
            .to_elastic(Uint256::from(20u128), true)
            .unwrap()
    );
}
