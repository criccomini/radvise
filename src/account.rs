enum AccountType {
    Investment,
    Credit,
    Depository,
    Loan,
    Other,
}

// Subset of https://plaid.com/docs/api/products/#investments-holdings-get-response-accounts-subtype
enum AccountSubtype {
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

pub struct Account {
    pub type: AccountType,
    pub subtype: Option<AccountSubtype>,
    pub holdings: Vec<Holding>,
}
