## 2026-10-22 - Checked Arithmetic for Soroban Smart Contract Loan Calculations
**Vulnerability:** Unchecked arithmetic during loan cap calculation (`balance * 80 / 100`) in `request_loan` could cause integer overflow when balance is very large.
**Learning:** In Rust/Soroban contracts, standard arithmetic operators like `*` can panic or overflow in debug/release depending on build flags if values exceed integer type bounds.
**Prevention:** Always use safe checked arithmetic (`checked_mul`, `checked_div`, `checked_add`, `checked_sub`) when performing financial calculations on user balances and loan amounts.
