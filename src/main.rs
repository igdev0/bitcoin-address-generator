extern crate secp256k1;
use clap::{Arg, Command};

fn main() {
    let private_key = Arg::new("private-key").short('p');
    let version = Arg::new("version").short('v');
    let app = Command::new("app").args([private_key, version]);
    let matches = app.get_matches();
    let mut conf = bitcoin_address_generator::Config {
        private_key: None,
        version: None,
    };
    if let Some(pk) = matches.get_one::<String>("private-key") {
        conf.private_key = Some(pk.to_string())
    }

    if let Some(vs) = matches.get_one::<String>("version") {
        let v: u8 = u8::from_str_radix(vs.trim().trim_start_matches("0x"), 16).unwrap();
        conf.version = Some(v);
    }

    let generator = bitcoin_address_generator::run(conf);

    if let Ok(result) = generator {
        println!("BTC Address: {}", result.address);
        println!("Public key: {}", result.public_key);
        println!("Private key: {}", result.private_key);
    }
}
