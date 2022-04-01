use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use echo_solana_bootcamp::{instruction::echo, processor::Processor};

#[tokio::test]
async fn test_echo_program() {
    // PK for "on-chain" program
    let program_id = Pubkey::new_unique();

    // For testing program without client
    let mut test_program_env = ProgramTest::new(
        "echo_solana_bootcamp",
        program_id,
        processor!(Processor::process),
    );

    // Create account and add to test program env
    let echo_buffer_account = Keypair::new();
    test_program_env.add_account(
        echo_buffer_account.pubkey(),
        Account {
            lamports: 5,
            data: vec![0u8; 10], // allocated for echo_buffer
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = test_program_env.start().await;

    // Instruction arg/parameter to be copied to buffer
    let echo_data: Vec<u8> = vec![1, 2, 3];

    // Create the echo_buffer instruction then send with signature
    // Note: On client, we will pack account creation together
    // with EchoInstruction as well.
    let mut transaction = Transaction::new_with_payer(
        &[echo(&program_id, &echo_buffer_account.pubkey(), echo_data).unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    let echo_account_data_after = banks_client
        .get_account(echo_buffer_account.pubkey())
        .await
        .expect("Account not found!")
        .expect("Account is empty!");
    let echo_buffer_data_after = echo_account_data_after.data.as_slice();

    let mut expected: Vec<u8> = vec![0u8; 10];
    expected[0] = 1;
    expected[1] = 2;
    expected[2] = 3;

    assert_eq!(echo_buffer_data_after, expected)
}
