mod migration;
mod command;
mod util;
use clap::Arg;
use command::handle_command;

fn main() {
    let cmd = clap::Command::new("njord")
        .bin_name("njord")
        .author("Marcus Cvjeticanin. <mjovanc@icloud.com>")
        .about("Njord CLI ⛵ for handling migration changes.")
        .subcommand_required(true)
        .subcommand(clap::command!("setup").about("Initializes Njord with an empty migrations directory and a njord.toml config file."))
        .subcommand(
            clap::command!("migration")
                .subcommand(
                    clap::command!("generate")
                        .about("Generates a new migration file with the specified name.")

                        .arg(Arg::new("name")
                            .help("Specifies the name of the schema change.")
                            .value_name("name"))
                        
                        .arg(Arg::new("env")
                            .help("Specifies the environment (e.g., development, test, staging, production).")
                            .value_name("env"))
                        
                        .arg(Arg::new("log-level")
                            .help("Sets the logging level (e.g., standard, debug).")
                            .value_name("log-level"))
                        
                        .arg(Arg::new("dry-run")
                            .help("Simulates the migration without applying changes."))

                        .arg(Arg::new("dir")
                            .help("Specifies the target directory for generated migration changes.")
                            .value_name("path"))
                )
                .subcommand(
                    clap::command!("run")
                        .about("Applies all pending migrations to the database.")
                        
                        .arg(Arg::new("env")
                            .help("Target a specific environment."))
                        
                        .arg(Arg::new("log-level")
                            .help("Sets the logging level (e.g., standard, debug).")
                            .value_name("log-level")),
                )
                .subcommand(
                    clap::command!("rollback")
                        .about("Rolls back the last applied migration or to a specific version.")
                        
                        .arg(Arg::new("to")
                            .long("to")
                            .help("Sets a previous migration change to rollback to (e.g. --to=20231204120000")
                            .value_name("change"))
                        
                        .arg(Arg::new("env")
                            .help("Target a specific environment."))
                            
                        .arg(Arg::new("log-level")
                            .help("Sets the logging level (e.g., standard, debug).")
                            .value_name("log-level")),
                )
        )
        .get_matches();

    if let Some((cmd, sub_matches)) = cmd.subcommand() {
        handle_command(cmd, sub_matches);
    } else {
        eprintln!("Invalid command. Use 'njord --help' for usage information.");
        std::process::exit(1);
    }
}
