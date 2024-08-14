use bip39::Mnemonic;

fn main() {
    let mnemonic = Mnemonic::generate(12).unwrap();
    println!("{}", mnemonic.to_string());
}
