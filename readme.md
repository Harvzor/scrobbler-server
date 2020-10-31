# Drinks Drunk

An API for tracking drinks drunk.

## Running

```
cargo +nightly run
```

or

```
docker-compose up drinks-drunk
```

## Watching and running

```
cargo +nightly watch -x run
```

or

```
docker-compose up -d drinks-drunk-dev
docker-compose exec drinks-drunk-dev bash
# cargo +nightly watch --poll -x run
```

## Running migrations

This project is using [Diesel](http://diesel.rs/) as the ORM.

I didn't want to install Postgres on my system so I could use the Diesel CLI, so it's dockerized.

```
docker-compose up -d diesel-cli
docker-compose exec diesel-cli bash
# cd core
# diesel setup
# diesel migration run
```
