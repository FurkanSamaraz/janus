use bcrypt::{hash, DEFAULT_COST};
use std::error::Error;

pub fn bcrypt(value: &str, rounds: Option<u32>) -> Result<(), Box<dyn Error>> {
    let cost = rounds.unwrap_or(DEFAULT_COST);
    
    match hash(value, cost) {
        Ok(hash) => {
            println!("Encrypted value: {}", hash);
        }
        Err(err) => {
            eprintln!("Error encrypting value: {}", err);
        }
    }
    
    Ok(())
}

