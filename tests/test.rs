use caesar_cipher::{encrypt, decrypt, brute_force};

#[test]
fn test_encrypt_in_range() {
    assert_eq!(encrypt("abc", 3), "def");
    assert_eq!(encrypt("xyz", 2), "zab");
}

#[test]
fn test_encrypt_out_of_range() {
    assert_eq!(encrypt("abc", 29), "def"); // 29 % 26 == 3
    assert_eq!(encrypt("xyz", 28), "zab"); // 28 % 26 == 2
}

#[test]
fn test_decrypt_in_range() {
    assert_eq!(decrypt("def", 3), "abc");
    assert_eq!(decrypt("zab", 2), "xyz");
}

#[test]
fn test_decrypt_out_of_range() {
    assert_eq!(decrypt("def", 29), "abc");
    assert_eq!(decrypt("zab", 28), "xyz");
}

#[test]
fn test_encrypt_sentence() {
    assert_eq!(encrypt("Hello, world!", 5), "Mjqqt, btwqi!");
}

#[test]
fn test_decrypt_sentence() {
    assert_eq!(decrypt("Mjqqt, btwqi!", 5), "Hello, world!");
}

#[test]
fn test_brute_force() {
    let (shift, plaintext) = brute_force("Khoor, zruog!");
    assert_eq!(shift, 3);
    assert_eq!(plaintext, "Hello, world!");
}

#[test]
fn test_brute_force_complex() {
    let ciphertext = "Aopz zapss ullkz ptwyvcltlua. Thfil h tvyl lealuzpcl dvykspza, iba ovd tbjo pz avv tbjo?";
    let (shift, plaintext) = brute_force(ciphertext);
    assert_eq!(shift, 7);
    assert_eq!(plaintext, "This still needs improvement. Maybe a more extensive wordlist, but how much is too much?");
}