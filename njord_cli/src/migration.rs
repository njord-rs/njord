use std::{fs, path::Path};
use std::ops::Deref;

use njord::sqlite;
use rusqlite::{Connection, Error, ErrorCode};

use crate::util::{create_migration_files, get_local_migration_versions, get_migrations_directory_path, get_next_migration_version, version_in_database, MigrationHistory, read_config};

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
        if let Some(migrations_dir) = get_migrations_directory_path(&config) {
            // get the next migration version based on existing ones
            if let Ok(next_version) = get_next_migration_version(&migrations_dir) {
                let migration_name = name.map(|s| s.as_str()).unwrap_or("example_name");

                // create migration files
                if let Err(err) = create_migration_files(&migrations_dir, &next_version, migration_name)
                {
                    eprintln!("Error creating migration files: {}", err);
                    return;
                }

                println!("Migration files generated successfully:");
                println!("Version: {}", next_version);
                println!("Name: {}", migration_name);
                println!("Environment: {:?}", env);
                println!("Dry-run: {:?}", dry_run);
            } else {
                eprintln!("Error determining next migration version.");
            }
        } else {
            eprintln!("Error determining migrations directory.");
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
    if let Ok(config) = read_config() {
        if let Some(migrations_dir) = get_migrations_directory_path(&config) {
            let conn = sqlite::open("sqlite.db");

            match conn {
                Ok(conn) => {
                    println!("Database connection established successfully.");

                    // obtain the latest version from the "migration_history" table
                    if let Ok(latest_db_version) = get_latest_migration_version(&conn) {
                        println!("latest_db_version: {}", latest_db_version);

                        // get all local migration changes
                        let local_versions = match get_local_migration_versions(&migrations_dir) {
                            Ok(versions) => versions,
                            Err(err) => {
                                eprintln!("Error retrieving local migration versions: {}", err);
                                return;
                            }
                        };

                        for value in &local_versions {
                            println!("Value: {}", value);
                        }

                        for local_version in &local_versions {
                            match version_in_database(&conn, &local_version) {
                                Ok(_) => {
                                    println!("Version {} not found in database. Executing code...", local_version);

                                    let migrations_dir = format!("migrations/{}", local_version);
                                    execute_pending_migration(&conn, &migrations_dir, &local_version).unwrap();
                                }
                                Err(_) => {}
                            }
                        }
                    } else {
                        eprintln!("Error obtaining latest migration version.");
                    }
                }
                Err(err) => eprintln!("Error establishing database connection: {}", err),
            };

            println!(
                "Running migration with env '{:?}' and log-level '{:?}'",
                env, log_level
            );
        }
    }
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
    if let Some(target_version) = to {
        let conn = sqlite::open("sqlite.db");

        match conn {
            Ok(conn) => {
                println!("Database connection established successfully.");

                // construct paths to migration directories
                let migrations_dir = format!("migrations/{}", target_version);

                // execute down.sql for the specified version
                if let Err(down_err) = execute_sql_from_file(&conn, &migrations_dir, "down.sql") {
                    eprintln!("Error executing down.sql: {}", down_err);
                } else {
                    println!("down.sql executed successfully.");
                }
            }
            Err(err) => eprintln!("Error establishing database connection: {}", err),
        };

        println!(
            "Rolling back migration with env '{:?}' to '{:?}' log_level '{:?}'",
            env, target_version, log_level
        );
    } else {
        eprintln!("Error: Please provide a target version to rollback to.");
    }
}

/// Retrieves the latest migration version from the "migration_history" table.
///
/// # Arguments
///
/// * `conn` - A reference to a `rusqlite::Connection`.
///
/// # Returns
///
/// A `Result` containing the latest migration version as a `String` or a `rusqlite::Error` if an error occurs.
///
/// # Note
///
/// This function queries the "migration_history" table to obtain the latest version.
fn get_latest_migration_version(conn: &Connection) -> Result<String, Error> {
    let query = "SELECT version FROM migration_history ORDER BY version DESC LIMIT 1;";
    let result: Result<String, Error> = conn.query_row(query, [], |row| row.get(0));

    match result {
        Ok(version) => Ok(version),
        Err(err) => {
            match err {
                Error::SqliteFailure(error, _) if error.code == ErrorCode::Unknown => {
                    Ok("00000000000000_njord_initial_setup".to_string())
                }
                _ => {
                    eprintln!("Error getting latest migration version: {}", err);
                    Err(err)
                }
            }
        }
    }
}

/// Executes SQL content from a file on the provided database connection.
///
/// # Arguments
///
/// * `conn` - A reference to a `rusqlite::Connection`.
/// * `migrations_dir` - A string slice representing the path to the migrations directory.
/// * `file_name` - A string slice representing the name of the SQL file to execute.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or a `rusqlite::Error` if an error occurs.
///
/// # Note
///
/// This function reads the SQL content from the specified file and executes it on the provided database connection.
fn execute_sql_from_file(
    conn: &Connection,
    migrations_dir: &str,
    file_name: &str,
) -> Result<(), Error> {
    let file_path = Path::new(migrations_dir).join(file_name);

    let sql_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading {}: {}", file_path.display(), err);
            return Err(Error::QueryReturnedNoRows); // placeholder error
        }
    };

    match conn.execute_batch(&sql_content) {
        Ok(_) => {
            println!("{} executed successfully.", file_path.display());
            Ok(())
        }
        Err(err) => {
            eprintln!("Error executing {}: {}", file_path.display(), err);
            Err(err)
        }
    }
}

/// Executes migration changes in directories that have not been applied yet.
///
/// # Arguments
///
/// * `conn` - A reference to a `rusqlite::Connection`.
/// * `migrations_dir` - A string slice representing the path to the migrations directory.
/// * `next_version` - A string representing the next migration version.
///
/// # Errors
///
/// Returns a `rusqlite::Error` if there is an issue executing the SQL scripts or inserting into the database.
fn execute_pending_migration(
    conn: &Connection,
    migrations_dir: &str,
    next_version: &str,
) -> Result<(), Error> {
    // Execute up.sql in the directory
    match execute_sql_from_file(&conn, &migrations_dir, "up.sql") {
        Ok(_) => {
            println!("up.sql executed successfully.");
            // Insert new row with the version into the database
            let row = MigrationHistory { version: next_version.to_string() };
            // sqlite::insert(conn, &row)?;
        }
        Err(up_err) => {
            // If up.sql fails, run down.sql
            eprintln!("Error executing up.sql: {}", up_err);
            if let Err(down_err) = execute_sql_from_file(&conn, &migrations_dir, "down.sql") {
                eprintln!("Error executing down.sql: {}", down_err);
            } else {
                println!("down.sql executed successfully.");
            }
        }
    }
    Ok(())
}

