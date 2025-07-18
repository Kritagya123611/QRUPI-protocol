use dialoguer::{theme::ColorfulTheme, Select};
use qrupi_crypto::qrupi::{
    kem, sign, hybrid, fhe, fhe_hybrid, engine, privacy_bridge,
};

fn main() {
    loop {
        let options = vec![
            "Run Kyber KEM (Key Exchange)",
            "Run Dilithium Signature Demo",
            "Run Hybrid Key Exchange (ECDH + Kyber)",
            "Run FHE Add + Mul Demo",
            "Run FHE Encrypted Hybrid Secret",
            "Run Full QRUPI Engine Demo",
            "Privacy Bridge Cross-Chain Transfer Demo",
            "Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(" QRUPI Protocol CLI Menu")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => kem::run_kyber_demo(),
            1 => sign::run_dilithium_demo(),
            2 => {
                let _ = hybrid::get_hybrid_secret(); // already prints
            }
            3 => fhe::fhe_addition_demo(),
            4 => {
                let secret = hybrid::get_hybrid_secret();
                fhe_hybrid::encrypt_and_decrypt(&secret);
            }
            5 => engine::run_qrupi_pipeline(),
            6 => privacy_bridge::demo_cross_chain_transfer(),
            7 => {
                println!(" Goodbye.");
                break;
            }
            _ => unreachable!(),
        }
    }
}
