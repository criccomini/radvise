#[derive(PartialEq, Eq, Hash)]
pub enum AccountType {
    Investment,
    Credit,
    Depository,
    Loan,
    Other,
}

// Subset of https://plaid.com/docs/api/products/#investments-holdings-get-response-accounts-subtype
#[derive(PartialEq, Eq, Hash)]
pub enum AccountSubtype {
    Retirement401k,
    Education529,
    Brokerage,
    Ira,
    Roth,
    CreditCard,
    Checking,
    Savings,
    Mortgage,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Account {
    pub name: String,
    pub r#type: AccountType,
    pub subtype: Option<AccountSubtype>,
}
