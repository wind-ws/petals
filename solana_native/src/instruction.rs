use borsh::{BorshDeserialize, BorshSerialize};

use crate::instructions::{InsInitMintRmb, InsPublishRaiseFund};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    /// 空指令(无意义指令)
    None,
    /// 发布募捐
    PublishRaiseFund(InsPublishRaiseFund),
    /// 实名捐助
    RealNameDonation,
    /// 匿名捐助
    AnonymousDonation,
    /// 初始化mint_rmb
    ///
    /// # accounts
    ///
    /// .`[signer,writable]` payer \
    /// .`[]` rent \
    /// .`[writable]` mint_account \
    /// .`[]` system_program \
    /// .`[]` token_program \
    InitMintRmb(InsInitMintRmb),
}
