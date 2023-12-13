pub fn generate(name: Option<&String>, env: Option<&String>, dry_run: Option<&String>) {
    println!(
        "Generating migration file '{:?}' env '{:?}', dry-run: {:?}",
        name, env, dry_run
    );
}

pub fn run(env: Option<&String>, log_level: Option<&String>) {
    println!(
        "Running migration with env '{:?}' and log-level '{:?}'",
        env, log_level
    );
}

pub fn rollback(env: Option<&String>, to: Option<&String>, log_level: Option<&String>) {
    println!(
        "Rolling back migration with env '{:?}' to '{:?}' log_level '{:?}'",
        env, to, log_level
    );
}
