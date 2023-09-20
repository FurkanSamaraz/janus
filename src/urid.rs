use std::error::Error;
use uuid::Uuid;

pub fn urid_generate() -> Result<(), Box<dyn Error>> {
    println!("Generating a new Maple URID compatible with the Core system");
    let id = Uuid::new_v4();
    let urid = id.to_string();
    println!("Generated URID: {}", urid);
    Ok(())
}

pub fn urid_convert(urid: &str) -> Result<(), Box<dyn Error>> {
    println!("Converting URID '{}' back into a SQL UUID", urid);
    let id = Uuid::parse_str(urid)?;
    println!("Converted UUID: {}", id);
    Ok(())
}