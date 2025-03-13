use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{instruction::Instruction, instructions::*};

pub struct Processor {}
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let ins = Instruction::try_from_slice(input)?;

        match ins {
            Instruction::InitProgram(args) =>{
                msg!("Ins: InitProgram");
                ins_init_program(program_id, accounts, args)
            }
            Instruction::CreateTokenAccount(args) => {
                msg!("Ins: CreateTokenAccount");
                ins_create_token_account(program_id, accounts, args)
            }
            Instruction::AirDrop(args) => {
                msg!("Ins: AirDrop");
                ins_air_drop(program_id, accounts, args)
            }
            Instruction::CreateUserInfo(args)=>{
                msg!("Ins: CreateUserInfo");
                ins_create_user_info(program_id, accounts, args)
            }
            Instruction::PublishRaiseFund(args)=>{
                msg!("Ins: PublishRaiseFund");
                ins_publish_raise_fund(program_id, accounts, args)
            }
            Instruction::Donation(args)=>{
                msg!("Ins: Donation");
                ins_donation(program_id, accounts, args)
            }
            Instruction::None => {
                msg!("Ins: none!!!");
                Ok(())
            }
            _ => {
                msg!("ins-todo");
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }
}
