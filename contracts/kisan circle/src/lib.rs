#![no_std]

// Kisan Circle: an on-chain savings-and-lending circle for a farmer
// cooperative. Members deposit fixed contributions, build a visible
// on-chain balance, and can borrow up to 80% of their own savings
// from the shared pool. This removes the two failure modes of an
// informal "samity" group: travel cost to a physical treasurer, and
// disputes over who contributed what.
//
// NOTE: For clarity in a hackathon/demo build, balances are tracked
// as internal ledger amounts (i128) rather than moving a real token.
// In production, deposit/repay/loan disbursement would call into the
// USDC Stellar Asset Contract via `soroban_sdk::token::Client` so
// funds actually move on-chain instead of only being recorded.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContributionAmount,
    TotalPool,
    Members,
    Balance(Address),
    Loan(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct LoanInfo {
    pub principal: i128,
    pub outstanding: i128,
    pub active: bool,
}

#[contract]
pub struct KisanCircle;

fn require_initialized(env: &Env) {
    // Security check: ensure contract is initialized before allowing state mutations
    if !env.storage().instance().has(&DataKey::Admin) {
        panic!("contract not initialized");
    }
}

#[contractimpl]
impl KisanCircle {
    /// Sets up the circle: records the admin (cooperative coordinator)
    /// and the standard weekly contribution amount. Can only run once.
    pub fn initialize(env: Env, admin: Address, contribution_amount: i128) {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        if contribution_amount <= 0 {
            panic!("contribution amount must be positive");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::ContributionAmount, &contribution_amount);
        env.storage().instance().set(&DataKey::TotalPool, &0i128);

        let members: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Members, &members);
    }

    /// Registers a farmer as a member of the circle, giving them a
    /// zeroed savings balance to start contributing to.
    pub fn join_circle(env: Env, member: Address) {
        require_initialized(&env);
        member.require_auth();

        let mut members: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Members)
            .unwrap_or(Vec::new(&env));

        if !members.contains(&member) {
            members.push_back(member.clone());
            env.storage().instance().set(&DataKey::Members, &members);
            env.storage()
                .persistent()
                .set(&DataKey::Balance(member.clone()), &0i128);
        }
    }

    /// Records a member's deposit into their savings balance and adds
    /// it to the shared pool available for loans. This is the action
    /// a farmer takes weekly instead of walking cash to a treasurer.
    pub fn deposit(env: Env, member: Address, amount: i128) {
        require_initialized(&env);
        member.require_auth();
        if amount <= 0 {
            panic!("amount must be positive");
        }

        let balance_key = DataKey::Balance(member.clone());
        let mut balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&balance_key, &balance);

        let mut pool: i128 = env.storage().instance().get(&DataKey::TotalPool).unwrap_or(0);
        pool += amount;
        env.storage().instance().set(&DataKey::TotalPool, &pool);
    }

    /// Lets a member borrow against their own savings, capped at 80%
    /// of their balance, and only if the shared pool has liquidity.
    /// A member cannot open a second loan while one is still active.
    pub fn request_loan(env: Env, member: Address, amount: i128) {
        require_initialized(&env);
        member.require_auth();
        if amount <= 0 {
            panic!("amount must be positive");
        }

        let balance_key = DataKey::Balance(member.clone());
        let balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        let max_loan = balance * 80 / 100;
        if amount > max_loan {
            panic!("loan exceeds 80% of savings");
        }

        let loan_key = DataKey::Loan(member.clone());
        if let Some(existing) = env.storage().persistent().get::<DataKey, LoanInfo>(&loan_key) {
            if existing.active {
                panic!("existing loan must be repaid first");
            }
        }

        let mut pool: i128 = env.storage().instance().get(&DataKey::TotalPool).unwrap_or(0);
        if amount > pool {
            panic!("insufficient pool liquidity");
        }
        pool -= amount;
        env.storage().instance().set(&DataKey::TotalPool, &pool);

        let loan = LoanInfo {
            principal: amount,
            outstanding: amount,
            active: true,
        };
        env.storage().persistent().set(&loan_key, &loan);
    }

    /// Applies a repayment toward a member's active loan. Any amount
    /// repaid flows back into the shared pool, making it available
    /// for other members' loans. Fully repaying closes the loan.
    pub fn repay_loan(env: Env, member: Address, amount: i128) {
        require_initialized(&env);
        member.require_auth();
        if amount <= 0 {
            panic!("amount must be positive");
        }

        let loan_key = DataKey::Loan(member.clone());
        let mut loan: LoanInfo = env
            .storage()
            .persistent()
            .get(&loan_key)
            .expect("no active loan");
        if !loan.active {
            panic!("no active loan");
        }

        let applied = if amount >= loan.outstanding {
            let repaid = loan.outstanding;
            loan.outstanding = 0;
            loan.active = false;
            repaid
        } else {
            loan.outstanding -= amount;
            amount
        };

        let mut pool: i128 = env.storage().instance().get(&DataKey::TotalPool).unwrap_or(0);
        pool += applied;
        env.storage().instance().set(&DataKey::TotalPool, &pool);

        env.storage().persistent().set(&loan_key, &loan);
    }

    /// Read-only: a member's current savings balance.
    pub fn get_balance(env: Env, member: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(member))
            .unwrap_or(0)
    }

    /// Read-only: a member's current loan state.
    pub fn get_loan(env: Env, member: Address) -> LoanInfo {
        env.storage()
            .persistent()
            .get(&DataKey::Loan(member))
            .unwrap_or(LoanInfo {
                principal: 0,
                outstanding: 0,
                active: false,
            })
    }

    /// Read-only: total liquidity currently available in the shared pool.
    pub fn get_pool(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalPool).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;