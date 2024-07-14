use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, ProgramError};
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn test_voting_app() {
    // Setup testing environment
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "dao-voting",
        program_id,
        processor!(voting::process_instruction),
    )
    .start()
    .await;

    // Initialize test accounts
    let mut context = program_test::ProgramTestContext {
        banks_client: banks_client.clone(),
        payer: payer.clone(),
        recent_blockhash,
    };

    // Initialize voting program
    let mut voting_program = anchor_lang::program!(&mut context, program_id);

    let mut transaction = Transaction::new_with_payer(
        &[voting_program
            .instruction
            .initialize(context.accounts.clone())
            .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    assert!(context.banks_client.process_transaction(transaction).await.is_ok());

    // Test creating a proposal
    let proposal_title = "Test Proposal".to_string();
    let proposal_description = "This is a test proposal.".to_string();

    let mut transaction = Transaction::new_with_payer(
        &[voting_program
            .instruction
            .create_proposal(
                context.accounts.clone(),
                proposal_title.clone(),
                proposal_description.clone(),
            )
            .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    assert!(context.banks_client.process_transaction(transaction).await.is_ok());

    // Test voting for a proposal
    let mut transaction = Transaction::new_with_payer(
        &[voting_program
            .instruction
            .cast_vote(context.accounts.clone(), 0)
            .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    assert!(context.banks_client.process_transaction(transaction).await.is_ok());

    // Test closing voting
    let mut transaction = Transaction::new_with_payer(
        &[voting_program
            .instruction
            .close_voting(context.accounts.clone())
            .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    assert!(context.banks_client.process_transaction(transaction).await.is_ok());

    // Test that voting is closed
    let voting_account = voting_program.voting.fetch(&context.accounts.voting.to_account_info()).unwrap();
    assert_eq!(voting_account.voting_open, false);
}
