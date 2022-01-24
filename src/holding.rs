use rust_decimal::Decimal;
use crate::security::Security;

pub struct Holding {
    pub security: Security,
    pub quantity: Decimal,
    pub purchase_price: Decimal,
    pub current_price: Decimal,
    pub current_price_as_of: Option<u64>,
}