## 2024-10-20 - Soroban Uninitialized State Reset
**Vulnerability:** Operations on an uninitialized Soroban contract executed using default fallbacks (`unwrap_or`), allowing state creation before `initialize()`. A subsequent call to `initialize()` reset `TotalPool` and `Members` instance storage keys, causing severe state and accounting corruption.
**Learning:** In Soroban smart contracts, default fallbacks on instance storage queries permit uninitialized execution unless an explicit initialization guard check is enforced.
**Prevention:** Enforce `require_initialized(&env)` at the entry point of all state-mutating functions prior to reading or setting instance/persistent storage.
