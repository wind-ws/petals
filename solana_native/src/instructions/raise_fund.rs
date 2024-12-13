use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{
    error::MyError,
    state::{mint_rmb::MintRmb, raise_fund::*},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsCreateUserInfo {
    /// 真实姓名(需要被募捐的)
    name: RealName,
    /// 手机号码
    phone: PhoneNumber,
    /// 身份证号码,全局唯一
    id: IDNumber,
}

/// 基于token_rmb创建user_info
///
pub fn ins_create_user_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InsCreateUserInfo,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let mint_rmb_account = next_account_info(accounts_iter)?;
    let owner_rmb_account = next_account_info(accounts_iter)?;
    let token_rmb_account = next_account_info(accounts_iter)?;
    let user_info_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (mint_rmb, _) = MintRmb::pda(program_id);
    let (token_rmb, _) =
        MintRmb::token_account(program_id, owner_rmb_account.key, &mint_rmb);
    let (user_info, user_info_bump) = UserInfo::pda(program_id, &token_rmb);
    assert!(mint_rmb.eq(mint_rmb_account.key));
    assert!(token_rmb.eq(token_rmb_account.key));
    assert!(user_info.eq(user_info_account.key));

    if user_info_account.lamports() != 0 {
        return Err(MyError::UserInfoExist.into());
    }
    if mint_rmb_account.lamports() == 0 {
        return Err(MyError::MintRmbNonExist.into());
    }
    if token_rmb_account.lamports() == 0 {
        return Err(MyError::TokenRmbNonExist.into());
    }
    msg!("user-info: {}", user_info_account.key);
    msg!("Creating user-info account ...");
    let rent = (Rent::get()?).minimum_balance(UserInfo::INIT_SPACE as usize);
    invoke_signed(
        &system_instruction::create_account(
            payer_account.key,
            user_info_account.key,
            rent,
            UserInfo::INIT_SPACE,
            system_program.key,
        ),
        &[payer_account.clone(), user_info_account.clone()],
        &[&[
            b"user_info",
            &token_rmb_account.key.to_bytes(),
            &[user_info_bump],
        ]],
    )?;
    msg!("user-info account created successfully");
    let data = UserInfo::new(BaseInfo::new(args.name, args.phone, args.id));
    let serialized_data = borsh::to_vec(&data).unwrap();
    let account_data = &mut user_info_account.try_borrow_mut_data()?;
    account_data[..serialized_data.len()].copy_from_slice(&serialized_data);
    msg!("init user-info account data successfully");

    Ok(())
}
