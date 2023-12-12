mod migration;
use clap::Arg;
use migration::{generate, rollback, run};

fn main() {
    let cmd = clap::Command::new("njord")
        .bin_name("njord")
        .author("Marcus Cvjeticanin. <mjovanc@icloud.com>")
        .about("Njord CLI ⛵ for handling migration changes.")
        .subcommand_required(true)
        .subcommand(
            clap::command!("migration")
                .subcommand(
                    clap::command!("generate")
                        .about("Generates a new migration file with the specified name.")
                        
                        .arg(Arg::new("env")
                            .long("env")
                            .help("Specifies the environment (e.g., development, test, staging, production).")
                            .value_name("env"))
                        
                        .arg(Arg::new("log-level")
                            .long("log-level")
                            .help("Sets the logging level (e.g., standard, debug).")
                            .value_name("log-level"))
                        
                        .arg(Arg::new("dry-run")
                            .long("dry-run")
                            .help("Simulates the migration without applying changes."))

                        .arg(Arg::new("dir")
                            .long("dir")
                            .help("Specifies the target directory for generated migration changes.")
                            .value_name("path"))
                )
                .subcommand(
                    clap::command!("run")
                        .about("Applies all pending migrations to the database.")
                        .arg(Arg::new("env")
                            .long("env")
                            .help("Target a specific environment.")),
                )
                .subcommand(
                    clap::command!("rollback")
                        .about("Rolls back the last applied migration or to a specific version.")
                        .arg(Arg::new("to")
                            .long("to")
                            .help("Sets a previous migration change to rollback to (e.g. --to=20231204120000")
                            .value_name("change")),
                )
        )
        .get_matches();

    // match a given command/subcommand and run corresponding function
    match cmd.subcommand_name() {
        Some("migration") => generate(), // we should not run generate here, we should have another match statement to check the next subcommand which is generate/run/rollback
        _ => {
            eprintln!("Invalid command. Use 'njord --help' for usage information.");
            std::process::exit(1);
        },
    }
}
