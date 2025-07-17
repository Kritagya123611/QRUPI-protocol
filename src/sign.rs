// src/sign.rs
use pqcrypto_dilithium::dilithium3::{keypair as dilithium_keypair, detached_sign, verify_detached_signature};

pub fn run_dilithium_demo() {
    let (pk, sk) = dilithium_keypair();
    let message = b"hi i want this to be signed";

    let sig = detached_sign(message, &sk);
    let verified = verify_detached_signature(&sig, message, &pk);

    println!("Dilithium Signature Verified: {:?}", verified.is_ok());
}
