use std::{cell::RefCell, rc::Rc};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
use solana_sdk::program_error::PrintProgramError;
use spl_token::{instruction, state::Mint};

use crate::{error::MyError, state::mint_rmb::MintRmbPda};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsInitMintRmb {}

/// for [`Instruction::InitMintRmb`]
///
/// # accounts(looking at the code)
/// 0.
pub fn ins_init_mint_rmb(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InsInitMintRmb,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // 若以存在,则返回错误
    if mint_account.lamports() != 0 {
        return Err(MyError::MintRmbExist.into());
    }

    let (mint_pda, bump) = MintRmbPda::pda(program_id);
    assert!(&mint_pda.eq(mint_account.key));
    assert!(system_program::check_id(system_program.key));
    assert!(spl_token::check_id(token_program.key));

    msg!("Mint: {}", mint_account.key);

    msg!("Creating mint account");
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
        &[&[MintRmbPda::SEED, &[bump]]],
    )?;
    msg!("Initializing mint account");

    invoke_signed(
        &instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            mint_account.key,
            Some(mint_account.key),
            2,
        )?,
        &[mint_account.clone(), rent.clone()],
        &[&[MintRmbPda::SEED, &[bump]]],
    )?;


    msg!("Token mint created successfully");

    Ok(())
}
