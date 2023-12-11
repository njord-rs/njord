mod migration;
use migration::Command;

fn main() {
    let matches = Command::new("njord")
        .version("0.1.0")
        .author("Marcus Cvjeticanin")
        .about("A simple migration tool")
        .subcommand(Command::with_name("migration"))
        .get_matches();

    let command = Command::from_args(&matches);

    match command {
        Command::Generate(args) => {
            // Implement generate command logic here
            println!("Generating migration file...");
        }
        Command::Run(args) => {
            // Implement run command logic here
            println!("Running pending migrations...");
        }
        Command::Rollback(args) => {
            // Implement rollback command logic here
            println!("Rolling back migrations...");
        }
    }
}
