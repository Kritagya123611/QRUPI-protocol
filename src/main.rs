// src/main.rs
mod kem;
mod sign;
mod benchmark;
mod hybrid;
mod fhe;
mod fhe_hybrid;

fn main() {
    kem::run_kyber_demo();
    sign::run_dilithium_demo();
    benchmark::run_benchmark_demo();
    hybrid::run_hybrid_handshake();
    fhe::fhe_addition_demo();
    fhe_hybrid::fhe_hybrid();
}
