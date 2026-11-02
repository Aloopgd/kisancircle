# Sentinel's Journal

Critical security learnings and patterns discovered during development.

## 2025-05-18 - Circle Membership Enforcement on Soroban Smart Contracts
**Vulnerability:** Functions `deposit` and `request_loan` authenticated the transaction signer via `member.require_auth()`, but did not check whether `member` was an enrolled circle member in contract storage. Non-members could deposit funds or attempt actions bypassing membership registration.
**Learning:** In Soroban contracts, address authentication (`require_auth`) confirms caller identity, but does not imply contract-level authorization or state enrollment. Operations that assume entity state (such as membership lists) must explicitly assert existence in persistent/instance storage.
**Prevention:** Implement explicit membership check helpers (e.g. `require_member`) before executing domain logic for registered roles.
