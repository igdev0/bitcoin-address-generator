extern crate secp256k1;
use clap::{Arg, Command};

fn main() {
    let private_key = Arg::new("private-key").short('p');
    let version = Arg::new("version").short('v');
    let use_uncompressed = Arg::new("use-uncompressed")
        .short('u')
        .long("use-uncompressed")
        .num_args(0..=1)
        .default_missing_value("true")
        .default_value("false");

    let app = Command::new("app")
        .args([private_key, version])
        .arg(use_uncompressed);

    let matches = app.get_matches();
    let mut conf = btc_addgen::Config {
        private_key: None,
        version: None,
        use_uncompressed: None,
    };

    if let Some(pk) = matches.get_one::<String>("private-key") {
        conf.private_key = Some(pk.to_string())
    }

    if let Some(vs) = matches.get_one::<String>("version") {
        let v: u8 = u8::from_str_radix(vs.trim().trim_start_matches("0x"), 16).unwrap();
        conf.version = Some(v);
    }

    if let Some(vs) = matches.get_one::<String>("use-uncompressed") {
        conf.use_uncompressed = Some(vs.parse().unwrap());
    }

    let generator = btc_addgen::run(conf);

    if let Ok(result) = generator {
        println!("BTC Address: {}", result.address);
        println!("Public key: {}", result.public_key);
        println!("Public key compressed: {}", result.public_key_compressed);
        println!("Private key: {}", result.private_key);
    }
}
