// Subset of: https://plaid.com/docs/api/products/#investments-holdings-get-response-securities-type
enum SecurityType {
    Cash,
    Derivative,
    Equity,
    Etf,
    Fixed,
    Loan,
    MutualFund,
    Other,
}

// Wealthfront assett classes: https://research.wealthfront.com/whitepapers/investment-methodology/
enum WealthfrontAssetClass {
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
enum BettermentAssetClass {
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

pub struct Security {
    pub ticker_symbol: String,
    pub type: SecurityType,
    // Left as a string to allow arbitrary asset classes
    pub asset_class: Option<String>,
}