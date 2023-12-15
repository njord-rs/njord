<img align="right" width="128" height="128" alt="njord" src="https://github.com/njord-rs/resources/raw/master/logo.png">

# Njord

![build](https://img.shields.io/github/actions/workflow/status/njord-rs/njord/ci.yml?branch=master)
![crates.io](https://img.shields.io/crates/v/njord.svg)
[![njord: rustc 1.74+](https://img.shields.io/badge/njord-rustc_1.74+-lightgray.svg)](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)
[![njord_derive: rustc 1.74+](https://img.shields.io/badge/njord_derive-rustc_1.74+-lightgray.svg)](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)
[![discord](https://img.shields.io/discord/1181504958802186240.svg?style=flat&color=lightgray&logo=discord)](https://discord.gg/2uppTzjUHE)

A lightweight and extensible ORM framework for Rust.

## Getting Started

### Initializing a new project

The first thing we need to do is generate our project.

```sh
cargo new --bin njord_demo
```

Now, let’s add Njord to our dependencies. We’re also going to use a tool called .env to manage our environment variables for us. We’ll add it to our dependencies as well.

```toml
[dependencies]

# The core APIs, including the Table trait. 
# using #[derive(Table)] to make njord work with structs
# and enums defined in your crate.
njord = "0.1.0"
njord_derive = "0.1.0"
dotenvy = "0.15"
```

### Installing Njord CLI

Njord provides a separate CLI tool to help manage your project. Since it’s a standalone binary, and doesn’t affect your project’s code directly, we don’t add it to `Cargo.toml`. Instead, we just install it on our system.

```sh
cargo install njord --no-default-features --features "sqlite"
```

SQlite is per default, so the above command only illustrates how you could use a different feature such as `postgres`, `mysql`, `mariadb`, `oracle` and `mssql` etc.

### Setup Njord for your project

We now use the setup command of the Njord CLI application to create our `.toml` file as well as an initial migrations directory and two `.sql` files.

```sh
njord setup
```

Two empty files now exists in a directory called `migrations`.

```
migrations/00000000000000_njord_initial_setup/up.sql
migrations/00000000000000_njord_initial_setup/down.sql
```

We also get a `njord.toml` file which is the configuration file where to load our `schema.rs` file and where to store our migrations directory. Default for the schema file is under `src/schema.rs` and migrations in the root directory. 

```toml
# For documentation on how to configure this file,
# see https://njord.rs

[schema]
file = "src/schema.rs"

[migrations_directory]
dir = "migrations"
```

## Add a schema file

Now we are going to define our schema file that we will create under `src/schema.rs`. We will store basically our structs that will map against the database. 

```rust
#[derive(Table, Default)]
pub struct User {
    user_id: usize,
    username: String,
    email: String,
    address: String,
}

#[derive(Table, Default)]
pub struct Product {
    product_id: usize,
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    category: String,
}

#[derive(Table, Default)]
pub struct Order {
    order_id: usize,
    user_id: usize,
    products: Vec<Product>,
    total_cost: f64,
    order_date: NaiveDateTime,
}
```

Now that we have that in place, we need to create the SQL for setting this up in the database so go to `migrations/00000000000000_njord_initial_setup` and open up first `up.sql` and add:

```sql
-- users table
CREATE TABLE users (
    user_id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    address TEXT NOT NULL
);

-- products table
CREATE TABLE products (
    product_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL,
    stock_quantity INTEGER NOT NULL,
    category TEXT NOT NULL
);

-- orders table
CREATE TABLE orders (
    order_id INTEGER PRIMARY KEY,
    user_id INTEGER REFERENCES users(user_id),
    total_cost REAL NOT NULL,
    order_date TEXT NOT NULL
);

-- order_products table
CREATE TABLE order_products (
    order_id INTEGER REFERENCES orders(order_id),
    product_id INTEGER REFERENCES products(product_id),
    PRIMARY KEY (order_id, product_id)
);
```

Now we need to also setup a `down.sql` file that will revert back the change.

```sql
-- Drop tables in reverse order to handle foreign key constraints

DROP TABLE IF EXISTS order_products;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS users;
```

Now we are going to generate a new migration so we can

```sh
njord migration generate --name=update_something -- --env=development
```

### Apply new schema changes

We apply the newly created migration files by running `migration run` command with the `--env` flag for which target enviroment and we can also set a `--log-level` flag to get either more or less log output.

```sh
njord migration run --env=development --log-level=debug
```

### Rollback schema changes

To rollback to last previous change we can do that by running the `rollback` command and specifying which enviroment we are in.

```sh
njord rollback --env=development --to=00000000000000
```

To rollback to a specific change we can to that by adding the flag `--to` with its ID.

```sh
njord rollback --env=development --to=00000000000000
```


## Getting Help

Are you having trouble with Njord? We want to help!

- Read through the documentation on our [docs](https://docs.rs/njord/latest/njord/).

- If you are upgrading, read the release notes for upgrade instructions and "new and noteworthy" features.

- Ask a question we monitor stackoverflow.com for questions tagged with Njord.

- Report bugs with Njord at https://github.com/mjovanc/njord/issues.

## Reporting Issues

Njord uses GitHub’s integrated issue tracking system to record bugs and feature requests. If you want to raise an issue, please follow the recommendations below:

- Before you log a bug, please search the issue tracker to see if someone has already reported the problem.

- If the issue doesn’t already exist, create a new issue.

- Please provide as much information as possible with the issue report. We like to know the Njord version, operating system, and Rust version version you’re using.

- If you need to paste code or include a stack trace, use Markdown. ``` escapes before and after your text.

- If possible, try to create a test case or project that replicates the problem and attach it to the issue.

## Contributors

The following contributors have either helped to start this project, have contributed
code, are actively maintaining it (including documentation), or in other ways
being awesome contributors to this project. **We'd like to take a moment to recognize them.**

[<img src="https://github.com/mjovanc.png?size=72" alt="mjovanc" width="72">](https://github.com/mjovanc)
[<img src="https://github.com/appelskrutt34.png?size=72" alt="appelskrutt34" width="72">](https://github.com/appelskrutt34)
[<img src="https://github.com/ahsentekdemir.png?size=72">](https://github.com/ahsentekdemir)

## License

The GPLv3 License.
