# Sentinel Security Journal

## 2025-05-18 - Circle Membership Verification on Deposit
**Vulnerability:** The `deposit` function only verified caller signature (`member.require_auth()`) but failed to check if the caller was an enrolled circle member (`DataKey::Members`). Unregistered addresses could deposit and alter circle state without completing enrollment.
**Learning:** In smart contract architectures that separate registration/enrollment (`join_circle`) from operational actions (`deposit`, `request_loan`), every state-mutating function must explicitly validate registration against the membership registry rather than relying solely on cryptographic caller authentication.
**Prevention:** Enforce membership checks on all member-specific smart contract entry points prior to mutating internal balances or pool liquidity.
