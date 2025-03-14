# Encrypter

A secure file encryption tool using AES-256-GCM (Galois/Counter Mode) encryption.

## Features

- Strong AES-256-GCM encryption
- Random key generation
- Secure nonce handling
- Simple command-line interface

## Installation

Ensure you have Rust and Cargo installed on your system. Then clone this repository:

```sh
git clone https://github.com/yourusername/Encrypter.git
cd Encrypter
```

or use the precompiled binary's

## Usage

### Encrypting a file

compiled

```sh
cargo run --bin encrypt <input_file>
```

precompiled

```sh
PATH_TO_BINARY <input_file>
```

This will:

- Create an encrypted version of your file with `.enc` extension
- Generate a key file named `encryption.key`

### Decrypting a file

```sh
cargo run --bin decrypt <encrypted_file>
```

precompiled

```sh
PATH_TO_BINARY <encrypted_file>
```

Make sure the `encryption.key` file is in the same directory as your encrypted file.

## Security Notes

- Keep your `encryption.key` file secure - it's required for decryption
- Each encryption generates a unique key and nonce
- The encryption key is 256 bits (32 bytes)
- The nonce is 96 bits (12 bytes) as recommended for AES-GCM

## Dependencies

- `aes-gcm`: For AES-256-GCM encryption
- `rand`: For secure random number generation

## License

This project is licensed under the MIT License - see the License file for details.
