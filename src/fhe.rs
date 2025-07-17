use tfhe::shortint::prelude::*;

pub fn fhe_addition_demo() {
    let params = PARAM_MESSAGE_4_CARRY_4;

    let (client_key, server_key) = gen_keys(params);
    let a = 2;
    let b = 3;

    let ct_a = client_key.encrypt(a);
    let ct_b = client_key.encrypt(b);

    let ct_add = server_key.unchecked_add(&ct_a, &ct_b);
    let ct_mul = server_key.unchecked_mul_lsb(&ct_a, &ct_b);

    let add_result = client_key.decrypt(&ct_add);
    let mul_result = client_key.decrypt(&ct_mul);

    println!("FHE Encrypted Addition: {} + {} = {}", a, b, add_result);
    println!("FHE Encrypted Multiplication: {} * {} = {}", a, b, mul_result);
}
