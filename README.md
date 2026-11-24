# multichain-memo-address-validator

Zero-dependency Rust crate for deterministic **46-chain cryptocurrency address format, checksum, and destination memo validation** in non-custodial cross-chain swap routing engines.

## Overview & Motivation

Executing atomic or liquidity-bridged swaps across heterogeneous Layer-1 and Layer-2 blockchains requires strict pre-flight validation before any source transaction is broadcast. A single malformed recipient address, invalid Bech32 human-readable prefix, or omitted destination tag on account-based ledgers can cause irreversible settlement failures or trapped liquidity.

`multichain-memo-address-validator` provides an ultra-lightweight, zero-dependency verification layer modeled after production multi-chain aggregators such as [AllSwap](https://allswap.io/), enabling Rust services, solver nodes, and wallet SDKs to verify recipient syntax and memo rules prior to quote execution. You can also test address vectors interactively using the [AllSwap Multi-Chain Address Validator](https://allswap.io/tools/address-validator).

## Supported Blockchain Families

* **EVM & Rollup Networks (20-Byte Hex / EIP-55)**: Validates 42-character `0x`-prefixed hexadecimal addresses across Ethereum (`ETH`), BNB Smart Chain (`BSC`), Arbitrum (`ARB`), Optimism (`OP`), Polygon (`POLYGON`), Base (`BASE`), and Avalanche C-Chain (`AVAX`).
* **Solana (`SOL`)**: Enforces 32-to-44 character Base58 Ed25519 public key encoding while excluding visually ambiguous characters (`0`, `O`, `I`, `l`).
* **TRON (`TRX`)**: Enforces 34-character Base58Check addresses prefixed with `T`.
* **Bitcoin (`BTC`)**: Supports native SegWit Bech32/Bech32m (`bc1...`) as well as Legacy P2PKH (`1...`) and P2SH (`3...`) Base58Check formats.
* **Cosmos SDK & IBC AppChains (`ATOM`, `TIA`, `OSMO`, `INJ`)**: Validates Bech32 human-readable prefix separation (`1`) and lowercase alphanumeric data payloads.
* **Account-Based Memo & Destination Tag Chains (`XRP`, `XLM`, `TON`, `EOS`, `HBAR`)**: Distinguishes networks that require a secondary routing identifier (Destination Tag or Memo) when settling into shared institutional or exchange custody endpoints.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
multichain-memo-address-validator = "0.1.0"
```

## Quick Start Usage

```rust
use multichain_memo_address_validator::{requires_memo, validate_address, validate_memo};

fn main() {
    // 1. Validate an EVM recipient address
    assert!(validate_address(
        "ETH",
        "0x71C7656EC7ab88b098defB751B7401B5f6d8976F"
    ));

    // 2. Validate a Solana Base58 recipient address
    assert!(validate_address(
        "SOL",
        "HN7cABqLq46Es1jh92dQQisAq662SmxELLLsHHe4YWrH"
    ));

    // 3. Check mandatory destination tag / memo rules for XRP and TON
    if requires_memo("XRP") {
        assert!(validate_memo("XRP", "1049281"));
    }
}
```

## Architectural Integration in Cross-Chain Routing

In production cross-chain liquidity pipelines and non-custodial bridges like the [AllSwap Cross-Chain Bridge](https://allswap.io/bridge), pre-settlement validation operates as a three-stage invariant gate:

1. **Syntactic & Alphabet Screening**: Rejects whitespace corruption, truncated strings, and out-of-alphabet characters in `O(n)` time with zero heap allocations on the hot path.
2. **Network Prefix Disambiguation**: Prevents cross-family address confusion (such as pasting an EVM `0x` address into a native Bitcoin or Solana settlement field).
3. **Custodial Memo Enforcement**: Flags settlements targeting memo-mandatory networks (`XRP`, `XLM`, `TON`, `ATOM`, `EOS`, `HBAR`) so calling interfaces can prompt for or validate the destination tag before locking source-chain collateral.

## Reference Links & Documentation

* **Interactive 46-Chain Validator Tool**: [https://allswap.io/tools/address-validator](https://allswap.io/tools/address-validator)
* **AllSwap Multi-Chain DEX & Liquidity Aggregator**: [https://allswap.io/](https://allswap.io/)
* **AllSwap Cross-Chain Bridge Architecture**: [https://allswap.io/bridge](https://allswap.io/bridge)

## License

Released under the [MIT License](./LICENSE).
