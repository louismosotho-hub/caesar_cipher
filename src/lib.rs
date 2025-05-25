use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_wordlist(path: &str) -> HashSet<String> {
    let file = File::open(path).expect("Could not open wordlist file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.to_lowercase())
        .collect()
}

pub fn encrypt(plaintext: &str, shift: u8) -> String {
    let shift = shift % 26;
    let mut ciphertext = String::new();
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((c as u8 - base + shift) % 26) + base;
            ciphertext.push(shifted as char);
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

pub fn decrypt(ciphertext: &str, shift: u8) -> String {
    let shift = shift % 26;
    let mut plaintext = String::new();
    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((c as u8 - base + 26 - shift) % 26) + base;
            plaintext.push(shifted as char);
        } else {
            plaintext.push(c);
        }
    }
    plaintext
}

pub fn brute_force(ciphertext: &str) -> (u8, String) {
    let wordlist = load_wordlist("wordlist.txt");
    let mut best_score = 0;
    let mut best_shift = 0;

    // Try all possible shifts and score by word matches
    for shift in 0..=25 {
        let decrypted = decrypt(ciphertext, shift);
        let lower = decrypted.to_lowercase();
        let words = lower
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter(|w| !w.is_empty());
        let score = words.filter(|word| wordlist.contains(*word)).count();
        if score > best_score {
            best_score = score;
            best_shift = shift;
        }
    }
    // Always return the full decryption using the best shift found
    (best_shift, decrypt(ciphertext, best_shift))
}