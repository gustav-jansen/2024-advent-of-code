use std::fs;
use std::error::Error;
use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    Ok(())
}
