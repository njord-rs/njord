# Insert data to table

To insert data, we need to initialize our Posts struct that we created with values for each field:

```rust
let table_row: Posts = Posts {
    title: "A post title".to_string(),
    description: "Some description for for a post".to_string(),
};
```

We insert the row by passing a connection and a reference to the table_row:

```rust
sqlite::insert(conn, &table_row);
```