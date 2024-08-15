use bitcoin_hashes::{hash160, sha256, sha256d, sha512, Hash};
use pbkdf2::pbkdf2_hmac;

fn main() {
    let data1 = "abc";
    let data2 = "a very long message here";
    println!("data1 hex: {}", hex::encode(data1.as_bytes()));
    println!("data2 hex: {}\n", hex::encode(data2.as_bytes()));

    demo_sha256(data1, data2);
    demo_hash256(data1, data2);
    demo_hash160(data1, data2);
    demo_sha512(data1, data2);
    demo_pbkdf2(data1, data2);
}

fn demo_sha256(data1: &str, data2: &str) {
    let hash1 = sha256::Hash::hash(data1.as_bytes());
    let hash2 = sha256::Hash::hash(data2.as_bytes());
    println!("sha256");
    println!("hash1: {}", hash1.to_string());
    println!("hash2: {}\n", hash2.to_string());
}

fn demo_hash256(data1: &str, data2: &str) {
    let hash1 = sha256d::Hash::hash(data1.as_bytes());
    let hash2 = sha256d::Hash::hash(data2.as_bytes());
    println!("hash256");
    println!("hash1: {}", hash1.to_string());
    println!("hash2: {}\n", hash2.to_string());
}

fn demo_hash160(data1: &str, data2: &str) {
    let hash1 = hash160::Hash::hash(data1.as_bytes());
    let hash2 = hash160::Hash::hash(data2.as_bytes());
    println!("hash160");
    println!("hash1: {}", hash1.to_string());
    println!("hash2: {}\n", hash2.to_string());
}

fn demo_sha512(data1: &str, data2: &str) {
    let hash1 = sha512::Hash::hash(data1.as_bytes());
    let hash2 = sha512::Hash::hash(data2.as_bytes());
    println!("sha512");
    println!("hash1: {}", hash1.to_string());
    println!("hash2: {}\n", hash2.to_string());
}

fn demo_pbkdf2(data1: &str, data2: &str) {
    let password = data1.as_bytes();
    let salt = data2.as_bytes();
    let iterations = 2048;

    let mut buf = [0u8; 64];
    pbkdf2_hmac::<sha2::Sha512>(password, salt, iterations, &mut buf);
    println!("pbkdf2");
    print!("pbkdf2: {}", hex::encode(&buf));
}
