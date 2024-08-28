use chrono::{DateTime, FixedOffset, Utc};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub access_token: String,
}

/// Retrieves the timezone offset from the environment variables.
///
/// # Functionality
///
/// This function performs the following steps:
/// 1. Loads environment variables from a .env file.
/// 2. Retrieves the TIMEZONE_OFFSET value from environment variables.
/// 3. Parses the offset into an integer number of hours.
/// 4. Converts the hour offset to seconds.
/// 5. Creates a FixedOffset based on whether the offset is positive (east) or negative (west).
///
/// # Returns
///
/// Returns a `Result<FixedOffset, Box<dyn std::error::Error>>`:
/// - `Ok(FixedOffset)`: A FixedOffset representing the timezone offset if successful.
/// - `Err(Box<dyn std::error::Error>)`: An error if the offset couldn't be retrieved or parsed.
///
/// # Errors
///
/// This function will return an error in the following situations:
/// - The TIMEZONE_OFFSET environment variable is not set.
/// - The TIMEZONE_OFFSET value cannot be parsed as an integer.
/// - The resulting offset is invalid (e.g., too large).
///
/// # Example
///
/// ```rust
/// match get_timezone_offset() {
///     Ok(offset) => println!("Timezone offset: {:?}", offset),
///     Err(e) => eprintln!("Error getting timezone offset: {}", e),
/// }
/// ```
///
/// # Note
///
/// Ensure that the TIMEZONE_OFFSET is set in your .env file before calling this function.
/// The offset should be an integer representing hours from UTC (e.g., -3 for Brasília Time).
pub fn get_timezone_offset() -> Result<FixedOffset, Box<dyn std::error::Error>> {
    dotenv().ok();

    let offset = env::var("TIMEZONE_OFFSET")
        .map_err(|_| "TIMEZONE_OFFSET must be set in .env file")?
        .parse::<i32>()
        .map_err(|_| "TIMEZONE_OFFSET must be a valid integer")?;

    let offset_seconds = offset * 3600;

    if offset_seconds >= 0 {
        FixedOffset::east_opt(offset_seconds)
    } else {
        FixedOffset::west_opt(-offset_seconds)
    }
    .ok_or_else(|| "Invalid timezone offset".into())
}

/// Converts a UTC datetime to the local time based on the configured timezone offset.
///
/// # Functionality
///
/// This function performs the following steps:
/// 1. Retrieves the timezone offset using the `get_timezone_offset()` function.
/// 2. Applies this offset to the provided UTC datetime to convert it to local time.
///
/// # Parameters
///
/// - `now`: A `DateTime<Utc>` representing the UTC time to be converted.
///
/// # Returns
///
/// Returns a `Result<DateTime<FixedOffset>, Box<dyn Error>>`:
/// - `Ok(DateTime<FixedOffset>)`: The local time as a DateTime with a FixedOffset if successful.
/// - `Err(Box<dyn Error>)`: An error if the timezone offset couldn't be retrieved.
///
/// # Errors
///
/// This function will return an error if `get_timezone_offset()` fails, which can happen if:
/// - The TIMEZONE_OFFSET environment variable is not set or invalid.
/// - The resulting offset is invalid.
///
/// # Example
///
/// ```rust
/// let utc_now = Utc::now();
/// match get_local_time(utc_now) {
///     Ok(local_time) => println!("Local time: {}", local_time),
///     Err(e) => eprintln!("Error converting to local time: {}", e),
/// }
/// ```
///
/// # Note
///
/// This function relies on the `get_timezone_offset()` function, so ensure that
/// the TIMEZONE_OFFSET is correctly set in your environment before use.
pub fn get_local_time(now: DateTime<Utc>) -> Result<DateTime<FixedOffset>, Box<dyn Error>> {
    let brazil_offset = get_timezone_offset()?;
    Ok(now.with_timezone(&brazil_offset))
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
