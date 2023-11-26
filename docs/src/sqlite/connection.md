# Setup connection

First thing you need to do is to setup a connection is:

```rust
let conn = sqlite::open("my_database.db");
```

We can also use a in-memory database:

```rust
let conn = sqlite::open_in_memory("my_database.db");
```