use rust_decimal::Decimal;
use std::collections::HashMap;
use crate::account::Account;
use crate::holding::Holding;

pub struct EngineConfig<'a> {
    balances: HashMap<&'a Account, Vec<&'a Holding>>,
    targets: HashMap<&'a Account, HashMap<&'a str, Decimal>>,
}

impl <'a> EngineConfig<'a> {
    pub fn new() -> EngineConfig<'a> {
        EngineConfig {
            balances: HashMap::new(),
            targets: HashMap::new(),
        }
    }

    pub fn add_holding(&'a mut self, account: &'a Account, holding: &'a Holding) -> &'a mut EngineConfig {
        self.balances
            .entry(account)
            .or_insert(Vec::new())
            .append(&mut Vec::from([holding]));
        self
    }

    pub fn set_target(&'a mut self, account: &'a Account, targets: &'a HashMap<&'a str, Decimal>) {
    }

    /*
    pub fn add_security() {
    }

    pub fn add_rule() {
    }

    pub fn add_income() {
    }

    pub fn add_expense() {
    }

    pub fn add_tax() {
    }

    pub fn add_transaction() {
    }
    */
}

//fn rebalance(transactions: Vec<Transaction>, config: EngineConfig) -> Vec<Transaction> {
//}