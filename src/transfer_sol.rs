use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{message::Message, pubkey::Pubkey, system_instruction, transaction::Transaction};

pub fn create_instruction(from: String, to: String, amount: u64) -> String {
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(&rpc_url);

    let from_pubkey = Pubkey::from_str(&from).expect("Failed to parse from pubkey");
    let to_pubkey = Pubkey::from_str(&to).expect("Failed to parse to pubkey");
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, amount);

    let message =
        Message::new_with_blockhash(&[instruction], Some(&from_pubkey), &recent_blockhash);

    let unsigned_tx = Transaction::new_unsigned(message);

    let serialized_tx = bincode::serialize(&unsigned_tx).expect("Failed to serialize transaction");
    let base64_tx =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &serialized_tx);

    base64_tx
}

pub fn send_signed_transaction(
    signed_tx_base64: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Connect to Solana network
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(&rpc_url);

    // Decode the base64 signed transaction
    let signed_tx_data = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &signed_tx_base64,
    )?;

    // Deserialize the transaction
    let signed_tx: Transaction = bincode::deserialize(&signed_tx_data)?;

    // Send and confirm transaction
    let signature = client.send_and_confirm_transaction(&signed_tx)?;

    Ok(signature.to_string())
}
