use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::user_pda::*;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsPublishRaiseFund {
    /// 真实姓名(需要被募捐的)
    name: RealName,
    /// 手机号码
    phone: PhoneNumber,
    /// 身份证号码,全局唯一
    id: IDNumber,
    /// 简介
    brief: String,
}

pub fn publish_raise_fund_fn() {}
