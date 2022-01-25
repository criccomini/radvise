mod account;
mod engine;
mod holding;
mod security;

#[cfg(test)]
mod tests {
    use crate::engine::EngineConfig;
    use crate::holding::Holding;
    use crate::account::Account;
    use crate::account::AccountType;
    use crate::account::AccountSubtype;
    use crate::security::Security;
    use crate::security::SecurityType;
    use crate::security::WealthfrontAssetClass;
    use rust_decimal_macros::dec;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let mut targets = HashMap::new();
        targets.insert("VTSAX", dec!(.6));
        targets.insert("VGSLX", dec!(.4));
        let vanguard_brokerage = Account {
            name: "Vanguard Brokerage",
            r#type: AccountType::Investment,
            subtype: Some(AccountSubtype::Brokerage),
        };
        let vtsax_holding = Holding {
            security: Security {
                ticker_symbol: "VTSAX",
                r#type: SecurityType::MutualFund,
                asset_class: Some(WealthfrontAssetClass::EquityUs.to_string()),
            },
            quantity: dec!(1.0),
            purchase_price: dec!(1.0),
            current_price: dec!(2.0),
            current_price_as_of: None,
        };
        let mut config = EngineConfig::new();
        let rebalance_transactions = config
            .add_holding(&vanguard_brokerage, &vtsax_holding)
            .set_target(&vanguard_brokerage, &targets)
            .rebalance(&vanguard_brokerage);
        println!("{:?}", rebalance_transactions)
    }
}
