use std::iter::Map;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// seed = [RealName]+[IDNumber]+[PubKey]
pub fn user_pda_seed(pubkey: &Pubkey, program_id: &Pubkey) {

    // Pubkey::find_program_address(&[b"user",pubkey.as_ref()], program_id);
}

/// 募捐用户pda的实际存储类型
pub struct UserPda {
    info: UserInfo,
    /// 需要的捐款数量
    require_amount: u32,
    /// 以捐款的数量
    amount: u32,
    /// 收款方
    payee: Pubkey,
    /// 付款方
    payer: Map<Pubkey, u32>,
    /// ture:是运行状态(可被发现,可被捐款)
    is_run: bool,
    /// 计数器,每次交易成功后 加1
    count: u32,
}

/// 约束长度
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RealName(String);

/// 国际区号
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub enum AreaCode {
    /// +86
    #[default]
    China,
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PhoneNumber {
    area_code: AreaCode,
    number: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Brief(String);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct IDNumber(String);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BankCardNumber(String);

/// 用户信息
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserInfo {
    /// 真实姓名(需要被募捐的)
    name: RealName,
    /// 手机号码
    phone: PhoneNumber,
    /// 身份证号码,全局唯一
    id: IDNumber,
    /// 银行卡号
    bank_card: Vec<BankCardNumber>,
    /// 简介
    brief: Brief,
    /// 图片URL来源, 多张
    picture_url: Option<String>,
    /// URL来源的多张图片的 hash
    picture_hash: Option<()>,
}
