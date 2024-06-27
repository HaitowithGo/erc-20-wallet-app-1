// ERC-20 token contract
#[ink::contract]
pub mod my_erc20_token {
    #[ink(storage)]
    pub struct Erc20Token {
        total_supply: Balance,
        balances: storage::HashMap<AccountId, Balance>,
    }

    impl Erc20Token {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            Self {
                total_supply: initial_supply,
                balances: storage::HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            *self.balances.get(&account).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn mint(&self, recipient: AccountId, amount: Balance) -> Result<(), Error> {
            let mut new_total_supply = self.total_supply;
            new_total_supply.checked_add(amount)?;
            self.total_supply = new_total_supply;

            let mut balances = self.balances.clone();
            let mut recipient_balance = balances.entry(recipient).or_insert(0);
            *recipient_balance += amount;
            self.balances = balances;

            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), Error> {
            if self.balances[&from] < amount {
                return Err(Error::InsufficientFunds);
            }

            let mut balances = self.balances.clone();
            *balances.get_mut(&from) -= amount;
            *balances.entry(to).or_insert(0) += amount;
            self.balances = balances;

            Ok(())
        }
    }
}

// wallet contract
#[ink::contract]
pub mod my_wallet {
    use ink_std::collections::HashMap;

    #[ink(storage)]
    pub struct Wallet {
        balances: HashMap<AccountId, Balance>,
    }

    impl Wallet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn deposit(&self, amount: Balance) -> Result<(), Error> {
            let mut balances = self.balances.clone();
            *balances.entry(self.env().caller()).or_insert(0) += amount;
            self.balances = balances;

            Ok(())
        }

        #[ink(message)]
        pub fn withdraw(&self, amount: Balance) -> Result<(), Error> {
            let mut balances = self.balances.clone();
            let balance = balances.get_mut(&self.env().caller()).unwrap_or_else(|| return Err(Error::InsufficientFunds));
            if *balance < amount {
                return Err(Error::InsufficientFunds);
            }

            *balance -= amount;
            self.balances = balances;

            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            *self.balances.get(&account).unwrap_or(&0)
        }
    }
}
