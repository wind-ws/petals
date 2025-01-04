use borsh::{BorshDeserialize, BorshSerialize};

use crate::instructions::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    /// 空指令(无意义指令)
    None,
    /// 初始化程序,仅执行一次,且在程序部署后立马执行
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
    InitProgram(InsInitProgram),
    /// 发布募捐
    /// 
    /// # accounts
    /// .`[signer,writable]` payer_account \
    /// 
    PublishRaiseFund(InsPublishRaiseFund),
    /// 捐助
    /// 
    /// # accounts
    /// 
    /// 
    Donation(InsDonation),

    // ViewInfo(),
    // #[deprecated(note = "Use `InitProgram` instead")]
    // InitMintRmb(),
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
    /// 为指定token_rmb_account 创建 一个user_info,用于存储用户信息数据
    ///
    /// # accounts
    /// .`[signer,writable]` payer_account \
    /// .`[]` mint_rmb_account \
    /// .`[]` owner_rmb_account \
    /// .`[]` token_rmb_account \
    /// .`[writable]` user_info_account \
    ///
    CreateUserInfo(InsCreateUserInfo),
    /// 注销所有数据和账号,且回收sol
    /// 
    /// # accounts
    /// 
    /// # note
    /// - 当 token_rmb amount 不为零时,指令执行失败
    /// - 当 还存在募捐在运行状态时, 指令执行失败
    /// 
    LogoutRecycling()
}
