use std::{collections::HashMap};

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

#[cfg(test)]
impl TokenState {
    pub fn mint_for_test(&mut self, address: Address, amount: Balance) {
        self.balances.insert(address, amount);
    }
}

impl TokenState {
    pub fn total_supply(&self) -> Balance {
        self.total_supply
    }

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_token() {
        // AAA Pattern(Arrange-Act-Assert)
        //   Arrange: 테스트 데이터 준비
        //   Act: 테스트할 함수 실행
        //   Assert: 결과 검증

        let creator = "alice".to_string();
        let initial_supply = 1000;

        let token = TokenState::new(creator, initial_supply);

        assert_eq!(token.total_supply(), initial_supply);
    }

    #[test]
    fn test_balance_of_existing_address() {
        let creator = String::from("alice");
        let initial_supply = 1000;
        let token = TokenState::new(creator.clone(), initial_supply);

        let balance = token.balance_of(&creator);
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_balance_of_non_existing_address() {
        let creator = String::from("alice");
        let initial_supply = 1000;
        let token = TokenState::new(creator.clone(), initial_supply);

        let bob = "bob".to_string();
        let balance = token.balance_of(&bob);
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_transfer_success() {
        let creator = "alice".to_string();
        let recipient = String::from("bob");
        let initial_supply = 1000;
        let mut token = TokenState::new(creator.clone(), initial_supply);

        let result = token.transfer(&creator, &recipient, 100);
        
        assert!(result.is_ok());
        assert_eq!(token.balance_of(&creator), 900);
        assert_eq!(token.balance_of(&recipient), 100);
        assert_eq!(token.total_supply(), 1000);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let creator = "alice".to_string();
        let recipient = String::from("bob");
        let initial_supply = 100;
        let mut token = TokenState::new(creator.clone(), initial_supply);

        let result = token.transfer(&creator, &recipient, 200);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TokenError::InsufficientBalance { required:200, available: 100 });
    }

    #[test]
    fn test_transfer_self() {
        let creator = "alice".to_string();
        let initial_supply = 1000;
        let mut token = TokenState::new(creator.clone(), initial_supply);

        let result = token.transfer(&creator, &creator, 100);
        assert_eq!(result.unwrap_err(), TokenError::SelfTransfer);
    }

    #[test]
    fn test_transfer_zero_amount() {
        let creator = "alice".to_string();
        let reciptient = "bob".to_string();
        let initial_supply = 1000;
        let mut token = TokenState::new(creator.clone(), initial_supply);

        let result = token.transfer(&creator, &reciptient, 0);
        assert_eq!(result.unwrap_err(), TokenError::ZeroAmount);
    }

    #[test]
    fn test_transfer_overflow() {
        let creator = "alice".to_string();
        let reciptient = "bob".to_string();
        let initial_supply = 1000;
        let mut token = TokenState::new(creator.clone(), initial_supply);

        // bob에게 일단 u64::MAX - 100을 줌
        token.mint_for_test(reciptient.clone(), u64::MAX - 100);

        let result = token.transfer(&creator, &reciptient, 200);
        assert_eq!(result.unwrap_err(), TokenError::BalanceOverFlow);
    }
}