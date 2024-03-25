use core::fmt;
use serde::Deserialize;
use std::error::Error as StdError;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::{env, fs};
use chrono::{DateTime, Local};
use toml::Value as TomlConfig;
use njord_derive::Table;
use njord::table::Table;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Toml(toml::de::Error),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    migrations_directory: Option<MigrationsDirectory>,
    schema_file: Option<SchemaFile>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MigrationsDirectory {
    dir: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SchemaFile {
    file: String,
}

#[derive(Table)]
#[table_name = "migration_history"]
pub struct MigrationHistory {
    pub version: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "IO error: {}", err),
            ConfigError::Toml(err) => write!(f, "TOML error: {}", err),
        }
    }
}

impl StdError for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Toml(err)
    }
}

/// Reads the configuration from the file.
///
/// This function reads the content of the `njord.toml` file located in the root
/// of the repository, parses it into a `Config` struct, and returns the result.
/// If any error occurs during the file reading or parsing, it returns a
/// `ConfigError`.
///
/// # Errors
///
/// Returns a `ConfigError` if there is an issue with reading the file or
/// parsing its content.
///
pub fn read_config() -> Result<TomlConfig, ConfigError> {
    let current_dir = env::current_dir()?;

    // construct the path to njord.toml in the root of the repository
    let config_path = current_dir.join("njord.toml");

    // read the content of njord.toml
    let config_content = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading config.toml: {}", err);
            return Err(err.into());
        }
    };

    // parse the content
    let config: TomlConfig = match toml::from_str(&config_content) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing config.toml: {}", err);
            return Err(err.into());
        }
    };

    Ok(config)
}

/// Gets the next migration version based on the existing ones in the migrations directory.
///
/// This function reads the existing migration versions from the specified
/// `migrations_dir`, determines the maximum version, increments it, and
/// returns the next migration version as a string.
///
/// # Arguments
///
/// * `migrations_dir` - The path to the directory containing existing
///   migration versions.
///
/// # Errors
///
/// Returns a `std::io::Error` if there is an issue reading the existing
/// migration versions.
///
pub fn get_next_migration_version(migrations_dir: &Path) -> Result<String, std::io::Error> {
    let entries = fs::read_dir(migrations_dir)?;
    let versions: Vec<u64> = entries
        .filter_map(|entry| {
            entry.ok()
                .and_then(|e| e.file_name().to_str().map(String::from))
        })
        .filter_map(|version| {
            // Split the version string by '_' and take the first part
            version.split('_').next().unwrap_or_default().parse().ok()
        })
        .collect();

    let max_version = versions.into_iter().max();

    match max_version {
        Some(max_value) => {
            let next_version = max_value + 1;
            Ok(format!("{:014}", next_version))
        }
        None => Ok("00000000000001_unknown".to_string()), // initial version
    }
}

/// Creates migration files in the specified directory.
///
/// This function creates migration files in the specified directory based on
/// the provided `version` and `name`. It creates two files: `up.sql` and
/// `down.sql` within the migration directory.
///
/// # Arguments
///
/// * `migrations_dir` - The path to the directory where migration files will be created.
/// * `version` - The version of the migration.
/// * `name` - The name of the migration.
///
/// # Errors
///
/// Returns a `std::io::Error` if there is an issue creating the migration files.
///
pub fn create_migration_files(
    migrations_dir: &Path,
    version: &str,
    name: &str,
) -> Result<(), std::io::Error> {
    let mut dir_path = PathBuf::from(migrations_dir);
    dir_path.push(format!("{}_{}", version, name));

    println!("{:?}", dir_path);

    fs::create_dir_all(&dir_path)?;

    let up_sql_path = dir_path.join("up.sql");
    let down_sql_path = dir_path.join("down.sql");

    fs::File::create(up_sql_path)?;
    fs::File::create(down_sql_path)?;

    Ok(())
}

pub fn get_migrations_directory_path(config: &TomlConfig) -> Option<PathBuf> {
    let migrations_dir = config
        .get("migrations_directory") // Adjust this based on the actual structure of your toml::Value
        .and_then(|value| value.get("dir"))
        .and_then(|dir| dir.as_str())
        .map(PathBuf::from);

    migrations_dir
}
