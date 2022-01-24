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
        targets.insert("VTSAX", dec!(1.0));
        let vanguard_brokerage = Account {
            name: "Vanguard Brokerage".to_string(),
            r#type: AccountType::Investment,
            subtype: Some(AccountSubtype::Brokerage),
        };
        let engine_config = EngineConfig::new()
            .add_holding(&vanguard_brokerage, &Holding {
                security: Security {
                    ticker_symbol: "VTSAX".to_string(),
                    r#type: SecurityType::MutualFund,
                    asset_class: Some(WealthfrontAssetClass::EquityUs.to_string()),
                },
                quantity: dec!(1.0),
                purchase_price: dec!(1.0),
                current_price: dec!(2.0), // TODO probably move this to a separate method .set_price()
                current_price_as_of: None,
            })
            .set_target(&vanguard_brokerage, &targets);
    }
}
