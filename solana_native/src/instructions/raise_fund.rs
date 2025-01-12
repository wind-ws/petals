use std::io::Write;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::state::Account;

use crate::{
    error::MyError,
    state::{mint_rmb::MintRmb, raise_fund::*},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsCreateUserInfo {
    /// 真实姓名(需要被募捐的)
    pub name: String,
    /// 手机号码
    pub phone: PhoneNumber,
    /// 身份证号码,全局唯一
    pub id: String,
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

    let (mint_rmb, _) = MintRmb::pda(program_id);
    let (token_rmb, _) = MintRmb::token_account(
        program_id,
        owner_rmb_account.key,
        &mint_rmb,
    );
    let (user_info, user_info_bump) =
        UserInfo::pda(program_id, &token_rmb);
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
    let data =
        UserInfo::new(BaseInfo::new(args.name, args.phone, args.id));
    let data = borsh::to_vec(&data).unwrap();
    let rent = (Rent::get()?).minimum_balance(data.len());
    // msg!("rent:{}",rent);
    invoke_signed(
        &system_instruction::create_account(
            payer_account.key,
            user_info_account.key,
            rent,
            data.len() as u64,
            program_id,
        ),
        &[payer_account.clone(), user_info_account.clone()],
        &[&[
            b"user_info",
            &token_rmb_account.key.to_bytes(),
            &[user_info_bump],
        ]],
    )?;
    msg!("user-info account created successfully");

    user_info_account.try_borrow_mut_data().unwrap()[..data.len()]
        .copy_from_slice(data.as_ref());
    msg!("init user-info account data successfully");

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsPublishRaiseFund {
    pub info: RaiseFundInfo,
    /// 需要的捐款数量
    pub require_amount: u64,
}

/// 发布募捐
pub fn ins_publish_raise_fund(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InsPublishRaiseFund,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let mint_rmb_account = next_account_info(accounts_iter)?;
    // 收款者 要求签名
    let owner_rmb_account = next_account_info(accounts_iter)?;
    // 收款者 token
    let token_rmb_account = next_account_info(accounts_iter)?;
    let user_info_account = next_account_info(accounts_iter)?;
    let raise_fund_account = next_account_info(accounts_iter)?;
    let raise_fund_list_account = next_account_info(accounts_iter)?;
    // let raise_fund_account = next_account_info(accounts_iter)?;

    let (mint_rmb, _mint_rmb_bump) = MintRmb::pda(program_id);
    let (token_rmb, token_rmb_bump) = MintRmb::token_account(
        program_id,
        owner_rmb_account.key,
        &mint_rmb,
    );
    let (user_info, _user_info_bump) =
        UserInfo::pda(program_id, &token_rmb);
    let (raise_fund, _raise_fund_bump) =
        RaiseFund::pda(program_id, &token_rmb);
    let (raise_fund_list, _raise_fund_list_bump) =
        RaiseFundList::pda(program_id);
    assert!(&mint_rmb.eq(mint_rmb_account.key));
    assert!(&token_rmb.eq(token_rmb_account.key));
    assert!(&user_info.eq(user_info_account.key));
    assert!(&raise_fund.eq(raise_fund_account.key));
    assert!(&raise_fund_list.eq(raise_fund_list_account.key));

    // 确定 user_info 的存在
    if user_info_account.lamports() == 0 {
        // err: user_info 不存在,无法发布募捐
        return Err(MyError::Todo.into());
    }
    // 验证 raise_fund_account 是否已经存在
    if raise_fund_account.lamports() != 0 {
        // raise_fund 存在
        let mut raise_fund_data = borsh::from_slice::<RaiseFund>(
            *raise_fund_account.try_borrow_mut_data()?,
        )?;
        if raise_fund_data.is_run {
            // raise_fund 运行中
            // err: raise_fund 正在运行中,无法创建新的raise_fund
            return Err(MyError::Todo.into());
        } else {
            // raise_fund 被创建初始化,但以结束运行
            raise_fund_data.info = args.info;
            raise_fund_data.require_amount = args.require_amount;
            raise_fund_data.amount = 0;
            raise_fund_data.is_run = true;
            raise_fund_data.serialize(
                &mut *raise_fund_account.data.borrow_mut(),
            )?;
            Ok(())
        }
    } else {
        // raise_fund 不存在
        // 创建and初始化 raise_fund_account pda
        let raise_fund_data =
            RaiseFund::new(args.info, args.require_amount, token_rmb);
        let data = borsh::to_vec(&raise_fund_data)?;
        let rent = (Rent::get()?).minimum_balance(data.len());
        invoke_signed(
            &system_instruction::create_account(
                payer_account.key,
                raise_fund_account.key,
                rent,
                data.len() as u64,
                program_id,
            ),
            &[payer_account.clone(), raise_fund_account.clone()],
            &[&[&RaiseFund::seed(&token_rmb), &[token_rmb_bump]]],
        )?;
        // 将 raise_fund_account.pubkey添加进 raise_fund_list_account 中
        let mut raise_fund_list_data =
            borsh::from_slice::<RaiseFundList>(
                *raise_fund_list_account.try_borrow_mut_data()?,
            )?;
        raise_fund_list_data.add(raise_fund);
        raise_fund_list_data.serialize(
            &mut *raise_fund_list_account.data.borrow_mut(),
        )?;

        Ok(())
    }
}

/// todo : rename the fn
pub fn ins_off_raise_fund() {
    // raise_fund.is_run set false

    // 移除 raise_fund_list 中的 raise_fund.pubkey
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InsDonation {
    /// raise_fund 在 raise_fund_list 中的索引位置
    index: usize,
    /// 捐款数量
    amount: u64,
}

/// 捐款 对目标募捐地址 发起
///
/// 确定性捐款,若没法捐指定数量, 交易被取消
///
/// check:
pub fn ins_donation(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InsDonation,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let mint_rmb_account = next_account_info(accounts_iter)?;
    // 捐款者账户 要求签名
    let owner_rmb_donor_account = next_account_info(accounts_iter)?;
    // 捐款者 token 要求可写
    let token_rmb_donor_account = next_account_info(accounts_iter)?;
    // 收款者账户
    let owner_rmb_payee_account = next_account_info(accounts_iter)?;
    // 收款者 token 要求可写
    let token_rmb_payee_account = next_account_info(accounts_iter)?;
    let user_info_payee_account = next_account_info(accounts_iter)?;
    // 收款者 发布的募捐
    let raise_fund_account = next_account_info(accounts_iter)?;
    let raise_fund_list_account = next_account_info(accounts_iter)?;

    let (mint_rmb, _mint_rmb_bump) = MintRmb::pda(program_id);
    let (token_rmb_donor, token_rmb_donor_bump) =
        MintRmb::token_account(
            program_id,
            owner_rmb_donor_account.key,
            &mint_rmb,
        );
    let (token_rmb_payee, token_rmb_payee_bump) =
        MintRmb::token_account(
            program_id,
            owner_rmb_payee_account.key,
            &mint_rmb,
        );
    let (user_info_payee, _user_info_bump) =
        UserInfo::pda(program_id, &token_rmb_payee);
    let (raise_fund, _raise_fund_bump) =
        RaiseFund::pda(program_id, &token_rmb_payee);
    let (raise_fund_list, _raise_fund_list_bump) =
        RaiseFundList::pda(program_id);
    assert!(&mint_rmb.eq(mint_rmb_account.key));
    assert!(&token_rmb_donor.eq(token_rmb_donor_account.key));
    assert!(&token_rmb_payee.eq(token_rmb_payee_account.key));
    assert!(&user_info_payee.eq(user_info_payee_account.key));
    assert!(&raise_fund.eq(raise_fund_account.key));
    assert!(&raise_fund_list.eq(raise_fund_list_account.key));

    // 验证 是否 is_run
    let mut raise_fund_data = borsh::from_slice::<RaiseFund>(
        *raise_fund_account.try_borrow_mut_data()?,
    )?;
    // 因为 is_run==false 就不应该被发现,所以不用Err返回,而是panic
    assert!(!raise_fund_data.is_run);
    // 检查index是否正确
    let mut raise_fund_list_data = borsh::from_slice::<RaiseFundList>(
        *raise_fund_list_account.try_borrow_mut_data()?,
    )?;
    // 业务逻辑中不应该这种情况,直接panic
    assert!(raise_fund != raise_fund_list_data.get(args.index));

    // 最大可捐款数量
    let max_amount =
        raise_fund_data.require_amount - raise_fund_data.amount;
    // 实际捐款数量
    // if args.amount > max_amount {
    //     // err: 捐款数量超过最大可捐款数量
    //     return Err(MyError::Todo.into());
    // }
    // 业务上 不应该发出 args.amount > max_amount 的交易
    assert!(args.amount > max_amount);

    // 验证 捐款者账户mount是否足够
    let token_rmb_donor_data =
        Account::unpack(*token_rmb_donor_account.try_borrow_data()?)?;
    // if args.amount > token_rmb_donor_data.amount {
    //     // err: 捐款数量 超过 所拥有的数量
    //     return Err(MyError::Todo.into());
    // }
    // 业务上 不应该发出 args.amount > token_rmb_donor_data.amount 的交易
    assert!(args.amount > token_rmb_donor_data.amount);

    // 发生转账
    invoke(
        &spl_token::instruction::transfer(
            &spl_token::ID,
            &token_rmb_donor,
            &token_rmb_payee,
            &token_rmb_donor,
            &[],
            args.amount,
        )?,
        &[],
    )
    .unwrap();

    // 记录 捐款者 的捐款信息
    let mut user_info_payee_data = borsh::from_slice::<UserInfo>(
        *user_info_payee_account.try_borrow_mut_data()?,
    )?;
    user_info_payee_data.add_donor_info(token_rmb_donor, args.amount);
    user_info_payee_data.serialize(
        &mut *user_info_payee_account.try_borrow_mut_data()?,
    )?;

    // 若 刚好捐款完成,则将 raise_fund.is_run 设置为 false, 且从 raise_fund_list中移除
    if args.amount == max_amount {
        raise_fund_data.is_run = false;
        raise_fund_list_data.remove(args.index);
        raise_fund_data.serialize(
            &mut *raise_fund_account.try_borrow_mut_data()?,
        )?;
        raise_fund_list_data.serialize(
            &mut *raise_fund_list_account.try_borrow_mut_data()?,
        )?;
    }

    Ok(())
}
