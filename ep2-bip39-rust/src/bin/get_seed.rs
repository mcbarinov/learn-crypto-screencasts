use bip39::Mnemonic;

fn main() {
    let mnemonic = Mnemonic::parse("monkey frog radio rubber weekend doctor leave project stay junior dad catch").unwrap();
    let passphrase = "my-secret";
    let seed = mnemonic.to_seed(passphrase);
    println!("mnemonic: {}", mnemonic.to_string());
    println!("passphrase: {}", passphrase);
    println!("seed: {}", hex::encode(seed));
}
