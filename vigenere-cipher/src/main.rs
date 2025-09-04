// Vigenere Cipher is a method of encrypting alphabetic text by using a simple form of polyalphabetic substitution.
// Polyalphabetic substitution is a method of encrypting alphabetic text by using multiple substitution alphabets.

fn vigenere_cipher_encrypt(text: &str, key: &str) -> String {
    let key = key.to_uppercase();

    // convert key to numeric shifts (A -> 0, B -> 1, etc)
    let key_bytes: Vec<u8> = key
        // iterate over Unicode characters
        .chars()
        // keep only letters
        .filter(|c| c.is_ascii_alphabetic())
        // convert char to byte (u8) and subtract ASCII 'A'
        .map(|c| c as u8 - b'A')
        .collect();

    if key_bytes.is_empty() {
        return text.to_string();
    }

    let mut key_index = 0;

    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'A' + shift) % 26) + b'A') as char
            } else if c.is_ascii_lowercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'A' + shift) % 26) + b'a') as char
            } else {
                c
            }
        })
        .collect()
}

fn vigenere_cipher_decrypt(text: &str, key: &str) -> String {
    let key = key.to_uppercase();
    let key_bytes: Vec<u8> = key.chars().filter(|c| c.is_ascii_alphabetic())
    .map(|c| c as u8 - b'A')
    .collect();

    if key_bytes.is_empty() {
        return text.to_string();
    }

    let mut key_index = 0;

    text.chars()
    .map(|c| {
        if c.is_ascii_uppercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'A' + 26 - shift) % 26) + b'A') as char
            } else if c.is_ascii_lowercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'a' + 26 - shift) % 26) + b'a') as char
            } else {
                c
            }
        })
        .collect()
}

pub fn main() {
    let plaintext = "Hello, world!";
    let key = "KEY";

    let encrypted = vigenere_cipher_encrypt(plaintext, key);
    let decrypted = vigenere_cipher_decrypt(&encrypted, key);

    println!("Plain Text:  {}", plaintext);
    println!("Key:         {}", key);
    println!("Encrypted:   {}", encrypted);
    println!("Decrypted:   {}", decrypted);
}
