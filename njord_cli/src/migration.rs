use std::path::Path;

use crate::util::{create_migration_files, get_next_migration_version, read_config};

/// Generates migration files with the specified name, environment, and dry-run option.
///
/// # Arguments
///
/// * `name` - Optional parameter representing the name of the migration file.
/// * `env` - Optional parameter specifying the environment (e.g., development, test, staging, production).
/// * `dry_run` - Optional parameter indicating whether to simulate the migration without applying changes.
///
/// # Example
///
/// ```rust
/// generate(Some("example_name"), Some("development"), Some("true"));
/// ```
pub fn generate(name: Option<&String>, env: Option<&String>, dry_run: Option<&String>) {
    if let Ok(config) = read_config() {
        let migrations_dir = Path::new(&config.migrations_directory.dir);

        // get the next migration version based on existing ones
        if let Ok(version) = get_next_migration_version(migrations_dir) {
            let migration_name = name.unwrap_or(&"example_name".to_string());

            // create migration files
            if let Err(err) = create_migration_files(migrations_dir, &version, &migration_name) {
                eprintln!("Error creating migration files: {}", err);
                return;
            }

            println!("Migration files generated successfully:");
            println!("Version: {}", version);
            println!("Name: {}", migration_name);
            println!("Environment: {:?}", env);
            println!("Dry-run: {:?}", dry_run);
        } else {
            eprintln!("Error determining next migration version.");
        }
    } else {
        eprintln!("Error reading configuration file.");
    }
}

/// Runs migration files with the specified environment and log level.
///
/// # Arguments
///
/// * `env` - Optional parameter specifying the target environment for applying migrations.
/// * `log_level` - Optional parameter setting the logging level (e.g., standard, debug).
///
/// # Example
///
/// ```rust
/// run(Some("production"), Some("debug"));
/// ```
pub fn run(env: Option<&String>, log_level: Option<&String>) {
    println!(
        "Running migration with env '{:?}' and log-level '{:?}'",
        env, log_level
    );
}

/// Rolls back migration changes to a specific version, with optional environment and log level.
///
/// # Arguments
///
/// * `env` - Optional parameter specifying the target environment for rolling back migrations.
/// * `to` - Optional parameter setting a previous migration change to rollback to.
/// * `log_level` - Optional parameter setting the logging level (e.g., standard, debug).
///
/// # Example
///
/// ```rust
/// rollback(Some("development"), Some("20231204120000"), Some("info"));
/// ```
pub fn rollback(env: Option<&String>, to: Option<&String>, log_level: Option<&String>) {
    println!(
        "Rolling back migration with env '{:?}' to '{:?}' log_level '{:?}'",
        env, to, log_level
    );
}
