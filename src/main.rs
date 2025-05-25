use std::io::{self, Write};
use caesar_cipher::{encrypt, decrypt, brute_force};

fn main() {
    println!("Caesar Cipher CLI");
    println!("-----------------");

    print!("Enter mode (encrypt/decrypt/bruteforce): ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode = mode.trim().to_lowercase();

    match mode.as_str() {
        "encrypt" => {
            print!("Enter text: ");
            io::stdout().flush().unwrap();
            let mut plaintext = String::new();
            io::stdin().read_line(&mut plaintext).unwrap();

            print!("Enter shift (0-25): ");
            io::stdout().flush().unwrap();
            let mut shift = String::new();
            io::stdin().read_line(&mut shift).unwrap();
            let shift: u8 = shift.trim().parse().unwrap_or(0);

            let ciphertext = encrypt(plaintext.trim(), shift);
            println!("Ciphertext: {}", ciphertext);
        }
        "decrypt" => {
            print!("Enter text: ");
            io::stdout().flush().unwrap();
            let mut ciphertext = String::new();
            io::stdin().read_line(&mut ciphertext).unwrap();

            print!("Enter shift (0-25): ");
            io::stdout().flush().unwrap();
            let mut shift = String::new();
            io::stdin().read_line(&mut shift).unwrap();
            let shift: u8 = shift.trim().parse().unwrap_or(0);

            let plaintext = decrypt(ciphertext.trim(), shift);
            println!("Plaintext: {}", plaintext);
        }
        "bruteforce" => {
            print!("Enter text: ");
            io::stdout().flush().unwrap();
            let mut ciphertext = String::new();
            io::stdin().read_line(&mut ciphertext).unwrap();

            let (shift, plaintext) = brute_force(ciphertext.trim());
            println!("Best shift: {}", shift);
            println!("Plaintext: {}", plaintext);
        }
        _ => {
            println!("Unknown mode. Please enter 'encrypt', 'decrypt', or 'bruteforce'.");
        }
    }
}