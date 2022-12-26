# nq-api

Natiq Quran open API

# Docker

Start nq-api with docker-compose

```bash
docker-compose up
```

Restore backup to the database with this command

```bash
cat init.sql | sudo docker exec -i {container_id} psql -U {username} -W -d {database}
```

`init.sql` is a backup file of database

# Build

```bash
cargo build --release
```

# Run

```bash
./target/release/nq-api
```

Api will listen to 0.0.0.0:8080
