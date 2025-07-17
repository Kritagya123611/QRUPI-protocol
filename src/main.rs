use pqcrypto_kyber::kyber1024::{decapsulate, encapsulate, keypair as kyber_keypair, Ciphertext};
use pqcrypto_dilithium::dilithium3::{keypair as dilithium_keypair, detached_sign, verify_detached_signature};
use pqcrypto_traits::kem::{SharedSecret, Ciphertext as CiphertextTrait}; // for .as_bytes()
use hex;

fn main() {
    //this is the kyber part
    let (pk, sk) = kyber_keypair();
    let (ss_sender, ciphertext) = encapsulate(&pk); 
    let ss_receiver = decapsulate(&ciphertext, &sk);

    println!("Kyber Shared Secret (Sender):   {}", hex::encode(ss_sender.as_bytes()));
    println!("Kyber Shared Secret (Receiver): {}", hex::encode(ss_receiver.as_bytes()));
    assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());

    //this is the dilithium part
    let (pk_dilithium, sk_dilithium) = dilithium_keypair();
    let message = b"hi i want this to be signed";

    let sig = detached_sign(message, &sk_dilithium);
    let verified = verify_detached_signature(&sig, message, &pk_dilithium);

    println!("Dilithium Signature Verified: {:?}", verified.is_ok());
}
