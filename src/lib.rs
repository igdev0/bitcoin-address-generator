extern crate base58check;
extern crate rand;
extern crate ripemd;
extern crate secp256k1;
extern crate sha2;

use base58check::ToBase58Check;
use core::fmt::Display;
use hex;
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
pub struct RunResult {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
}

pub enum RunError {
    ParseError(String),
}
impl Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::ParseError(err) => write!(f, "Parse Error: {}", err),
        }
    }
}

pub struct Config {
    pub private_key: Option<String>,
    pub version: Option<u8>,
}
fn get_key_pair(config: &Config) -> Result<(SecretKey, PublicKey), String> {
    let secp = Secp256k1::new();
    if let Some(private_key) = &config.private_key {
        let data =
            hex::decode(&private_key).expect("Unable to decode secret key from the data given.");
        let secret =
            SecretKey::from_slice(&data).expect("Unable to create secret key from the data given.");
        let public = PublicKey::from_secret_key(&secp, &secret);
        return Ok((secret, public));
    }

    let mut rng = rand::thread_rng();
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    Ok((secret_key, public_key))
}

pub fn run(config: Config) -> Result<RunResult, RunError> {
    // Step 1: Generate a random private key
    let key_pair = get_key_pair(&config);

    match key_pair {
        Ok((secret_key, public_key)) => {
            // Step 2: Generate the public key
            let serialized_public_key = public_key.serialize_uncompressed();

            // Step 3: Perform SHA-256 hashing on the public key
            let sha256_hash = Sha256::digest(&serialized_public_key[1..]);

            // Step 4: Perform RIPEMD-160 hashing on the result of SHA-256
            let mut ripemd160_hash = Ripemd160::digest(&sha256_hash);
            // println!("RIPEMD-160 Hash: {:?}", ripemd160_hash);

            // Step 5: Read the version code
            let version: u8 = config.version.unwrap_or(0x00);

            // Step 6:
            //      6.1 Prepend the version
            //      6.2 Double the SHA256 & append the checksum (first 4 digits) to the payload (version + ripemd160 hash)
            let bitcoin_address = ripemd160_hash.to_base58check(version);

            Ok(RunResult {
                address: bitcoin_address,
                private_key: secret_key.secret_bytes().to_base58check(0x80), //0x80 is the version byte for WIF
                public_key: hex::encode(public_key.serialize_uncompressed()),
            })
        }
        Err(error) => Err(RunError::ParseError(error)),
    }
}
