// Subset of: https://plaid.com/docs/api/products/#investments-holdings-get-response-securities-type
pub enum SecurityType {
    Cash,
    Derivative,
    Equity,
    Etf,
    Fixed,
    Loan,
    MutualFund,
    Other,
}

// Wealthfront asset classes: https://research.wealthfront.com/whitepapers/investment-methodology/
#[derive(strum_macros::Display)]
pub enum WealthfrontAssetClass {
    BondsUs,
    BondsUsCorporate,
    BondsUsMuni,
    BondsUsTips,
    BondsNonUsEmerging,
    Commodities,
    EquityUs,
    EquityUsDividend,
    EquityNonUsDeveloped,
    EquityNonUsEmerging,
    RealEstate,
}

// Betterment asset classes: https://www.betterment.com/help/core-portfolio-funds
pub enum BettermentAssetClass {
    EquityUs,
    EquityUsLargeCap,
    EquityUsMidCap,
    EquityUsSmallCap,
    EquityNonUsDeveloped,
    EquityNonUsEmerging,
    BondsUsHighQuality,
    BondsUsMuni,
    BondsUsTips,
    BondsUsShortTermTreasury,
    BondsUsShortTermInvestmentGrade,
    BondsNonUsDeveloped,
    BondsNonUsEmerging,
}

pub struct Security<'a> {
    pub ticker_symbol: &'a str,
    pub r#type: SecurityType,
    // Left as a string to allow arbitrary asset classes
    pub asset_class: Option<String>,
}