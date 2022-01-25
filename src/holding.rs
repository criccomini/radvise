use rust_decimal::Decimal;
use crate::security::Security;

pub struct Holding<'a> {
    pub security: Security<'a>,
    pub quantity: Decimal,
    pub purchase_price: Decimal,
    pub current_price: Decimal,
    pub current_price_as_of: Option<u64>,
}