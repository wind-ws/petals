use solana_program_test::*;
use solana_sdk::{
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
    instructions::InsInitMintRmb,
    state::mint_rmb::{self, MintRmbPda},
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
    let mint_rmb_pda = MintRmbPda::pda(&program_id);
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
