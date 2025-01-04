//! 每次都要验证 account 是不是 要求的account,干脆直接这样 获取时验证
//! 并且意义化account
//! 
/// todo: remove the mod

use solana_sdk::{account_info::{next_account_info, AccountInfo}, program_error::ProgramError};

pub fn system_account() {}

pub fn mint_rmb_account<'a, 'b, I: Iterator<Item = &'a AccountInfo<'b>>>(
    iter: &mut I,
) -> Result<I::Item, ProgramError> {
    let account = next_account_info(iter)?;
    // assert_eq!(account);
    Ok(account)
}
