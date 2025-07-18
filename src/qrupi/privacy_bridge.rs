use tfhe::shortint::prelude::*;
use tfhe::shortint::{gen_keys, Ciphertext};
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

/// Represents a private cross-chain token transfer
pub struct EncryptedTransfer {
    pub fhe_amount: Ciphertext,
    pub from_chain: String,
    pub to_chain: String,
}

/// Generates a private token transfer using FHE encryption
pub fn create_encrypted_transfer(
    amount: u64,
    from: &str,
    to: &str,
    client_key: &ClientKey,
) -> EncryptedTransfer {
    let encrypted = client_key.encrypt(amount);
    EncryptedTransfer {
        fhe_amount: encrypted,
        from_chain: from.to_string(),
        to_chain: to.to_string(),
    }
}

/// Decrypt the token transfer on the destination chain
pub fn decrypt_transfer(transfer: &EncryptedTransfer, client_key: &ClientKey) -> u64 {
    client_key.decrypt(&transfer.fhe_amount)
}

/// Demo to simulate a private cross-chain FHE transfer
pub fn demo_cross_chain_transfer() {
    println!("\n Simulating Private Cross-Chain Transfer...");

    // Generate keypair using pre-defined FHE parameters
    let (client_key, _server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);

    // Amount to transfer (in encrypted form)
    let sender_amount = 42;

    // Simulate FHE-encrypted transfer from Ethereum to Solana
    let transfer = create_encrypted_transfer(sender_amount, "Ethereum", "Solana", &client_key);

    println!(" Transfer created: {} -> {}", transfer.from_chain, transfer.to_chain);

    // Decrypt amount on receiver side
    let decrypted = decrypt_transfer(&transfer, &client_key);
    println!(" Decrypted amount on {}: {}", transfer.to_chain, decrypted);
}
