use std::error::Error;
use base64::encode;
use std::fs::File;
use std::io::Read;

pub fn base64_encode(value: &str, file: Option<&str>) -> Result<(), Box<dyn Error>> {
    if let Some(file_path) = file {
        println!("Base64 encoding file: '{}'", file_path);
        let mut file = File::open(file_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let encoded = encode(&contents);
        println!("{}", encoded);
    } else {
        println!("Base64 encoding: '{}'", value);
        let encoded = encode(value);
        println!("{}", encoded);
    }
    Ok(())
}

