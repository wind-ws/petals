use solana_program_test::*;
use solana_sdk::{
    address_lookup_table::instruction,
    feature_set::rent_for_sysvars,
    instruction::AccountMeta,
    msg, native_token,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::Signer,
    system_program, sysvar,
    transaction::{self, Transaction},
};

use crate::{
    entrypoint::process_instruction,
    instruction::Instruction,
    instructions::{
        InsAirDrop, InsCreateTokenAccount, InsCreateUserInfo, InsInitProgram,
    },
    state::{
        mint_rmb::{self, MintRmb},
        raise_fund::{self, PhoneNumber, RaiseFundList, UserInfo},
    },
};

#[tokio::test]
async fn test_none() {
    let program_id = Pubkey::new_unique();
    let (banks_client, payer, recent_blockhash) =
        ProgramTest::new("Ins ", program_id, processor!(process_instruction))
            .start()
            .await;
    let instruction = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::None,
        vec![],
    );
    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    let transaction_result =
        banks_client.process_transaction(transaction).await;
    assert!(transaction_result.is_ok());
}

#[tokio::test]
async fn test_init_mint_rmb() {
    let program_id = Pubkey::new_unique();
    let (banks_client, payer, recent_blockhash) =
        ProgramTest::new("Ins ", program_id, processor!(process_instruction))
            .start()
            .await;
    let mint_rmb_pda = MintRmb::pda(&program_id);
    let raise_fund_list = RaiseFundList::pda(&program_id);
    let instruction1 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::InitProgram(InsInitProgram::new()),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint_rmb_pda.0, false),
            AccountMeta::new(raise_fund_list.0, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
    );

    let mut transaction =
        Transaction::new_with_payer(&[instruction1], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    let transaction_result =
        banks_client.process_transaction(transaction).await;

    assert!(transaction_result.is_ok());
}

#[tokio::test]
async fn test_create_token_account() {
    let program_id = Pubkey::new_unique();
    let (banks_client, payer, recent_blockhash) =
        ProgramTest::new("Ins ", program_id, processor!(process_instruction))
            .start()
            .await;
    let mint_rmb = MintRmb::pda(&program_id);
    let token_rmb =
        MintRmb::token_account(&program_id, &payer.pubkey(), &mint_rmb.0);
    let raise_fund_list = RaiseFundList::pda(&program_id);
    let instruction1 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::InitProgram(InsInitProgram::new()),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint_rmb.0, false),
            AccountMeta::new(raise_fund_list.0, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
    );

    let instruction2 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::CreateTokenAccount(InsCreateTokenAccount {
            airdrop: 123,
        }),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(mint_rmb.0, false),
            AccountMeta::new_readonly(payer.pubkey(), false),
            AccountMeta::new(token_rmb.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(
        &[instruction1, instruction2],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    let transaction_result =
        banks_client.process_transaction(transaction).await;

    assert!(transaction_result.is_ok());
}

#[tokio::test]
async fn test_air_drop() {
    let program_id = Pubkey::new_unique();
    let (banks_client, payer, recent_blockhash) =
        ProgramTest::new("Ins ", program_id, processor!(process_instruction))
            .start()
            .await;
    let mint_rmb = MintRmb::pda(&program_id);
    let token_rmb =
        MintRmb::token_account(&program_id, &payer.pubkey(), &mint_rmb.0);

    let raise_fund_list = RaiseFundList::pda(&program_id);
    let instruction1 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::InitProgram(InsInitProgram::new()),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint_rmb.0, false),
            AccountMeta::new(raise_fund_list.0, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
    );

    let instruction2 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::CreateTokenAccount(InsCreateTokenAccount {
            airdrop: 123,
        }),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(mint_rmb.0, false),
            AccountMeta::new_readonly(payer.pubkey(), false),
            AccountMeta::new(token_rmb.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
    );

    let instruction3 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::AirDrop(InsAirDrop { airdrop: 12345 }),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(mint_rmb.0, false),
            AccountMeta::new_readonly(payer.pubkey(), false),
            AccountMeta::new(token_rmb.0, false),
            // AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            // AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(
        &[instruction1, instruction2, instruction3],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    let transaction_result =
        banks_client.process_transaction(transaction).await;

    assert!(transaction_result.is_ok());

    let token_rmb_account = banks_client
        .get_account(token_rmb.0)
        .await
        .unwrap()
        .unwrap();
    let data =
        spl_token::state::Account::unpack(&token_rmb_account.data).unwrap();
    msg!("{:#?}", data);
    assert!(data.amount == 12468);
}

#[tokio::test]
async fn test_create_user_info() {
    let program_id = Pubkey::new_unique();
    let (banks_client, payer, recent_blockhash) =
        ProgramTest::new("Ins ", program_id, processor!(process_instruction))
            .start()
            .await;
    let mint_rmb = MintRmb::pda(&program_id);
    let token_rmb =
        MintRmb::token_account(&program_id, &payer.pubkey(), &mint_rmb.0);
    let user_info = UserInfo::pda(&program_id, &token_rmb.0);
    let raise_fund_list = RaiseFundList::pda(&program_id);
    let instruction1 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::InitProgram(InsInitProgram::new()),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint_rmb.0, false),
            AccountMeta::new(raise_fund_list.0, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
    );

    let instruction2 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::CreateTokenAccount(InsCreateTokenAccount {
            airdrop: 123,
        }),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(mint_rmb.0, false),
            AccountMeta::new_readonly(payer.pubkey(), false),
            AccountMeta::new(token_rmb.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
    );
    let instruction3 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::CreateUserInfo(InsCreateUserInfo {
            name: "abc".to_string(),
            phone: PhoneNumber::new(1234567890),
            id: "ABCDEFG".to_string(),
        }),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(mint_rmb.0, false),
            AccountMeta::new_readonly(payer.pubkey(), false),
            AccountMeta::new_readonly(token_rmb.0, false),
            AccountMeta::new(user_info.0, false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(
        &[instruction1, instruction2, instruction3],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let transaction_result =
        banks_client.process_transaction(transaction).await;
    assert!(transaction_result.is_ok());

    let user_info_account = banks_client
        .get_account_with_commitment(
            user_info.0,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .unwrap()
        .unwrap();
    msg!("{:?}", user_info_account.data);
    msg!(
        "{:#?}",
        borsh::from_slice::<UserInfo>(user_info_account.data.as_ref()).unwrap()
    )
}
