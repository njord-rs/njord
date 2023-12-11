#[derive(Debug)]
pub enum Command {
    Generate(GenerateArgs),
    Run(RunArgs),
    Rollback(RollbackArgs),
}

#[derive(Debug)]
pub struct GenerateArgs {
    name: String,
    env: String,
    dry_run: bool,
    dir: String,
}

#[derive(Debug)]
pub struct RunArgs {
    env: String,
    log_level: String,
}

#[derive(Debug)]
pub struct RollbackArgs {
    env: String,
    to: String,
}

impl FromArgs for Command {
    fn from_args(matches: &clap::ArgMatches) -> Self {
        match matches.subcommand() {
            ("generate", Some(m)) => {
                let name = m.value_of("name").unwrap();
                let env = m.value_of("env").unwrap();
                let dry_run: bool = m.get_bool("dry-run").unwrap();
                let dir = m.value_of("dir").unwrap();

                Command::Generate(GenerateArgs {
                    name,
                    env,
                    dry_run,
                    dir,
                })
            }
            ("run", Some(m)) => {
                let env = m.value_of("env").unwrap();
                let log_level = m.value_of("log-level").unwrap();

                Command::Run(RunArgs { env, log_level })
            }
            ("rollback", Some(m)) => {
                let env = m.value_of("env").unwrap();
                let to = m.value_of("to").unwrap();

                Command::Rollback(RollbackArgs { env, to })
            }
            _ => panic!("Unknown subcommand"),
        }
    }
}
