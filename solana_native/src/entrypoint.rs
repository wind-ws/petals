use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::{PrintProgramError, ProgramError},
    pubkey::Pubkey,
};

use crate::{error::MyError, processor::Processor};

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) =
        Processor::process(program_id, accounts, instruction_data)
    {
        // 对异常的统一处理
        error.print::<MyError>();
        return Err(error);
    }
    Ok(())
}
