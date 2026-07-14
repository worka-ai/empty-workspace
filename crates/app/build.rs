fn main() -> Result<(), Box<dyn std::error::Error>> {
    worka::build::generate_descriptor()?;
    worka::build::generate_personaldb_schema(
        "worka/personaldb/schema.toml",
        "workspace_personaldb_schema.rs",
        "workspace_personaldb_schema",
    )?;
    Ok(())
}
