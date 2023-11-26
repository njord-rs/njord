# Drop table

To drop a table, it is a simple as:

```rust
sqlite::drop_table(conn, &Posts::default());
```