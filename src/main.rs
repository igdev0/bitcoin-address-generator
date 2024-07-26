extern crate secp256k1;

use bitcoin_address_generator::Config;

fn main() {
    let generator = bitcoin_address_generator::run(Config {
        private_key: None,
        version: Some(0x00),
    });

    if let Ok(result) = generator {
        println!("Address: {}, = {}len", result.address, result.address.len());
        println!(
            "Public key: {}, = {} len",
            result.public_key,
            result.public_key.len()
        );
        println!(
            "Private key: {}, = {}len",
            result.private_key,
            result.private_key.len()
        );
    }
}
