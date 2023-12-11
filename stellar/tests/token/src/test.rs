#![cfg(test)]
extern crate std;

use crate::{
    admin::{read_administrator, write_administrator},
    balance::{read_balance, receive_balance},
    storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{Address, Env};

#[test]
fn test() {
    let e = Env::default();
    let to = Address::new(&e);
    let amount = 100;

    let init_balance = read_balance(&e, to);

    receive_balance(&e, to, amount);

    let post_balance = read_balance(&e, to);
}
