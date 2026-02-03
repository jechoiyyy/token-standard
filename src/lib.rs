use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TokenError {
    InsufficientBalance {
        required: Balance,
        available: Balance,
    },
    SelfTransfer,
    ZeroAmount,
    BalanceOverFlow,
}

pub type Address = String; // 일단 간단하게
pub type Balance = u64;

pub struct TokenState {
    balances: HashMap<Address, Balance>,
    total_supply: Balance,
}

impl TokenState {
    pub fn new(creator: Address, initial_supply: Balance) -> Self {
        let mut balances = HashMap::new();
        balances.insert(creator, initial_supply);

        Self {
            balances,
            total_supply: initial_supply,
        }
    }

    pub fn balance_of(&self, address: &Address) -> Balance {
        self.balances.get(address).copied().unwrap_or(0)
    }

    pub fn transfer(
        &mut self,
        from: &Address,
        to: &Address,
        amount: Balance,
    ) -> Result<(), TokenError> {
        if from == to {
            return Err(TokenError::SelfTransfer);
        }
        if amount == 0 {
            return Err(TokenError::ZeroAmount);
        }

        let from_bal = self.balance_of(from);
        if from_bal < amount {
            return Err(TokenError::InsufficientBalance {
                required: amount,
                available: from_bal,
            });
        }

        let to_bal = self
            .balance_of(to)
            .checked_add(amount)
            .ok_or(TokenError::BalanceOverFlow)?;

        self.balances.insert(from.clone(), from_bal - amount);
        self.balances.insert(to.clone(), to_bal);

        Ok(())
    }
}
