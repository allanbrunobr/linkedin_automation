use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub access_token: String,
}

/// Loads a configuration from a TOML file.
///
/// This function reads the contents of a specified file, parses it as a TOML document,
/// and deserializes it into a `Config` struct.
///
/// # Arguments
///
/// * `file_path` - A string slice representing the path to the TOML file.
///
/// # Returns
///
/// * `Ok(Config)` - If the file is successfully read, parsed, and deserialized,
///   the function returns a `Result` containing the `Config` instance.
///
/// * `Err(Box<dyn std::error::Error>)` - If any error occurs during the file reading, parsing,
///   or deserialization process, the function returns a `Result` containing an error.
pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_string = std::fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&config_string)?;
    Ok(config)
}
