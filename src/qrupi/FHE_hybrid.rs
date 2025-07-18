use tfhe::shortint::prelude::*;
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;
use crate::qrupi::hybrid::get_hybrid_secret;
 // or use get_hybrid_secret if you renamed it

pub fn encrypt_and_decrypt(secret: &[u8; 32]){
    // Get the hybrid secret (32-byte array)
    let hybrid_secret = get_hybrid_secret();

    // Set FHE parameters and generate keys
    let params = PARAM_MESSAGE_2_CARRY_2;
    let (client_key, _server_key) = gen_keys(params);

    // Encrypt each byte
    let encrypted_secret: Vec<Ciphertext> = hybrid_secret.iter()
        .map(|&x| client_key.encrypt(x as u64)) 
        .collect();

    // Decrypt and print
    let decrypted: Vec<u8> = encrypted_secret.iter()
    .map(|ct| client_key.decrypt(ct) as u8)
    .collect();

    println!("\n[+] Decrypted FHE Hybrid Secret:");
    for byte in decrypted {
        print!("{:02x} ", byte);
    }
    println!();
}
