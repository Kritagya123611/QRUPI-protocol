use std::time::Instant;

use pqcrypto_dilithium::dilithium3::{keypair as dilithium_keypair, detached_sign, verify_detached_signature};
use pqcrypto_kyber::kyber1024::{decapsulate, encapsulate, keypair as kyber_keypair};
use pqcrypto_traits::kem::SharedSecret;
use hex;

pub fn run_benchmark_demo(){
    //first hum kyber ke benchmarks ko test karenge uske baaad delithium ko
    //this is the kyber part
    let now=Instant::now();
    let (pk, sk) = kyber_keypair();
    println!("Kyber keypair gen took: {:?}", now.elapsed());

    let now=Instant::now();
    let (ss_sender, ciphertext) = encapsulate(&pk);
    println!("Kyber encapsulation took: {:?}", now.elapsed());

    let now=Instant::now();
    let ss_receiver = decapsulate(&ciphertext, &sk);
    println!("Kyber decapsulation took: {:?}", now.elapsed());

    assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());

    //now the delithium part
     let now = Instant::now();
    let (pk, sk) = dilithium_keypair();
    println!("Dilithium keypair gen took: {:?}", now.elapsed());

    let message = b"benchmark test message";
    let now = Instant::now();
    let sig = detached_sign(message, &sk);
    println!("Dilithium signing took: {:?}", now.elapsed());

    let now = Instant::now();
    let verified = verify_detached_signature(&sig, message, &pk);
    println!("Dilithium verification took: {:?}", now.elapsed());

    println!("Signature valid: {:?}", verified.is_ok());
}