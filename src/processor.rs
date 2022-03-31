use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::EchoInstruction;

pub struct Processor {}

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            EchoInstruction::Echo { data } => {
                msg!("Echo Instruction");
                let account_info_iter = &mut accounts.iter();
                let echo_account_info = next_account_info(account_info_iter)?;
                msg!(
                    "Echo Buffer Account: signer: {}, writeable: {}",
                    echo_account_info.is_signer,
                    echo_account_info.is_writable
                );

                let mut echo_buffer = echo_account_info.data.borrow_mut();
                msg!("Initial Echo Buffer from Account: {:?}", echo_buffer);

                // Check if echo_buffer is non-zero
                let has_non_zero: bool = echo_buffer.into_iter().any(|byte| *byte != 0);
                if has_non_zero {
                    // Fail immediately
                    msg!("Echo Buffer has non-zero data. Quitting...");
                    // TODO: Maybe some more meaningful Error...
                    return Err(ProgramError::InvalidInstructionData);
                }

                // Check the echo_buffer is not allocated
                if echo_buffer.len() == 0 {
                    // Fail immediately
                    msg!("Echo Buffer is not allocated. Quitting...");
                    // TODO: Maybe some more meaningful Error...
                    return Err(ProgramError::InvalidInstructionData);
                }

                msg!("Copy from EchoInstruction{data} to on-chain echo_buffer");
                // Copy N bytes to echo_buffer where N is its length
                for it in data.iter().zip(echo_buffer.into_iter()) {
                    let (from, to) = it;
                    *to = *from;
                }
                msg!("Copied Echo Buffer from Account: {:?}", echo_buffer);
                //echo_buffer.serialize(&mut *echo_account_info.data.borrow_mut()).map_err(|_| ProgramError::InvalidInstructionData)?
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}
