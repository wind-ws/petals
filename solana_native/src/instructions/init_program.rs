use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::{self, Sysvar},
};
use spl_token::state::Mint;

use crate::{
    error::MyError,
    state::{mint_rmb::MintRmb, raise_fund::RaiseFundList},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsInitProgram {
    ins_init_mint_rmb: InitMintRmb,
    ins_init_raise_fund_list: InitRaiseFundList,
}
impl InsInitProgram {
    pub fn new() -> Self {
        Self {
            ins_init_mint_rmb: InitMintRmb {},
            ins_init_raise_fund_list: InitRaiseFundList {},
        }
    }
}

/// 初始化程序
///
/// # init task
/// * mint_rmb mint account init
/// * raise_fund_list pda account init
///
/// # accounts
/// .`[signer,writable]` payer_account \
/// .`[writable]` mint_rmb_account \
/// .`[writable]` raise_fund_list_account \
/// .`[]` rent \
/// .`[]` system_program \
/// .`[]` token_program \
pub fn ins_init_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InsInitProgram,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let _payer_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let raise_fund_list_account = next_account_info(accounts_iter)?;
    let rent_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // 若以存在,则返回错误
    if mint_account.lamports() != 0 {
        return Err(MyError::MintRmbExist.into());
    }
    let (mint_rmb, mint_rmb_bump) = MintRmb::pda(program_id);
    let (raise_fund_list, raise_fund_list_bump) =
        RaiseFundList::pda(program_id);
    assert!(&mint_rmb.eq(mint_account.key));
    assert!(raise_fund_list.eq(raise_fund_list_account.key));
    assert!(system_program::check_id(system_program.key));
    assert!(spl_token::check_id(token_program.key));
    assert!(sysvar::rent::check_id(rent_account.key));

    init_mint_rmb(program_id, accounts, args.ins_init_mint_rmb, mint_rmb_bump)?;
    init_raise_fund_list(
        program_id,
        accounts,
        args.ins_init_raise_fund_list,
        raise_fund_list_bump,
    )?;
    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct InitMintRmb {}
/// 初始化 mint_rmb
fn init_mint_rmb(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _args: InitMintRmb,
    mint_rmb_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let _raise_fund_list_account = next_account_info(accounts_iter)?;
    let rent_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

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
        &[&[MintRmb::SEED, &[mint_rmb_bump]]],
    )?;
    msg!("Initializing mint account");

    invoke_signed(
        &spl_token::instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            mint_account.key,
            Some(mint_account.key),
            2,
        )?,
        &[mint_account.clone(), rent_account.clone()],
        &[&[MintRmb::SEED, &[mint_rmb_bump]]],
    )?;

    msg!("Token mint created successfully");

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct InitRaiseFundList {}

/// 初始化 raise_fund_list
fn init_raise_fund_list(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _args: InitRaiseFundList,
    raise_fund_list_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let _mint_account = next_account_info(accounts_iter)?;
    let raise_fund_list_account = next_account_info(accounts_iter)?;
    let _rent_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let _token_program = next_account_info(accounts_iter)?;

    msg!("RaiseFundList: {}", raise_fund_list_account.key);
    msg!("Creating raise_fund_list account ...");
    let rent = (Rent::get()?).minimum_balance(RaiseFundList::INIT_SPACE);
    invoke_signed(
        &system_instruction::create_account(
            payer_account.key,
            raise_fund_list_account.key,
            rent,
            RaiseFundList::INIT_SPACE as u64,
            system_program.key,
        ),
        &[payer_account.clone(), raise_fund_list_account.clone()],
        &[&[&RaiseFundList::seed(0), &[raise_fund_list_bump]]],
    )?;
    msg!("raise_fund_list created successfully");

    let data = borsh::to_vec(&RaiseFundList::new()).unwrap();
    data.serialize(&mut *raise_fund_list_account.data.borrow_mut()).unwrap();
    

    Ok(())
}
