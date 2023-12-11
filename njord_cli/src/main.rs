use clap::Arg;

fn main() {
    let cmd = clap::Command::new("njord")
        .bin_name("njord")
        .author("Marcus Cvjeticanin. <mjovanc@icloud.com>")
        .about("Njord CLI ⛵ for handling migration changes.")
        .subcommand_required(true)
        .subcommand(
            clap::command!("migration")
                .subcommand(clap::command!("generate").arg(Arg::new("name"))),
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("migration", _migration_matches)) => {
            println!("Hello!")
        }
        _ => {
            eprintln!("Invalid command. Use 'njord --help' for usage information.");
            std::process::exit(1);
        }
    }
}
