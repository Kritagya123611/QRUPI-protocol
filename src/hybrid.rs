use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use pqcrypto_kyber::kyber1024::{keypair as kyber_keypair, encapsulate, decapsulate};
use pqcrypto_traits::kem::SharedSecret;
use sha3::{Digest, Sha3_256};
use hex;


pub fn run_hybrid_handshake() {

    //ye hai ecdh ka part
    
    let local_secret = EphemeralSecret::new(OsRng);
    let local_public = X25519PublicKey::from(&local_secret);

    let remote_secret = EphemeralSecret::new(OsRng);
    let remote_public = X25519PublicKey::from(&remote_secret);

    let local_ecdh_secret = local_secret.diffie_hellman(&remote_public);
    let remote_ecdh_secret = remote_secret.diffie_hellman(&local_public);

    //yaha se kyber ka part

    let (kyber_pk, kyber_sk) = kyber_keypair();
    let (kyber_sender_secret, ciphertext) = encapsulate(&kyber_pk);
    let kyber_receiver_secret = decapsulate(&ciphertext, &kyber_sk);

    let mut hasher = Sha3_256::new();
    hasher.update(local_ecdh_secret.as_bytes());              
    hasher.update(kyber_sender_secret.as_bytes());            
    let local_hybrid_secret = hasher.finalize();              // combined


    let mut hasher = Sha3_256::new();
    hasher.update(remote_ecdh_secret.as_bytes());
    hasher.update(kyber_receiver_secret.as_bytes());
    let remote_hybrid_secret = hasher.finalize();

    println!("Local  Hybrid Secret: {}", hex::encode(&local_hybrid_secret));
    println!("Remote Hybrid Secret: {}", hex::encode(&remote_hybrid_secret));
    println!("Hybrid secrets match: {}", local_hybrid_secret == remote_hybrid_secret);
}
