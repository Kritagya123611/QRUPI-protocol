// src/kem.rs
use pqcrypto_kyber::kyber1024::{decapsulate, encapsulate, keypair as kyber_keypair};
use pqcrypto_traits::kem::SharedSecret;
use hex;

pub fn run_kyber_demo() {
    // Kyber KEM: key generation, encapsulation, decapsulation
    let (pk, sk) = kyber_keypair();
    let (ss_sender, ciphertext) = encapsulate(&pk);
    let ss_receiver = decapsulate(&ciphertext, &sk);

    println!("Kyber Shared Secret (Sender):   {}", hex::encode(ss_sender.as_bytes()));
    println!("Kyber Shared Secret (Receiver): {}", hex::encode(ss_receiver.as_bytes()));
    assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());
}
