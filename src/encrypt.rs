use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use rand::RngCore;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided
    if args.len() != 2 {
        eprintln!("Usage: cargo run <input_file>");
        std::process::exit(1);
    }

    // Get the input file path from arguments
    let input_file = &args[1];

    // Read the contents of the text file
    let plaintext = fs::read_to_string(input_file)?;

    // Generate a random 256-bit key (32 bytes)
    let mut key_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut key_bytes);

    // Create the cipher instance with the key
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).expect("Invalid key length");
    // Generate a random nonce (12 bytes is recommended for AES-GCM)
    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the plaintext
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("Encryption failure!");

    // Save the nonce and ciphertext (nonce prepended for decryption)
    let mut output = nonce_bytes.to_vec();
    output.extend(ciphertext);

    // Generate output filename by appending .enc to the input filename
    let output_file = format!("{}.enc", input_file);
    fs::write(&output_file, &output)?;

    // Save the key for future decryption
    fs::write("encryption.key", &key_bytes)?;

    // Print a success message
    println!("Encryption successful. File saved as '{}'", output_file);
    println!("The key has been saved to 'encryption.key'.");
    println!("Keep this file secure - you'll need it to decrypt the data.");

    Ok(())
}
