use rather_staking::*;
use multiversx_sc::{codec::multi_types::OptionalValue, types::Address};
use multiversx_sc_scenario::{managed_address, rust_biguint, managed_biguint, testing_framework::*, DebugApi};

const WASM_PATH: &str = "output/rather-staking.wasm";
const USER_BALANCE: u64 = 1_000_000_000_000_000_000;
const RS_TIMESTAMP: u64 = 1686000640;

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> rather_staking::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub contract_wrapper: ContractObjWrapper<rather_staking::ContractObj<DebugApi>, ContractObjBuilder>,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> rather_staking::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let first_user_address = blockchain_wrapper.create_user_account(&rust_biguint!(USER_BALANCE));
    let second_user_address = blockchain_wrapper.create_user_account(&rust_biguint!(USER_BALANCE));

    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            let rate: u64 = 1000000000000000_u64;
            sc.init(rate);
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        contract_wrapper: cf_wrapper,
    }
}

#[test]
fn deploy_test() {
    let mut setup = setup_contract(rather_staking::contract_obj);

    // simulate deploy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let rate: u64 = 1000000000000000_u64;
                sc.init(rate);
            },
        )
        .assert_ok();
}

#[test]
fn stake_test() {
    let mut setup = setup_contract(rather_staking::contract_obj);
    let user_addr = setup.first_user_address.clone();

    setup
        .blockchain_wrapper
        .check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    setup
        .blockchain_wrapper
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));

    // stake full
    setup
        .blockchain_wrapper
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USER_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    managed_biguint!(USER_BALANCE)
                );
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .check_egld_balance(&user_addr, &rust_biguint!(0));
    setup.blockchain_wrapper.check_egld_balance(
        setup.contract_wrapper.address_ref(),
        &rust_biguint!(USER_BALANCE),
    );
}

#[test]
fn unstake_test() {
    let mut setup = setup_contract(rather_staking::contract_obj);
    let user_addr = setup.first_user_address.clone();

    // stake
    setup
        .blockchain_wrapper
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USER_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    managed_biguint!(USER_BALANCE)
                );
            },
        )
        .assert_ok();

    // unstake full
    setup
        .blockchain_wrapper
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unstake(OptionalValue::None);

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    managed_biguint!(0)
                );
            },
        )
        .assert_ok();
    
    setup
        .blockchain_wrapper
        .check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    setup
        .blockchain_wrapper
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));
}

#[test]
fn unstake_partial_test() {
    let mut setup = setup_contract(rather_staking::contract_obj);
    let user_addr = setup.first_user_address.clone();

    // stake
    setup
        .blockchain_wrapper
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USER_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    managed_biguint!(USER_BALANCE)
                );
            },
        )
        .assert_ok();

    // unstake partial
    setup
        .blockchain_wrapper
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unstake(OptionalValue::Some(managed_biguint!(USER_BALANCE / 2)));

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    managed_biguint!(USER_BALANCE / 2)
                );
            },
        )
        .assert_ok();
    
    setup
        .blockchain_wrapper
        .check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE / 2));
    setup.blockchain_wrapper.check_egld_balance(
        setup.contract_wrapper.address_ref(),
        &rust_biguint!(USER_BALANCE / 2),
    );
}

#[test]
fn claim_rewards_test() {
    let mut setup = setup_contract(rather_staking::contract_obj);
    let user_addr = setup.first_user_address.clone();

    setup
        .blockchain_wrapper.set_block_timestamp(RS_TIMESTAMP);

    // stake
    setup
        .blockchain_wrapper
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USER_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    managed_biguint!(USER_BALANCE)
                );
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper.set_block_timestamp(RS_TIMESTAMP+1);

    // claim rewards
    setup
        .blockchain_wrapper
        .execute_tx(
            &user_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_rewards();

                assert_eq!(
                    sc.staking_position(&managed_address!(&user_addr)).get(),
                    managed_biguint!(USER_BALANCE)
                );
            },
        )
        .assert_ok();
    
    
    setup
        .blockchain_wrapper
        .check_egld_balance(&user_addr, &rust_biguint!(1000000000000000));
}

#[test]
fn unstake_claim_rewards_test() {
    let mut setup = setup_contract(rather_staking::contract_obj);
    let blockchain_wrapper = &mut setup.blockchain_wrapper;
    let user_addr = setup.first_user_address.clone();

    // stake
    blockchain_wrapper
    .execute_tx(
        &user_addr,
        &setup.contract_wrapper,
        &rust_biguint!(USER_BALANCE),
        |sc| {
            sc.stake();

            assert_eq!(
                sc.staking_position(&managed_address!(&user_addr)).get(),
                managed_biguint!(USER_BALANCE)
            );
        },
    )
    .assert_ok();


   blockchain_wrapper.set_block_timestamp(RS_TIMESTAMP+1);


    // unstake full
    blockchain_wrapper
    .execute_tx(
        &user_addr,
        &setup.contract_wrapper,
        &rust_biguint!(0),
        |sc| {
            sc.unstake(OptionalValue::None);

            assert_eq!(
                sc.staking_position(&managed_address!(&user_addr)).get(),
                managed_biguint!(0)
            );
        },
    )
    .assert_ok();
    
    blockchain_wrapper
    .check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    
    blockchain_wrapper
    .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));

    // claim rewards
    let tx_result = blockchain_wrapper
    .execute_tx(
        &user_addr,
        &setup.contract_wrapper,
        &rust_biguint!(0),
        |sc| {
            sc.claim_rewards();

            assert_eq!(
                sc.staking_position(&managed_address!(&user_addr)).get(),
                managed_biguint!(USER_BALANCE)
            );
        },
    );
    
    
    blockchain_wrapper
        .check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
}
