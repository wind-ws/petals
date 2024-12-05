use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::{self, ProgramResult},
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
            Instruction::InitMintRmb(args) => {
                msg!("Ins: InitMintRmb");
                ins_init_mint_rmb(program_id, accounts, args)
            }
            Instruction::None => {
                msg!("none!!!");
                Ok(())
            }
            _ => {
                msg!("ins-todo");
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }
}
