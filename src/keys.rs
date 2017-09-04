use std::fs::File;
use std::io::Write;

extern crate rand;
extern crate secp256k1;

const KEYS_DIR_PATH: &'static str = "keys";

pub fn generate(id: String) -> Result<(), String> {
    let mut file = match File::create(format!("{}/{}", KEYS_DIR_PATH, id)) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to create file {}: {:?}", id, err)),
    };

    let mut rng = match rand::StdRng::new() {
        Ok(rng) => rng,
        Err(err) => return Err(format!("Creating OsRng failed: {:?}", err))
    };

    let secp256k1 = secp256k1::Secp256k1::new();
    let (secret_key, _) = match secp256k1.generate_keypair(&mut rng) {
        Ok((secret_key, public_key)) => (secret_key, public_key),
        Err(err) => return Err(format!("Creating keys failed: {:?}", err))
    };

    match file.write_all(&secret_key[0..32]) {
        Ok(()) => Ok(()),
        Err(err) => return Err(format!("Failed to write file {}: {:?}", id, err))
    }
}