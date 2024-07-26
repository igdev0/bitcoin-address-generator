
# Bitcoin Address Generator

This project is a Bitcoin address generator written in Rust. It leverages various cryptographic libraries to generate a Bitcoin address, along with the corresponding public and private keys. The project is designed to be installed and run via Cargo, the Rust package manager.

## Features

- Generate a random private key.
- Derive the corresponding public key from the private key.
- Compute the Bitcoin address from the public key.
- Optionally accept a user-provided private key and version.

## Installation

To install this project, ensure you have Rust and Cargo installed on your system. You can install it directly from crates.io or from the source.

### Install from crates.io

To install the Bitcoin address generator directly from crates.io, use the following command:

```sh
cargo install btc-addgen
```

### Install from source
To install from the source, clone this repository and navigate to its directory, then run the following command:
```sh
cargo build --release
```

## Usage
### Using the compiled binary
```sh
./target/release/bitcoin_address_generator [OPTIONS]
```
### Using the installed cargo binary
```sh
btc-addgen [OPTIONS]
```

### Command-line Options

- `-p, --private-key <PRIVATE_KEY>`: Specify a private key in hexadecimal format.
- `-v, --version <VERSION>`: Specify the version byte in hexadecimal format (e.g., `0x00` for mainnet).

### Example

Generate a Bitcoin address with a random private key:

```sh
./target/release/bitcoin_address_generator
```

Generate a Bitcoin address using a specific private key and version:

```sh
./target/release/bitcoin_address_generator -p <PRIVATE_KEY> -v <VERSION>
```

#### Key Functions and Structures

- `Config`: Holds the configuration for the address generation, including optional private key and version.
- `RunResult`: Represents the result of the address generation, including the private key, public key, and address.
- `RunError`: Enum for handling errors during the address generation process.
- `get_key_pair`: Generates a key pair (private and public key) based on the provided configuration.
- `run`: The main function that performs the address generation process.

## Dependencies

The project relies on several external crates for cryptographic operations:

- `base58check`: For Base58Check encoding.
- `rand`: For random number generation.
- `ripemd`: For RIPEMD-160 hashing.
- `secp256k1`: For elliptic curve operations.
- `sha2`: For SHA-256 hashing.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgements

This project uses various cryptographic libraries and builds upon standard Bitcoin address generation techniques. Special thanks to the authors of the `base58check`, `rand`, `ripemd`, `secp256k1`, and `sha2` crates for their invaluable work.