#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Env;

fn setup(env: &Env) -> (KisanCircleClient<'_>, Address, Address) {
    let contract_id = env.register_contract(None, KisanCircle);
    let client = KisanCircleClient::new(env, &contract_id);
    let admin = Address::generate(env);
    let farmer = Address::generate(env);

    client.initialize(&admin, &5_0000000i128); // 5 USDC weekly contribution (7 decimals)
    (client, admin, farmer)
}

// Test 1 (Happy path): a farmer joins, deposits, requests a loan
// within their 80% collateral limit, and repays it end-to-end.
#[test]
fn test_deposit_and_loan_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _admin, farmer) = setup(&env);

    client.join_circle(&farmer);
    client.deposit(&farmer, &100i128);
    client.request_loan(&farmer, &50i128); // 50% of 100, within 80% cap
    client.repay_loan(&farmer, &50i128);

    let loan = client.get_loan(&farmer);
    assert_eq!(loan.active, false);
    assert_eq!(loan.outstanding, 0);
}

// Test 2 (Edge case): requesting a loan above 80% of the member's
// savings must fail.
#[test]
#[should_panic(expected = "loan exceeds 80% of savings")]
fn test_loan_exceeds_collateral_limit_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _admin, farmer) = setup(&env);

    client.join_circle(&farmer);
    client.deposit(&farmer, &100i128);
    client.request_loan(&farmer, &90i128); // exceeds 80 cap
}

// Test 3 (State verification): after a deposit and a loan request,
// the member balance, loan record, and shared pool must all reflect
// the correct resulting state.
#[test]
fn test_state_after_deposit_and_loan() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _admin, farmer) = setup(&env);

    client.join_circle(&farmer);
    client.deposit(&farmer, &200i128);
    client.request_loan(&farmer, &100i128);

    assert_eq!(client.get_balance(&farmer), 200i128);

    let loan = client.get_loan(&farmer);
    assert_eq!(loan.principal, 100i128);
    assert_eq!(loan.outstanding, 100i128);
    assert_eq!(loan.active, true);

    // Pool started at 200 (the deposit) and 100 was disbursed as a loan.
    assert_eq!(client.get_pool(), 100i128);
}

// Test 4 (Edge case): a member cannot open a second loan while one
// is still active.
#[test]
#[should_panic(expected = "existing loan must be repaid first")]
fn test_duplicate_active_loan_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _admin, farmer) = setup(&env);

    client.join_circle(&farmer);
    client.deposit(&farmer, &200i128);
    client.request_loan(&farmer, &50i128);
    client.request_loan(&farmer, &50i128); // second loan while first is active
}

// Test 5 (State verification): fully repaying a loan closes it and
// restores the repaid amount to the shared pool's liquidity.
#[test]
fn test_full_repayment_restores_pool_liquidity() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _admin, farmer) = setup(&env);

    client.join_circle(&farmer);
    client.deposit(&farmer, &200i128);
    client.request_loan(&farmer, &100i128);
    // Pool is now 100 (200 deposited - 100 loaned out).
    client.repay_loan(&farmer, &100i128);

    let loan = client.get_loan(&farmer);
    assert_eq!(loan.active, false);
    assert_eq!(loan.outstanding, 0);
    assert_eq!(client.get_pool(), 200i128); // fully restored
}

// Test 6 (Security): a non-member cannot deposit funds.
#[test]
#[should_panic(expected = "not a member")]
fn test_deposit_non_member_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _admin, _farmer) = setup(&env);
    let non_member = Address::generate(&env);

    client.deposit(&non_member, &100i128);
}

// Test 7 (Security): a non-member cannot request a loan.
#[test]
#[should_panic(expected = "not a member")]
fn test_loan_non_member_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _admin, farmer) = setup(&env);
    client.join_circle(&farmer);
    client.deposit(&farmer, &100i128);

    let non_member = Address::generate(&env);
    client.request_loan(&non_member, &50i128);
}
