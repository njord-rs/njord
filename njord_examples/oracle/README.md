# Oracle

A Oracle database will need to be spun up. This can be found in the `docker-compose.yml` file.

Run the following command:

```bash
docker-compose up -d
```

Additionally, ODPI-C will need to be installed to communicate with the docker container:

https://odpi-c.readthedocs.io/en/latest/user_guide/installation.html#overview

Once the database is up and running, we can run the example.

To run this example:

```bash
cargo r --bin oracle
```
