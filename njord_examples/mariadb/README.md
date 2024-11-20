# MariaDB

A MariaDB database will need to be spun up. This can be found in the `docker-compose.yml` file.

Run the following command:

```bash
docker-compose up -d
```

Once the database is up and running, we can run the example.

To run this example:

```bash
cargo r --bin mariadb
```
