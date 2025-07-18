mod qrupi;

use std::io;
use qrupi::engine::*;
//use qrupi::fhe_vector_sum::*;
//use qrupi::fhe_cmp_threshold::*;

fn main() {
    println!("=== QRUPI Protocol Demo CLI ===");
    println!("1. Run Hybrid Key Exchange + FHE Encryption");
    println!("2. Encrypted Vote Tally");
    println!("3. Encrypted Threshold Comparison");
    println!("Choose an option: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => run_qrupi_pipeline(),
        //"2" => run_vector_sum_demo(),
        //"3" => run_threshold_demo(),
        _ => println!("Invalid option"),
    }
}
