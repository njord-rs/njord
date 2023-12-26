use std::{fs, path::Path};

use njord::sqlite;
use rusqlite::Connection;

use crate::util::{
    create_migration_files, get_migrations_directory_path, get_next_migration_version, read_config,
};

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
            if let Ok(version) = get_next_migration_version(&migrations_dir) {
                let migration_name = name.map(|s| s.as_str()).unwrap_or("example_name");

                // create migration files
                if let Err(err) = create_migration_files(&migrations_dir, &version, migration_name)
                {
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
    //TODO: this doesnt load since it looks in the wrong directory
    // need to update the open() function to look for either ../target or ./target dir
    // it should not be hardcoded here as well, we need a more elgant solution
    let conn = sqlite::open("sqlite.db");

    match conn {
        Ok(conn) => {
            println!("Database connection established successfully.");

            // obtain the latest version from the "migration_history" table
            if let Ok(latest_version) = get_latest_migration_version(&conn) {
                // construct paths to migration directories
                let migrations_dir = format!("migrations/{}", latest_version);

                // execute up.sql in directories that have not been applied yet
                if let Err(up_err) = execute_sql_from_file(&conn, &migrations_dir, "up.sql") {
                    // if up.sql fails, run down.sql
                    if let Err(down_err) = execute_sql_from_file(&conn, &migrations_dir, "down.sql")
                    {
                        eprintln!("Error executing down.sql: {}", down_err);
                    } else {
                        println!("down.sql executed successfully.");
                    }

                    eprintln!("Error executing up.sql: {}", up_err);
                } else {
                    println!("up.sql executed successfully.");
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
        //TODO: this doesn't load since it looks in the wrong directory
        // need to update the open() function to look for either ../target or ./target dir
        // it should not be hardcoded here as well, we need a more elgant solution
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
fn get_latest_migration_version(conn: &Connection) -> Result<String, rusqlite::Error> {
    let query = "SELECT version FROM migration_history ORDER BY version DESC LIMIT 1;";
    let result: Result<String, rusqlite::Error> = conn.query_row(query, [], |row| row.get(0));

    result.map_err(|err| {
        eprintln!("Error getting latest migration version: {}", err);
        err
    })
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
) -> Result<(), rusqlite::Error> {
    let file_path = Path::new(migrations_dir).join(file_name);

    let sql_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading {}: {}", file_path.display(), err);
            return Err(rusqlite::Error::QueryReturnedNoRows); // placeholder error
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
