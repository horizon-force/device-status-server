# device-status-server

## About

Server to keep track of status for fire detection devices

## Requirements

- [Rust 1.75.0](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/)

## Local Development

Run the Redis Docker container (ensure Docker is running)

```shell
$ docker run -p 10001:6379 -p 13333:8001 redis/redis-stack:latest
```

Link to Redis insight: http://localhost:13333/redis-stack/browser

Run the Rust server

```shell
cargo run
```