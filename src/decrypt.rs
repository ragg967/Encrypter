use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if encrypted file path was provided
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin decrypt <encrypted_file>");
        std::process::exit(1);
    }

    // Get the encrypted file path from arguments
    let encrypted_file = &args[1];

    // Check if the file has the .enc extension
    if !encrypted_file.ends_with(".enc") {
        eprintln!("Error: File must have .enc extension");
        std::process::exit(1);
    }

    // Read the encrypted content
    let encrypted_data = fs::read(encrypted_file)?;

    // Ensure the file is large enough to contain a nonce (12 bytes)
    if encrypted_data.len() < 12 {
        eprintln!("Error: Invalid encrypted file format");
        std::process::exit(1);
    }

    // Extract the nonce (first 12 bytes)
    let nonce_bytes = &encrypted_data[0..12];
    let nonce = Nonce::from_slice(nonce_bytes);

    // Extract the actual ciphertext (everything after the nonce)
    let ciphertext = &encrypted_data[12..];

    // Read the encryption key
    let key_bytes = fs::read("encryption.key")?;
    if key_bytes.len() != 32 {
        eprintln!("Error: Invalid key length");
        std::process::exit(1);
    }

    // Create the cipher instance with the key
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|_| "Invalid key length")?;

    // Decrypt the ciphertext
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed: Invalid key or corrupted data")?;

    // Convert the decrypted bytes to a string
    let decrypted_text = String::from_utf8(plaintext)?;

    // Generate output filename by removing .enc extension
    let output_file = encrypted_file.trim_end_matches(".enc");
    let output_file = format!("{}.decrypted", output_file);

    // Write the decrypted content to a new file
    fs::write(&output_file, decrypted_text)?;

    // Print success message
    println!(
        "Decryption successful. Decrypted content saved as '{}'",
        output_file
    );

    Ok(())
}
