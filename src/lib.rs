use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub enum TokenError {}

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
    ) -> Result<(), String> {
        if from == to {
            return Ok(());
        }

        let from_bal = self.balance_of(from);
        if from_bal < amount {
            return Err("Insufficient balance".to_string());
        }

        self.balances.insert(from.clone(), from_bal - amount);
        let to_bal = self.balance_of(to);
        self.balances.insert(to.clone(), to_bal + amount);

        Ok(())
    }
}
