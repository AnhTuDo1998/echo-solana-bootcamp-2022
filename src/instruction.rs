use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum EchoInstruction {
    // Required
    Echo {
        data: Vec<u8>,
    },
    // Highly Recommended
    InitializeAuthorizedEcho {
        buffer_seed: u64,
        buffer_size: usize,
    },
    // Highly Recommended
    AuthorizedEcho {
        data: Vec<u8>,
    },
    // Optional
    InitializeVendingMachineEcho {
        // Number of tokens required change the buffer
        price: u64,
        buffer_size: usize,
    },
    // Optional
    VendingMachineEcho {
        data: Vec<u8>,
    },
}

#[allow(clippy::too_many_arguments)]
pub fn echo(
    program_id: &Pubkey,
    echo_key: &Pubkey,
    echo_data: Vec<u8>,
) -> Result<Instruction, ProgramError> {
    // Pack an Echo instruction using Borsh
    let init_data = EchoInstruction::Echo { data: echo_data };
    let data = init_data.try_to_vec()?;

    // Pack account for Echo
    let accounts = vec![AccountMeta::new(*echo_key, false)];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
