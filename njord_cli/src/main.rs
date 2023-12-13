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
                            .long("env")
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
                            .long("env")
                            .help("Target a specific environment."))
                            
                        .arg(Arg::new("log-level")
                            .help("Sets the logging level (e.g., standard, debug).")
                            .value_name("log-level")),
                )
        )
        .get_matches();

    // match a given command/subcommand and run corresponding function
    match cmd.subcommand() {
        Some(("migration", sub_matches)) => {
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
        _ => {
            eprintln!("Invalid command. Use 'njord --help' for usage information.");
            std::process::exit(1);
        },
    }
}
