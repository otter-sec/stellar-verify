use crate::{address::Address, env::Env};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AddressPair {
    from: Address,
    to: Address,
}

#[derive(Clone, Debug)]
pub struct MockToken {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i128,
    pub balances: Vec<i128>,
    pub admin: Address,
}

impl MockToken {
    pub fn new(
        address: Address,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: i128,
        admin: Address,
    ) -> Self {
        Self {
            address,
            name,
            symbol,
            decimals,
            total_supply,
            balances: vec![0; 100],
            admin,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub env: Env,
    pub address: Address,
}

impl Client {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {
            env: env.clone(),
            address: *address,
        }
    }

    pub fn get_self_token(&self) -> MockToken {
        self.env
            .storage
            .borrow()
            .get_token(&self.address)
            .expect("Token not found")
    }

    pub fn balance(&self, address: &Address) -> i128 {
        let token = self.get_self_token();
        token.balances[address.val as usize]
    }

    pub fn transfer(&self, from: &Address, to: &Address, amount: &i128) {
        let mut token = self.get_self_token();
        let prev_bal_from = self.balance(from);

        assert!(prev_bal_from >= *amount, "Insufficient balance");

        let prev_bal_to = self.balance(to);

        let new_bal_from = prev_bal_from
            .checked_sub(*amount)
            .expect("Subtraction overflow");
        let new_bal_to = prev_bal_to.checked_add(*amount).expect("Addition overflow");

        token.balances[from.val as usize] = new_bal_from;
        token.balances[to.val as usize] = new_bal_to;

        self.env.storage.borrow_mut().update_token(token.clone());
    }
}

#[derive(Clone, Debug)]
pub struct AdminClient {
    pub env: Env,
    pub address: Address,
}

impl AdminClient {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {
            env: env.clone(),
            address: *address,
        }
    }

    pub fn get_self_token(&self) -> MockToken {
        self.env
            .storage
            .borrow()
            .get_token(&self.address)
            .expect("Token not found")
    }

    pub fn update_self_token(&self, token: &MockToken) {
        self.env.storage.borrow_mut().update_token(token.clone());
    }

    pub fn balance(&self, address: &Address) -> i128 {
        let token = self.get_self_token();
        token.balances[address.val as usize]
    }

    pub fn mint(&self, to: &Address, amount: &i128) {
        assert!(*amount > 0, "Minted amount must be positive");

        let mut token = self.get_self_token();
        let prev_bal = self.balance(to);

        let new_bal = prev_bal.checked_add(*amount).expect("Addition overflow");

        token.balances[to.val as usize] = new_bal;
        self.update_self_token(&token);
    }

    pub fn admin(&self) -> Address {
        let token = self.get_self_token();
        token.admin
    }

    pub fn set_admin(&self, new_admin: &Address) {
        let mut token = self.get_self_token();
        token.admin = *new_admin;
        self.env.storage.borrow_mut().update_token(token.clone());
    }
}
