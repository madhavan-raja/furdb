<div align="center">
  <h1>FurDB</h1>

  <img src="https://img.shields.io/badge/api-main-green.svg" />
  <img src="https://img.shields.io/badge/rustc-1.58+-orange.svg" />

  <br>
</div>

A small and space-efficient Database Management System that allows you to allocate the size of individual data types in bits (not bytes).

```
10011100 01010000
┌─┐┌───────┐┌───┐
  ^        ^    ^
  d1       d2   d3
```

## Installing

#### Cargo

**FurDB** can be directly used with `cargo`.

```sh
cargo install furdb
furdb
```

#### Compiling from Source

You can clone this repository, build and run the program.

```sh
git clone https://github.com/furdb/furdb.git
cd ./furdb
cargo run --release
```

#### Docker

You can pull an image and run it in a container.

```sh
docker run --name some-furdb -d madhavanraja/furdb:latest
```

You can clone this repository, build and run the container.

```sh
git clone https://github.com/furdb/furdb.git
cd ./furdb
docker-compose up --build
```

You can use the image as a service.

```yaml
version: "3"
services:
  db:
    image: madhavanraja/furdb:latest
    environment:
      FUR_DIRECTORY: /furdb
      FURDB_SERVER: 8080
    restart: on-failure
```

The server can be accessed at `http://db:{PORT}`.

## Usage

The documentation of this project will be moved to [crates.io](https://crates.io/crates/furdb) at some point in the future.

Right now, you can import the [`postman_collection`](furdb.postman_collection.json) into Postman and analyze the endpoints.
