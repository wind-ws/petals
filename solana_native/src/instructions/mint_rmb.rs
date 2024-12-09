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

use crate::{error::MyError, state::mint_rmb::MintRmb};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsInitMintRmb {}

/// for [`Instruction::InitMintRmb`]
///
/// # accounts(looking at the code)
/// 0.
pub fn ins_init_mint_rmb(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _args: InsInitMintRmb,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let rent_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // 若以存在,则返回错误
    if mint_account.lamports() != 0 {
        return Err(MyError::MintRmbExist.into());
    }

    let (mint_pda, bump) = MintRmb::pda(program_id);
    assert!(&mint_pda.eq(mint_account.key));
    assert!(system_program::check_id(system_program.key));
    assert!(spl_token::check_id(token_program.key));

    msg!("Mint: {}", mint_account.key);

    msg!("Creating mint account");
    invoke_signed(
        &system_instruction::create_account(
            payer_account.key,
            mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_account.clone(),
            payer_account.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
        &[&[MintRmb::SEED, &[bump]]],
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
        &[mint_account.clone(), rent_account.clone()],
        &[&[MintRmb::SEED, &[bump]]],
    )?;

    msg!("Token mint created successfully");

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsCreateTokenAccount {
    /// 给予一定的初始coin
    pub airdrop: u8,
}

/// for [`Instruction::CreateTokenAccount`]
pub fn ins_create_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InsCreateTokenAccount,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let mint_rmb_account = next_account_info(accounts_iter)?;
    let owner_rmb_account = next_account_info(accounts_iter)?;
    let token_rmb_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    if token_rmb_account.lamports() != 0 {
        return Err(MyError::TokenRmbExist.into());
    }

    let (mint_rmb, bump) = MintRmb::pda(program_id);
    let (token_rmb, token_bump) =
        MintRmb::token_account(program_id, owner_rmb_account.key, &mint_rmb);
    assert!(&mint_rmb.eq(mint_rmb_account.key));
    assert!(&token_rmb.eq(token_rmb_account.key));
    assert!(system_program::check_id(system_program.key));
    assert!(spl_token::check_id(token_program.key));
    assert!(spl_associated_token_account::check_id(
        associated_token_program.key
    ));

    msg!("Creating associated token account... :{}", &token_rmb);
    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            payer_account.key,
            payer_account.key,
            mint_rmb_account.key,
            token_program.key
        ),
        &[
                mint_rmb_account.clone(),
                token_rmb_account.clone(),
                payer_account.clone(),
                system_program.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
    )?;
    msg!("Creating associated token account successfully");

    if args.airdrop == 0 {
        return Ok(());
    }

    msg!(
        "Minting {} tokens to associated token account...",
        args.airdrop
    );
    invoke_signed(
        &spl_token::instruction::mint_to(
            token_program.key,
            mint_rmb_account.key,
            &token_rmb_account.key,
            mint_rmb_account.key,
            &[],
            args.airdrop as u64,
        )?,
        &[
            mint_rmb_account.clone(),
            token_rmb_account.clone(),
            payer_account.clone(),
            token_program.clone(),
        ],
        &[&[MintRmb::SEED, &[bump]]]
    )?;
    msg!("Tokens minted to wallet successfully.");

    Ok(())
}