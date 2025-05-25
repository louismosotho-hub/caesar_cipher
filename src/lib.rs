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

// Helper to check shift is in range
fn validate_shift(shift: u8) {
    if shift > 25 {
        panic!("Shift must be between 0 and 25");
    }
}

// Helper to shift a single character
fn shift_char(c: char, shift: i8) -> char {
    if c.is_ascii_alphabetic() {
        let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        let shifted = ((c as u8 - base) as i8 + shift).rem_euclid(26) as u8 + base;
        shifted as char
    } else {
        c
    }
}

pub fn encrypt(plaintext: &str, shift: u8) -> String {
    validate_shift(shift);
    let mut ciphertext = String::new();
    for c in plaintext.chars() {
        ciphertext.push(shift_char(c, shift as i8));
    }
    ciphertext
}

pub fn decrypt(ciphertext: &str, shift: u8) -> String {
    validate_shift(shift);
    let mut plaintext = String::new();
    for c in ciphertext.chars() {
        plaintext.push(shift_char(c, -(shift as i8)));
    }
    plaintext
}

pub fn brute_force(ciphertext: &str) -> (u8, String) {
    let wordlist = load_wordlist("wordlist.txt");
    let mut best_score = 0;
    let mut best_shift = 0;

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
    (best_shift, decrypt(ciphertext, best_shift))
}