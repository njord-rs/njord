<img align="right" width="128" height="128" alt="njord" src="https://github.com/njord-rs/resources/raw/master/logo.png">

# Njord <!-- omit in toc -->

![build](https://img.shields.io/github/actions/workflow/status/njord-rs/njord/ci.yml?branch=master)
![crates.io](https://img.shields.io/crates/v/njord.svg)
[![njord: rustc 1.77.1+](https://img.shields.io/badge/njord-rustc_1.77.1+-lightgray.svg)](https://blog.rust-lang.org/2024/03/28/Rust-1.77.1.html)
[![njord_derive: rustc 1.77+](https://img.shields.io/badge/njord_derive-rustc_1.77.1+-lightgray.svg)](https://blog.rust-lang.org/2024/03/28/Rust-1.77.1.html)
[![discord](https://img.shields.io/discord/1181504958802186240.svg?style=flat&color=lightgray&logo=discord)](https://discord.gg/2uppTzjUHE)

A lightweight and extensible ORM library for Rust.

## Table of Contents <!-- omit in toc -->
- [Supported Databases](#supported-databases)
- [Getting Started](#getting-started)
  - [Initializing a new project](#initializing-a-new-project)
  - [Add a schema file](#add-a-schema-file)
- [Usage](#usage)
  - [SQlite](#sqlite)
    - [Establish a connection](#establish-a-connection)
    - [Insert data](#insert-data)
    - [Update data](#update-data)
    - [Delete data](#delete-data)
    - [Select data](#select-data)
- [Getting Help](#getting-help)
- [Reporting Issues](#reporting-issues)
- [Contributing](#contributing)
- [Code of Conduct](#code-of-conduct)
- [Contributors](#contributors)
- [License](#license)



## Supported Databases

| Database   | Support   | Status                |
|------------|-----------|-----------------------|
| SQLite     | ✅        | Currently supported.  |
| PostgreSQL | ❌        | Not supported, help us implement it?        |
| MySQL      | ❌        | Not supported, help us implement it?        |
| MariaDB    | ❌        | Not supported, help us implement it?        |
| Oracle     | ❌        | Not supported, help us implement it?        |
| MSSQL      | ❌        | Not supported, help us implement it?        |

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
njord = { version = "<version>", features = ["sqlite"] }
njord_derive = { version = "<version>" }
```

### Add a schema file

Now we are going to define our schema file that we will create under `src/schema.rs`. We will store basically our structs that will map against the database. 

```rust
#[derive(Table)]
#[table_name = "users"]
pub struct User {
    id: usize,
    username: String,
    email: String,
    address: String,
}

#[derive(Table)]
#[table_name = "categories"]
pub struct Category {
    id: usize,
    name: String,
}

#[derive(Table)]
#[table_name = "products"]
pub struct Product {
    id: usize,
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    category: Category,     // one-to-one relationship
    discount: Option<f64>,  // allow for null 
}

#[derive(Table)]
#[table_name = "orders"]
pub struct Order {
    id: usize,
    user: User,             // one-to-one relationship
    products: Vec<Product>, // one-to-many relationship - populates from based on junction table (gets from macro attribute "table_name" and combines them for example, orders_products)
    total_cost: f64,
}
```

Now that we have that in place, we need to create the SQL for setting this up in the database and execute it.

```sql
-- users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    address TEXT NOT NULL
);

-- products table
CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL,
    stock_quantity INTEGER NOT NULL,
    category INTEGER REFERENCES categories(id),
    discount: REAL NULL
);

-- orders table
CREATE TABLE orders (
    id INTEGER PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    total_cost REAL NOT NULL
);

-- order_products table
CREATE TABLE order_products (
    order_id INTEGER REFERENCES orders(id),
    product_id INTEGER REFERENCES products(id),
    PRIMARY KEY (order_id, product_id)
);
```

## Usage

So how can we establish a connection and actually select or insert data to our database? Let's go through it. Note that these examples might be outdated, so dont treat it as a source of truth.

### SQlite 

#### Establish a connection

To establish a connection we first need to call the `sqlite::open()` function and use it with a match statement.

```rust
let db_name = "njord.db";
let db_path = Path::new(&db_name);

match sqlite::open(db_path) {
    Ok(c) => {
        println!("Database opened successfully!");
        
        // Additional logic when we are connected.
        // We need to open a connection and pass it 
        // to the corresponding sqlite function.
    }
    Err(err) => eprintln!("Error opening the database: {}", err),
}
```

#### Insert data

```rust
let user = User {
    username: String::from("john_doe"),
    email: String::from("john@example.com"),
    address: String::from("123 Main St"),
};

let result = sqlite::insert(c, vec![user]);
assert!(result.is_ok());
```

**Generated SQL**

```sql
INSERT INTO users (
    username,
    email,
    address
) VALUES (
    'john_doe',
    'john@example.com',
    '123 Main St'
)
```

#### Update data

```rust
let columns = vec!["username".to_string(), "address".to_string()];
let where_condition = Condition::Eq("username".to_string(), "john_doe".to_string());

let user = User {
    username: String::from("john_doe_2"),
    email: String::from("john@example.com"),
    address: String::from("1234 Main St"),
};

let mut order = HashMap::new();
order.insert(vec!["id".to_string()], "DESC".to_string());
        
let result = sqlite::update(c, user)
    .set(columns)
    .where_clause(where_condition)
    .order_by(order)
    .limit(4)
    .offset(0)
    .build();

assert!(result.is_ok());
```

**Generated SQL**

```sql
UPDATE users
SET
    username
WHERE
    username = 'john_doe'
ORDER BY
    id DESC
LIMIT 4
OFFSET 0

```

#### Delete data

```rust
let where_condition = Condition::Eq("username".to_string(), "john_doe".to_string());

let mut order = HashMap::new();
order.insert(vec!["id".to_string()], "DESC".to_string());

let result = sqlite::delete(c)
    .from(User::default())
    .where_clause(where_condition)
    .order_by(order)
    .limit(20)
    .offset(0)
    .build();

assert!(result.is_ok());
```

**Generated SQL**

```sql
DELETE FROM users
WHERE
    username = 'john_doe'
ORDER BY
    id DESC
LIMIT 20
OFFSET 0
```

#### Select data

```rust
let columns = vec!["id".to_string(), "username".to_string(), "email".to_string(), "address".to_string()];
let where_condition = Condition::Eq("username".to_string(), "john_doe".to_string());
let group_by = vec!["username".to_string(), "address".to_string()];

let mut order_by = HashMap::new();
order_by.insert(vec!["id".to_string()], "ASC".to_string());

let having_condition = Condition::Gt("id".to_string(), "1".to_string());

// Build the query
// We need to pass the struct User with the Default trait in .from()
let result: Result<Vec<User>> = sqlite::select(c, columns)
    .from(User::default())
    .where_clause(where_condition)
    .order_by(order_by)
    .group_by(group_by)
    .having(having_condition)
    .build();

match result {
    Ok(result) => {
        assert_eq!(result.len(), 1);
    }
    Err(error) => panic!("Failed to SELECT: {:?}", error),
};
```

**Generated SQL**

```sql
SELECT
    id,
    username,
    email,
    address
FROM
    users
WHERE
    username = 'mjovanc'
GROUP BY
    username,
    email
HAVING
    id > 1
ORDER BY
    email DESC
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

## Contributing

Before contributing, please read the [contribution](https://github.com/njord-rs/njord/blob/master/CONTRIBUTING.md) guide for useful information how to get started with Njord as well as what should be included when submitting a contribution to the project.

## Code of Conduct

Anyone who interacts with Njord in any space, including but not limited to this GitHub repository, must follow our code of conduct.

## Contributors

The following contributors have either helped to start this project, have contributed
code, are actively maintaining it (including documentation), or in other ways
being awesome contributors to this project. **We'd like to take a moment to recognize them.**

[<img src="https://github.com/mjovanc.png?size=72" alt="mjovanc" width="72">](https://github.com/mjovanc)
[<img src="https://github.com/appelskrutt34.png?size=72" alt="appelskrutt34" width="72">](https://github.com/appelskrutt34)
[<img src="https://avatars.githubusercontent.com/u/23294573?v=4&size=72">](https://github.com/ahsentekd)
[<img src="https://avatars.githubusercontent.com/u/167654108?v=4&size=72">](https://github.com/chinmer)

## License

The BSD 3-Clause License.
