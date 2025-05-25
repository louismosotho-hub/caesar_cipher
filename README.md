# Caesar Cipher

A simple command-line tool for encrypting, decrypting, and brute-forcing Caesar ciphers, written in Rust.

## Features

- **Encrypt** plaintext with a Caesar cipher shift
- **Decrypt** ciphertext with a known shift
- **Brute-force** decrypt ciphertext without knowing the shift
- Preserves punctuation, spaces, and case

## Usage

### Build

```sh
cargo build
```

### Run

#### Encrypt

```sh
cargo run -- encrypt 'Hello, world!' 5
# Output: Ciphertext: Mjqqt, btwqi!
```

#### Decrypt

```sh
cargo run -- decrypt 'Mjqqt, btwqi!' 5
# Output: Plaintext: Hello, world!
```

#### Brute-force

```sh
cargo run -- brute-force 'Mjqqt, btwqi!'
# Output:
# Best shift: 5
# Plaintext: Hello, world!
```

> **Note:**  
> If your input contains special shell characters (like `!`), use single quotes `'...'` to avoid shell issues.

## Wordlist

A wordlist is used to provide a predefined set of words for the brute-force process. This significantly reduces the time and computational effort compared to trying every possible combination of characters. By using a wordlist like `wordlist.txt` or any large collection of common words, the brute-force method focuses on likely candidates, increasing the chances of success.

## Dependencies

- [clap](https://crates.io/crates/clap) for CLI parsing
