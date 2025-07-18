use std::io::{self, Write};

use qrupi_crypto::qrupi::{
    kem,
    sign,
    hybrid,
    fhe,
    fhe_hybrid,
    engine,
};


fn main() {
    loop {
        println!("\n==== 🔐 QRUPI Protocol CLI ====");
        println!("1. Run Kyber KEM (Key Exchange)");
        println!("2. Run Dilithium Signature Demo");
        println!("3. Run Hybrid Key Exchange (ECDH + Kyber)");
        println!("4. Run FHE Add + Mul Demo");
        println!("5. Run FHE Encrypted Hybrid Secret");
        println!("6. Run Full QRUPI Engine Demo");
        println!("0. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => kem::run_kyber_demo(),
            "2" => sign::run_dilithium_demo(),
            "3" => {
                let _ = hybrid::get_hybrid_secret(); // already prints
            }
            "4" => fhe::fhe_addition_demo(),
            "5" => {
                let secret = hybrid::get_hybrid_secret();
                fhe_hybrid::encrypt_and_decrypt(&secret);
            }
            "6" => engine::run_qrupi_pipeline(),
            "0" => {
                println!("Goodbye.");
                break;
            }
            _ => println!("❌ Invalid option, try again."),
        }
    }
}

// This is the main entry point for the QRUPI CLI application.