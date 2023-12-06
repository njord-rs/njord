# Initialize database with tables

First thing we need to do before we do any `insert`, `select` etc is to initialize the database with the defined tables. To initialize the database we need to box the struct and call `default()` and save it to a variable:

```rust
let posts_table = Box::<Posts>::default();
let categories_table = Box::<Categories>::default();
```

Now we add them to a tables vector:

```rust
let mut tables: Vec<Box<dyn Table>> = vec![posts_table, categories_table];
```

Lastly, we call the `init()` function to setup the tables from the tables vector, such as:

```rust
sqlite::init(conn, tables);
```