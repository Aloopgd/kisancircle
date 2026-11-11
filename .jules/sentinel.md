## 2025-05-18 - Enforce Cooperative Membership Verification on Deposits
**Vulnerability:** The `deposit` function only required the caller's authorization and verified positive amounts, but failed to check if the caller had actually joined the savings circle via `join_circle`. Non-members could deposit funds directly, bypassing cooperative member registry tracking (`DataKey::Members`).
**Learning:** Default-or-fallback getters like `get(&balance_key).unwrap_or(0)` can inadvertently initialize state for unregistered entities in Soroban contracts if explicit existence checks are not performed before state writes.
**Prevention:** Check for enrollment (e.g. `env.storage().persistent().has(&balance_key)`) on all member action entrypoints before modifying ledger states.
