<img align="right" width="128" height="128" alt="njord" src="https://github.com/mjovanc/njord/raw/master/resources/logo.png">

# njord

![build](https://img.shields.io/github/actions/workflow/status/mjovanc/njord/master-ci.yml?branch=master)
[![license](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
![](https://img.shields.io/badge/Rust-1.73+-orange.svg)

A lightweight ORM library for Rust with strong-typed SQL DSL and sequence APIs.

## Getting Help

Are you having trouble with njord? We want to help!

- If you are upgrading, read the release notes for upgrade instructions and "new and noteworthy" features.

- Ask a question we monitor stackoverflow.com for questions tagged with njord.

- Report bugs with njord at https://github.com/mjovanc/njord/issues.

## Reporting Issues

njord uses GitHub’s integrated issue tracking system to record bugs and feature requests. If you want to raise an issue, please follow the recommendations below:

- Before you log a bug, please search the issue tracker to see if someone has already reported the problem.

- If the issue doesn’t already exist, create a new issue.

- Please provide as much information as possible with the issue report. We like to know the njord version, operating system, and Rust version version you’re using.

- If you need to paste code or include a stack trace, use Markdown. ``` escapes before and after your text.

- If possible, try to create a test case or project that replicates the problem and attach it to the issue.

## Get started

To install njord into your Rust project we need to include the dependencies:

**Cargo.toml**
```toml
[dependencies]

# The core APIs, including the Table trait. Always
# required when using njord. The "derive" feature is only required when
# using #[derive(Table)] to make njord work with structs
# and enums defined in your crate.
njord = { version = "0.1.0", features = ["derive"] }
```

## Usage

Currently njord is very limited in functionality, but has some basic SQL operations. The ORM is currently only supported for SQLite, but once most functionality is in place, more database types will be added.

### SQLite

First thing you need to do is to setup a connection:

```rust
let conn = sqlite::open("my_database.db");
```

Then you need to define your tables that will be initialized:

```rust
#[derive(Table, Default)]
struct Posts {
    title: String,
    description: String,
}

#[derive(Table, Default)]
struct Categories {
    name: String,
}
```

And then we box our tables and push them to a tables vector:

```rust
let posts_table = Box::new(Posts::default());
let categories_table = Box::new(Categories::default());

let mut tables: Vec<Box<dyn Table>> = Vec::new();
tables.push(posts_table);
tables.push(categories_table);
```

Now we simply just initialize the database with the init() function by sending in the database connection and tables we created:

```rust
sqlite::init(conn.unwrap(), tables);
```

Now we have two tables in our database `Posts` and `Categories`. Let's insert some data:

```rust
let table_row: Posts = Posts {
    title: "A post title".to_string(),
    description: "Some description for for a post".to_string(),
};
```

Then we insert the row by passing in the connection again and a reference to the table_row:

```rust
sqlite::insert(conn.unwrap(), &table_row);
```

## Contributors

The following contributors have either helped to start this project, have contributed
code, are actively maintaining it (including documentation), or in other ways
being awesome contributors to this project. **We'd like to take a moment to recognize them.**

[<img src="https://github.com/mjovanc.png?size=72" alt="mjovanc" width="72">](https://github.com/mjovanc)
[<img src="https://github.com/renovatebot.png?size=72" alt="mjovanc" width="72">](https://github.com/renovatebot)

## License

The 3-Clause BSD License.