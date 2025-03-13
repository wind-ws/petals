use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
use spl_token::{instruction, state::Mint};

use crate::{error::MyError, state::mint_rmb::MintRmb};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsCreateTokenAccount {
    /// 给予一定的初始coin
    pub airdrop: u32,
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

    if mint_rmb_account.lamports() == 0 {
        return Err(MyError::MintRmbNonExist.into());
    }
    if token_rmb_account.lamports() != 0 {
        return Err(MyError::TokenRmbExist.into());
    }

    let (mint_rmb, bump) = MintRmb::pda(program_id);
    let (token_rmb, _token_bump) =
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
            // bug: 按道理来说, funding和wallet 应该要存在一个owner才对
            payer_account.key,
            owner_rmb_account.key,
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
        &[&[MintRmb::SEED, &[bump]]],
    )?;
    msg!("Tokens minted to wallet successfully.");

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsAirDrop {
    pub airdrop: u16,
}

/// plan : 也许应该设置一个 一天只能空投一次 的限制
pub fn ins_air_drop(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InsAirDrop,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let mint_rmb_account = next_account_info(accounts_iter)?;
    let owner_rmb_account = next_account_info(accounts_iter)?;
    let token_rmb_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    let (mint_rmb, bump) = MintRmb::pda(program_id);
    let (token_rmb, _token_bump) =
        MintRmb::token_account(program_id, owner_rmb_account.key, &mint_rmb);
    assert!(&mint_rmb.eq(mint_rmb_account.key));
    assert!(&token_rmb.eq(token_rmb_account.key));
    assert!(spl_token::check_id(token_program.key));

    if mint_rmb_account.lamports() == 0 {
        return Err(MyError::MintRmbNonExist.into());
    }
    if token_rmb_account.lamports() == 0 {
        return Err(MyError::TokenRmbNonExist.into());
    }
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
        &[&[MintRmb::SEED, &[bump]]],
    )?;
    msg!("Tokens minted to wallet successfully.");

    Ok(())
}

pub fn tranfer(from:&Pubkey,to:Pubkey,amount:u64){

}
