use solana_program_test::*;
use solana_sdk::{
    address_lookup_table::instruction,
    feature_set::rent_for_sysvars,
    instruction::AccountMeta,
    pubkey::Pubkey,
    signature::Signer,
    system_program, sysvar,
    transaction::{self, Transaction},
};

use crate::{
    entrypoint::process_instruction,
    instruction::Instruction,
    instructions::{InsCreateTokenAccount, InsInitMintRmb},
    state::mint_rmb::{self, MintRmb},
};

#[tokio::test]
async fn test_none() {
    let program_id = Pubkey::new_unique();
    let (banks_client, payer, recent_blockhash) = ProgramTest::new(
        "Ins InitMintRmb",
        program_id,
        processor!(process_instruction),
    )
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
    let instruction = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::InitMintRmb(InsInitMintRmb {}),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new(mint_rmb_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
    );

    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
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

    let instruction1 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::InitMintRmb(InsInitMintRmb {}),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new(mint_rmb.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
    );

    let instruction2 = solana_program::instruction::Instruction::new_with_borsh(
        program_id,
        &Instruction::CreateTokenAccount(InsCreateTokenAccount {
            airdrop: 123
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
