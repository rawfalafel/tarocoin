use std::fs;
use std::io::Write;

extern crate rand;
extern crate secp256k1;

use error;

const KEYS_DIR_PATH: &'static str = "keys";

pub fn generate(id: String) -> Result<(), error::Error> {
    let mut file = fs::File::create(format!("{}/{}", KEYS_DIR_PATH, id))?;

    let mut rng = rand::StdRng::new()?;

    let secp256k1 = secp256k1::Secp256k1::new();
    let (secret_key, _) = secp256k1.generate_keypair(&mut rng)?;

    file.write_all(&secret_key[0..32])?;

    Ok(())
}