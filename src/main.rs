// src/main.rs
mod kem;
mod sign;

fn main() {
    kem::run_kyber_demo();
    sign::run_dilithium_demo();
}
