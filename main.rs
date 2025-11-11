use sha2::{Digest, Sha256};
use bs58;
use hex;
use std::io::{self, Write};

fn hex_to_wif(hex_privkey: &str, compressed: bool) -> Result<String, Box<dyn std::error::Error>> {
    let privkey_bytes = hex::decode(hex_privkey)?;
    let mut extended = vec![0x80]; // mainnet prefix
    extended.extend(&privkey_bytes);

    if compressed {
        extended.push(0x01); // compressed pubkey flag
    }

    let checksum = Sha256::digest(&Sha256::digest(&extended));
    let checksum_bytes = &checksum[0..4];

    extended.extend(checksum_bytes);
    let wif = bs58::encode(extended).into_string();

    Ok(wif)
}

fn main() {
    print!("Enter hex private key: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let hex_privkey = input.trim();

    match hex_to_wif(hex_privkey, false) {
        Ok(wif) => println!("Uncompressed WIF: {}", wif),
        Err(e) => eprintln!("Error (uncompressed): {}", e),
    }

    match hex_to_wif(hex_privkey, true) {
        Ok(wif) => println!("Compressed WIF:   {}", wif),
        Err(e) => eprintln!("Error (compressed): {}", e),
    }
}
