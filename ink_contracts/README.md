# Ink! Smart Contract Libraries and Templates

This directory provides a collection of pre-built smart contract libraries and templates for the Ink! programming language. These libraries can be used to accelerate development and encourage code reuse.

## Included Libraries

The following smart contract libraries are currently included:

* BalanceManager: A library for managing balances in Ink! contracts.
* TokenManager: A library for managing fungible tokens in Ink! contracts.
* NFTManager: A library for managing non-fungible tokens (NFTs) in Ink! contracts.

## Using the Libraries

To use a library in your Ink! contract, add the following line to your contract's Cargo.toml file:

```toml
[dependencies]
ink-contracts = { path = "../ink_contracts" }
