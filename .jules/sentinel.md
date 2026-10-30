# Sentinel Security Journal

## 2026-03-30 - Uninitialized Contract Methods & Unregistered Deposit Balance Reset
**Vulnerability:** In Soroban smart contracts, methods that interact with persistent or instance storage without verifying prior admin initialization or membership status can allow operations on unitialized state or lead to accidental balance resets. Specifically, allowing `deposit` without checking membership permitted non-members to accumulate balances, which were subsequently wiped out when `join_circle` initialized their `Balance` entry back to `0i128`.
**Learning:** Smart contracts relying on multi-step onboarding workflows (e.g., initialization -> membership -> deposit/loan) must guard against out-of-order execution and missing initialization assertions across all write endpoints.
**Prevention:** Always enforce `require_initialized` and strict state invariants (e.g., `require_member`) on write functions before reading or mutating state storage keys.
