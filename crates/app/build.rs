fn main() -> Result<(), Box<dyn std::error::Error>> {
    worka::build::generate_descriptor()?;
    Ok(())
}
