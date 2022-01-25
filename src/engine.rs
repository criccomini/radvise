use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use crate::account::Account;
use crate::holding::Holding;

pub struct EngineConfig<'a> {
    balances: HashMap<&'a Account<'a>, Vec<&'a Holding<'a>>>,
    targets: HashMap<&'a Account<'a>, &'a HashMap<&'a str, Decimal>>,
}

impl <'a> EngineConfig<'a> {
    pub fn new() -> EngineConfig<'a> {
        EngineConfig {
            balances: HashMap::new(),
            targets: HashMap::new(),
        }
    }

    pub fn add_holding(&'a mut self, account: &'a Account, holding: &'a Holding<'a>) -> &'a mut EngineConfig {
        self.balances
            .entry(account)
            .or_insert(Vec::new())
            .append(&mut Vec::from([holding]));
        self
    }

    pub fn set_target(&'a mut self, account: &'a Account, targets: &'a HashMap<&'a str, Decimal>) -> &'a mut EngineConfig {
        self.targets
            .insert(account, targets);
        self
    }

    pub fn rebalance(&'a self, account: &'a Account) -> HashMap<&'a str, Decimal>{
        let zero = &dec!(0); // Need a long-lived zero to use as a default for unheled security balances
        let total_balance: Decimal = self.balances
            .get(account)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|h| h.quantity * h.current_price)
            .sum();
        let target_balances: HashMap<&'a str, Decimal>  = self.targets
            .get(account)
            .unwrap_or(&&HashMap::new())
            .iter()
            .map(|(k, v)| (*k, v * total_balance))
            .collect();
        let mut rebalance_transactions = target_balances.clone();
        for holding in self.balances.get(account).unwrap_or(&Vec::new()) {
            let ticker_symbol = holding.security.ticker_symbol;
            let target_balance = target_balances.get(ticker_symbol).unwrap_or(zero);
            let current_balance = holding.quantity * holding.current_price;
            rebalance_transactions.insert(ticker_symbol, target_balance - current_balance);
        }
        rebalance_transactions
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