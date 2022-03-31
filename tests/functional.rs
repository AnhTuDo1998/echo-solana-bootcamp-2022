use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::Transaction};

use echo_solana_bootcamp::{instruction::echo, processor::Processor};

#[tokio::test]
async fn test_echo_program() {
    // TODO: Create user keypair & allocate on-chain data buffer...

    // PK for "on-chain" program
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "echo_solana_bootcamp",
        program_id,
        processor!(Processor::process),
    )
    .start()
    .await;

    // Instruction arg/parameter
    let echo_data: Vec<u8> = vec![1, 2, 3];

    // Transaction (signed) that contains the Ix Echo payload
    let mut transaction = Transaction::new_with_payer(
        &[echo(&program_id, &payer.pubkey(), echo_data).unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    let echo_account_data_after = banks_client
        .get_account(payer.pubkey())
        .await
        .unwrap()
        .unwrap();
    let echo_buffer_data_after = echo_account_data_after.data.as_slice();
    assert_eq!(echo_buffer_data_after, vec![1, 2, 3])
}
