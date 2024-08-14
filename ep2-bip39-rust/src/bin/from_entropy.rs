use bip39::rand::RngCore;
use bip39::{rand, Mnemonic};

fn main() {
    let mut rng = rand::thread_rng();
    let mut entropy = [0u8; 16];
    RngCore::fill_bytes(&mut rng, &mut entropy);
    println!("entropy: {}", hex::encode(&entropy));

    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
    println!("mnemonic: {}", mnemonic.to_string());
}
