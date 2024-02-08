use crate::{address::Address, env::Env, string::String};

#[doc(hidden)]
#[deprecated(note = "use TokenInterface")]
pub use TokenInterface as Interface;

#[doc(hidden)]
#[deprecated(note = "use TokenClient")]
pub use TokenClient as Client;

#[derive(Clone, Debug, Default)]
pub struct MockToken {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i128,
    pub balances: Vec<i128>,        // balances[owner_index]
    pub allowances: Vec<Vec<i128>>, // allowances[owner_index][spender_index]
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
            allowances: vec![vec![0; 100]; 100],
            admin,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TokenClient {
    pub env: Env,
    pub address: Address,
}

impl TokenClient {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {
            env: env.clone(),
            address: *address,
        }
    }

    pub fn decimals(&self, env: Env) -> u32 {
        let token = self.get_self_token();
        token.decimals as u32
    }

    pub fn name(&self, env: Env) -> String {
        let token = self.get_self_token();
        token.name
    }

    pub fn symbol(&self, env: Env) -> String {
        let token = self.get_self_token();
        token.symbol
    }

    pub fn get_self_token(&self) -> MockToken {
        let token = self.env.storage.borrow().get_token(&self.address);
        if let Some(token) = token {
            token
        } else {
            panic!("Token not found")
        }
    }

    pub fn balance(&self, address: &Address) -> i128 {
        let token = self.get_self_token();
        token.balances[address.val as usize]
    }

    pub fn transfer(&self, from: &Address, to: &Address, amount: &i128) {
        let mut token = self.get_self_token();
        let prev_bal_from = self.balance(from);

        assert!(prev_bal_from >= *amount);

        let prev_bal_to = self.balance(to);

        let new_bal_from = prev_bal_from.saturating_sub(*amount);
        let new_bal_to = prev_bal_to.saturating_add(*amount);

        token.balances[from.val as usize] = new_bal_from;
        token.balances[to.val as usize] = new_bal_to;

        self.env.storage.borrow_mut().update_token(token.clone());
    }

    pub fn mint(&self, to: &Address, amount: &i128) {
        assert!(*amount > 0, "Mint amount must be positive");

        let mut token = self.get_self_token();
        let prev_bal = self.balance(to);

        let new_bal = prev_bal.saturating_add(*amount);

        token.balances[to.val as usize] = new_bal;
        self.env.storage.borrow_mut().update_token(token.clone());
    }

    pub fn burn(&self, from: &Address, amount: &i128) {
        let mut token = self.get_self_token();
        let prev_bal = self.balance(from);

        assert!(prev_bal >= *amount, "Insufficient balance");

        let new_bal = prev_bal.saturating_sub(*amount);

        token.balances[from.val as usize] = new_bal;
        self.env.storage.borrow_mut().update_token(token.clone());
    }

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    /// `from`.
    pub fn approve(
        &self,
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        _expiration_ledger: u32,
    ) {
        let mut token = self.get_self_token();
        token.allowances[from.val as usize][spender.val as usize] = amount;
        self.env.storage.borrow_mut().update_token(token.clone());
    }

    /// Returns the allowance for `spender` to transfer from `from`.
    pub fn allowance(&self, env: Env, from: Address, spender: Address) -> i128 {
        let token = self.get_self_token();
        token.allowances[from.val as usize][spender.val as usize]
    }

    /// Transfer `amount` from `from` to `to`, consuming the allowance of
    /// `spender`. Authorized by spender (`spender.require_auth()`).
    pub fn transfer_from(
        &self,
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        let mut token = self.get_self_token();
        let prev_bal_from = self.balance(&from);

        assert!(prev_bal_from >= amount, "Insufficient balance");

        let prev_allowance = self.allowance(env, from, spender);
        assert!(prev_allowance >= amount, "Insufficient allowance");

        let prev_bal_to = self.balance(&to);

        let new_bal_from = prev_bal_from.saturating_sub(amount);
        let new_bal_to = prev_bal_to.saturating_add(amount);

        let new_allowance = prev_allowance.saturating_sub(amount);

        token.balances[from.val as usize] = new_bal_from;
        token.balances[to.val as usize] = new_bal_to;
        token.allowances[from.val as usize][spender.val as usize] = new_allowance;

        self.env.storage.borrow_mut().update_token(token.clone());
    }

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    pub fn burn_from(&self, env: Env, spender: Address, from: Address, amount: i128) {
        let mut token = self.get_self_token();
        let prev_bal_from = self.balance(&from);

        assert!(prev_bal_from >= amount, "Insufficient balance");

        let prev_allowance = self.allowance(env, from, spender);
        assert!(prev_allowance >= amount, "Insufficient allowance");

        let new_bal_from = prev_bal_from.saturating_sub(amount);
        let new_allowance = prev_allowance.saturating_sub(amount);

        token.balances[from.val as usize] = new_bal_from;
        token.allowances[from.val as usize][spender.val as usize] = new_allowance;

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
        let token = self.env.storage.borrow().get_token(&self.address);
        if let Some(token) = token {
            token
        } else {
            panic!("Token not found")
        }
    }

    pub fn update_self_token(&self, token: &MockToken) {
        self.env.storage.borrow_mut().update_token(token.clone());
    }

    pub fn balance(&self, address: &Address) -> i128 {
        let token = self.get_self_token();
        token.balances[address.val as usize]
    }

    pub fn mint(&self, to: &Address, amount: &i128) {
        assert!(*amount > 0);

        let mut token = self.get_self_token();
        let prev_bal = self.balance(to);

        let new_bal = prev_bal.saturating_add(*amount);

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

pub trait TokenInterface {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address spending the tokens held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;
    fn verify_allowance() {}

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    /// `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address being authorized to spend the tokens held by
    ///   `from`.
    /// * `amount` - The tokens to be made available to `spender`.
    /// * `expiration_ledger` - The ledger number where this allowance expires. Cannot
    ///    be less than the current ledger number unless the amount is being set to 0.
    ///    An expired entry (where expiration_ledger < the current ledger number)
    ///    should be treated as a 0 amount allowance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["approve", from: Address,
    /// spender: Address], data = [amount: i128, expiration_ledger: u32]`
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);
    fn verify_approve() {}

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a balance is being queried. If the
    ///   address has no existing balance, returns 0.
    fn balance(env: Env, id: Address) -> i128;
    fn verify_balance() {}

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn verify_transfer() {}

    /// Transfer `amount` from `from` to `to`, consuming the allowance of
    /// `spender`. Authorized by spender (`spender.require_auth()`).
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the transfer, and having its
    ///   allowance consumed during the transfer.
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);
    fn verify_transfer_from() {}

    /// Burn `amount` from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be
    ///   burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = [amount:
    /// i128]`
    fn burn(env: Env, from: Address, amount: i128);
    fn verify_burn() {}

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the burn, and having its allowance
    ///   consumed during the burn.
    /// * `from` - The address holding the balance of tokens which will be
    ///   burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = [amount:
    /// i128]`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);
    fn verify_burn_from() {}

    /// Returns the number of decimals used to represent amounts of this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn decimals(env: Env) -> u32;
    fn verify_decimals() {}

    /// Returns the name for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn name(env: Env) -> String;
    fn verify_name() {}

    /// Returns the symbol for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn symbol(env: Env) -> String;
    fn verify_symbol() {}
}

pub trait StellarAssetInterface {
    /// Sets the administrator to the specified address `new_admin`.
    ///
    /// # Arguments
    ///
    /// * `new_admin` - The address which will henceforth be the administrator
    ///   of this token contract.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["set_admin", admin: Address], data =
    /// [new_admin: Address]`
    fn set_admin(env: Env, new_admin: Address);

    /// Returns the admin of the contract.
    ///
    /// # Panics
    ///
    /// If the admin is not set.
    fn admin(env: Env) -> Address;

    /// Sets whether the account is authorized to use its balance. If
    /// `authorized` is true, `id` should be able to use its balance.
    ///
    /// # Arguments
    ///
    /// * `id` - The address being (de-)authorized.
    /// * `authorize` - Whether or not `id` can use its balance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["set_authorized", id: Address], data =
    /// [authorize: bool]`
    fn set_authorized(env: Env, id: Address, authorize: bool);

    /// Returns true if `id` is authorized to use its balance.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which token authorization is being checked.
    fn authorized(env: Env, id: Address) -> bool;

    /// Mints `amount` to `to`.
    ///
    /// # Arguments
    ///
    /// * `to` - The address which will receive the minted tokens.
    /// * `amount` - The amount of tokens to be minted.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["mint", admin: Address, to: Address], data
    /// = [amount: i128]`
    fn mint(env: Env, to: Address, amount: i128);

    /// Clawback `amount` from `from` account. `amount` is burned in the
    /// clawback process.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance from which the clawback will
    ///   take tokens.
    /// * `amount` - The amount of tokens to be clawed back.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["clawback", admin: Address, to: Address],
    /// data = [amount: i128]`
    fn clawback(env: Env, from: Address, amount: i128);
}
