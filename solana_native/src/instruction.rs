use borsh::{BorshDeserialize, BorshSerialize};

use crate::instructions::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    /// 空指令(无意义指令)
    None,
    /// 初始化程序,仅执行一次,且在程序部署后立马执行
    InitProgram(),
    /// 发布募捐
    PublishRaiseFund,
    /// 实名捐助
    RealNameDonation,
    /// 匿名捐助
    AnonymousDonation,
    /// 初始化mint_rmb
    ///
    /// note: 这个指令应该在部署程序后立马执行,且只执行一次
    ///
    /// # accounts
    ///
    /// .`[signer,writable]` payer_account \
    /// .`[]` rent \
    /// .`[writable]` mint_rmb_account \
    /// .`[]` system_program \
    /// .`[]` token_program \
    InitMintRmb(InsInitMintRmb),
    /// 创建一个 associated token account pda  with mint_rmb
    ///
    /// # accounts
    /// .`[signer,writable]` payer_account \
    /// .`[]` mint_rmb_account \
    /// .`[]` owner_rmb_account \
    /// .`[writable]` token_rmb_account \
    /// .`[]` system_program \
    /// .`[]` token_program \
    /// .`[]` associated_token_program \
    CreateTokenAccount(InsCreateTokenAccount),
    /// 对 token_account 投放空投
    ///
    /// # accounts
    /// .`[signer,writable]` payer_account \
    /// .`[]` mint_rmb_account \
    /// .`[]` owner_rmb_account \
    /// .`[writable]` token_rmb_account \
    /// .`[]` token_program \
    AirDrop(InsAirDrop),
    CreateUserInfo(InsCreateUserInfo),
}
