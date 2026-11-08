# Sentinel Security Journal

## 2025-05-18 - Missing Membership Verification on Cooperative Operations
**Vulnerability:** In `KisanCircle`, `deposit` and `request_loan` verified the caller's signature via `require_auth()` but did not verify whether the caller had joined the circle. This allowed non-members to create unindexed savings balances or borrow pooled funds without being registered on the cooperative membership roster.
**Learning:** In Soroban contracts that maintain participant lists (`DataKey::Members`) alongside individual user storage (`DataKey::Balance(Address)`), caller authorization alone (`require_auth()`) does not guarantee authorization within the contract domain model. State mutations should always verify participant registration.
**Prevention:** Check membership state (e.g. `env.storage().persistent().has(&DataKey::Balance(member))`) on state mutations to ensure contract actions are strictly restricted to registered participants.
