# Caesar Cipher CLI

A simple command-line tool for encrypting, decrypting, and brute-forcing Caesar ciphers, written in Rust.

## Features

- **Encrypt** plaintext with a Caesar cipher shift
- **Decrypt** ciphertext with a known shift
- **Brute-force** decrypt ciphertext without knowing the shift
- Preserves punctuation, spaces, and case
- Interactive CLI mode for guided user input

## Usage

### Build

```sh
cargo build
```

### Run (Interactive CLI)

```sh
cargo run
```

You will be guided through prompts:

```
Caesar Cipher CLI
-----------------
Enter mode (encrypt/decrypt/bruteforce): encrypt
Enter text: Hello, world!
Enter shift (0-25): 5
Ciphertext: Mjqqt, btwqi!
```

## Wordlist

A wordlist is used to provide a predefined set of words for the brute-force process. This significantly reduces the time and computational effort compared to trying every possible combination of characters. By using a wordlist like [`wordlist.txt`](https://www.mit.edu/~ecprice/wordlist.10000) or any large collection of common words, the brute-force method focuses on likely candidates, increasing the chances of success.

## Project Structure

```
caesar_cipher/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── test.rs
├── wordlist.txt
└── README.md
```

## Testing

To run the unit tests:

```sh
cargo test
```