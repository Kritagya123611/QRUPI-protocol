use crate::qrupi::hybrid::get_hybrid_secret;
use crate::qrupi::fhe_hybrid::encrypt_and_decrypt;

pub fn run_qrupi_pipeline() {
    println!("\n [QRUPI Engine] Starting full hybrid FHE pipeline...");
    let secret = get_hybrid_secret();
    encrypt_and_decrypt(&secret);
}
