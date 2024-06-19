# Define tables

To define a table we use derive with the `Table` (provided by Njord) and `Default` procedural macro. Then we create a simple struct with a given name. The name of the struct will be the table name in the database:

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