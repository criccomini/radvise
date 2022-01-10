# radvise

## Account Hierachy

`radvise` uses a a subset of the [Plaid data models](https://plaid.com/docs/api/products/#investments) for institutions, portfolios, accounts, holdings, and securities; it uses [Wealthfront data models](https://research.wealthfront.com/whitepapers/investment-methodology/#2-finding_asset_classes) for asset classes.

```
institution
└── portfolio
    └── account
        └── holding
```