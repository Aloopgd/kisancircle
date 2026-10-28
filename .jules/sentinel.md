## 2025-02-13 - Uninitialized State Execution in Soroban Smart Contracts
**Vulnerability:** State-mutating functions (`join_circle`, `deposit`, `request_loan`, `repay_loan`) could be invoked prior to calling `initialize`.
**Learning:** In Soroban smart contracts, calling state-mutating functions before `initialize` populates storage with default fallback values (e.g. `unwrap_or_default`), which are subsequently overwritten when `initialize` runs, causing state corruption/loss (e.g., reset of `TotalPool` and `Members`).
**Prevention:** Guard all contract entrypoints with a `require_initialized` check that panics if the admin/initialization flag is missing from storage, and validate initialization inputs (e.g., `contribution_amount > 0`).
