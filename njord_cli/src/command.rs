use clap::ArgMatches;
use std::fs;
use std::path::Path;

use crate::migration::{generate, rollback, run};

/// Initializes Njord with an empty migrations directory and a `njord.toml` config file.
///
/// This function is responsible for setting up the initial configuration for Njord, a migration
/// tool. It does not require any command-line arguments and initializes Njord with an empty
/// migrations directory and a `njord.toml` configuration file. This allows users to start fresh
/// with an initial setup for managing database migrations.
///
/// # Example
///
/// ```rust
/// use njord_cli::handle_setup;
///
/// handle_setup();
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Notes
///
/// - The migrations directory will be empty initially.
/// - A `njord.toml` configuration file will be created with default settings.
pub fn handle_setup() {
    println!("Setting up Njord with an empty migrations directory and a njord.toml config file...");

    // include content of njord.toml template
    let toml_content = include_str!("../templates/njord.toml");

    // determine the current dir where njord is running from
    if let Ok(current_dir) = std::env::current_dir() {
        let destination_path = current_dir.join("njord.toml");

        if !destination_path.exists() {
            if let Err(err) = fs::write(&destination_path, toml_content) {
                eprintln!("Error writing njord.toml: {}", err)
            } else {
                println!("njord.toml successfully copied to the current directory.")
            }
        } else {
            println!("njord.toml already exists in the current directory. Skipping copy.")
        }
    } else {
        eprintln!("Error determining the current directory.")
    }
}

/// Handles the "migration" subcommand based on the provided `ArgMatches`.
///
/// # Arguments
///
/// * `sub_matches` - The `ArgMatches` object containing subcommand-specific matches.
///
/// # Example
///
/// ```rust
/// use clap::App;
/// use crate::handle_migration_subcommand;
///
/// let matches = App::new("MyApp").get_matches();
/// let sub_matches = matches.subcommand_matches("migration").unwrap();
/// handle_migration_subcommand(sub_matches);
/// ```
pub fn handle_migration_subcommand(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("generate", generate_matches)) => {
            let name = generate_matches.get_one::<String>("name");
            let env = generate_matches.get_one::<String>("env");
            let dry_run = generate_matches.get_one::<String>("dry-run");

            generate(name, env, dry_run)
        }
        Some(("run", run_matches)) => {
            let env = run_matches.get_one::<String>("env");
            let log_level = run_matches.get_one::<String>("log-level");

            run(env, log_level)
        }
        Some(("rollback", rollback_matches)) => {
            let env = rollback_matches.get_one::<String>("env");
            let to = rollback_matches.get_one::<String>("to");
            let log_level = rollback_matches.get_one::<String>("log-level");

            rollback(env, to, log_level)
        }
        _ => {
            eprintln!("Invalid subcommand for 'migration'. Use 'njord migration --help' for usage information.");
            std::process::exit(1);
        }
    }
}

/// Handles the top-level command based on the provided command name and `ArgMatches`.
///
/// # Arguments
///
/// * `cmd` - The name of the command.
/// * `sub_matches` - The `ArgMatches` object containing command-specific matches.
///
/// # Example
///
/// ```rust
/// use clap::App;
/// use crate::handle_command;
///
/// let matches = App::new("MyApp").get_matches();
/// let cmd = "migration";
/// let sub_matches = matches.subcommand_matches(cmd).unwrap();
/// handle_command(cmd, sub_matches);
/// ```
pub fn handle_command(cmd: &str, sub_matches: &ArgMatches) {
    match cmd {
        "migration" => handle_migration_subcommand(sub_matches),
        "setup" => handle_setup(),
        _ => {
            eprintln!("Invalid command. Use 'njord --help' for usage information.");
            std::process::exit(1);
        }
    }
}
