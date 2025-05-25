use clap::{Parser, Subcommand};
use caesar_cipher::{encrypt, decrypt, brute_force};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Encrypt {
        plaintext: String,
        shift: u8,
    },

    Decrypt {
        ciphertext: String,
        shift: u8,
    },

    BruteForce {
        ciphertext: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { plaintext, shift } => {
            let ciphertext = encrypt(&plaintext, shift);
            println!("Ciphertext: {}", ciphertext);
        }
        Commands::Decrypt { ciphertext, shift } => {
            let plaintext = decrypt(&ciphertext, shift);
            println!("Plaintext: {}", plaintext);
        }
        Commands::BruteForce { ciphertext } => {
            let (shift, plaintext) = brute_force(&ciphertext);
            println!("Best shift: {}", shift);
            println!("Plaintext: {}", plaintext);
        }
    }
}