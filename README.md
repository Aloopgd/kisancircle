# Kisan Circle

On-chain savings-and-lending circle for farmer cooperatives, built on Stellar/Soroban.

## Problem

A 40-member rice farmer cooperative in Bogura, Bangladesh keeps its pooled savings in a
treasurer's tin box. Members lose two days' wages traveling to deposit or withdraw funds,
and disputes over who contributed what have dissolved neighboring cooperatives.

## Solution

Farmers deposit USDC (cashed in through a local mobile-money anchor) into a shared Soroban
contract from a basic smartphone. The contract enforces transparent deposit records and
collateralized micro-loans on-chain, so no treasurer can misreport balances. Stellar's
sub-cent fees and ~5-second settlement make frequent small-value transactions from dozens
of smallholders economically viable.

## Timeline (hackathon build)

- Day 1: Contract design, storage schema, `initialize`/`join_circle`/`deposit`
- Day 2: `request_loan`/`repay_loan` logic, collateral rules, tests
- Day 3: Frontend wallet flow, anchor cash-in/cash-out mock, demo polish

## Stellar Features Used

- USDC transfers (via local anchor on/off-ramp)
- Soroban smart contracts (savings + lending logic)
- Trustlines (anchor-issued USDC)

## Vision and Purpose

Replace informal ROSCA/samity savings groups — used by hundreds of millions of smallholder
farmers across South Asia — with a transparent, tamper-proof group ledger, while keeping
lending decisions instant and collateral-based rather than paperwork-based.

## Prerequisites

- Rust (`rustup` with the `wasm32-unknown-unknown` target)
- Soroban CLI v21+ (`cargo install --locked soroban-cli`)

## Build

```bash
soroban contract build
```

## Test

```bash
cargo test
```

## Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kisan_circle.wasm \
  --source <YOUR_IDENTITY> \
  --network testnet
```

## Sample CLI Invocation

Initialize the circle (contribution amount in stroops, 7 decimals — 5 USDC below):

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_IDENTITY> \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --contribution_amount 50000000
```

A farmer joins, deposits, and requests a loan:

```bash
soroban contract invoke --id <CONTRACT_ID> --source <FARMER_IDENTITY> --network testnet \
  -- join_circle --member <FARMER_ADDRESS>

soroban contract invoke --id <CONTRACT_ID> --source <FARMER_IDENTITY> --network testnet \
  -- deposit --member <FARMER_ADDRESS> --amount 100

soroban contract invoke --id <CONTRACT_ID> --source <FARMER_IDENTITY> --network testnet \
  -- request_loan --member <FARMER_ADDRESS> --amount 50
```

Link
 https://stellar.expert/explorer/testnet/tx/c11b82e1a9431340ecfe3f82f357567a361e578aaa8eefc898c96d96e39d7486
🔗 https://lab.stellar.org/r/testnet/contract/CARITB23I4FLZU573FM2J7IPYNICLD32BYUMZGVWAZ2WPKBK2EXZ267W


## License

MIT