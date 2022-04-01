use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use echo_solana_bootcamp::{instruction::echo, processor::Processor};

#[tokio::test]
async fn test_echo_program() {
    // PK for "on-chain" program
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "echo_solana_bootcamp",
        program_id,
        processor!(Processor::process),
    )
    .start()
    .await;

    // echo_buffer account
    let echo_buffer_account = Keypair::new();
    // Instruction arg/parameter
    let echo_data: Vec<u8> = vec![1, 2, 3];

    // Create the echo_buffer account and send instruction in 1 transaction
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &echo_buffer_account.pubkey(),
                0,
                10,
                &program_id,
            ),
            echo(&program_id, &echo_buffer_account.pubkey(), echo_data).unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &echo_buffer_account], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    let echo_account_data_after = banks_client
        .get_account(payer.pubkey())
        .await
        .unwrap()
        .unwrap();
    let echo_buffer_data_after = echo_account_data_after.data.as_slice();

    // TODO: Does this assert correct ?
    assert_eq!(echo_buffer_data_after, vec![1, 2, 3])
}
