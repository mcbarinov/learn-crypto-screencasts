use bip39::Mnemonic;

fn main() {
    check("monkey frog radio rubber weekend doctor leave project stay junior dad catch");
    check("frog radio rubber weekend doctor leave project stay junior dad catch monkey");
}

fn check(mnemonic: &str) {
    println!("mnemonic: {}", mnemonic);

    match Mnemonic::parse(mnemonic) {
        Ok(_) => {
            println!("valid")
        }
        Err(err) => {
            println!("invalid: {:?}", err)
        }
    }
}
