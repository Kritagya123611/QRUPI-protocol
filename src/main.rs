// src/main.rs
mod kem;
mod sign;
mod benchmark;

fn main() {
    kem::run_kyber_demo();
    sign::run_dilithium_demo();
    benchmark::run_benchmark_demo();
}
