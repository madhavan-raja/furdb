<div align="center">
  <h1>FurDB</h1>

[![Docker Image CI](https://github.com/madhavan-raja/furdb/actions/workflows/docker-image.yml/badge.svg)](https://github.com/madhavan-raja/furdb/actions)
[![Minimum rustc 1.70](https://img.shields.io/badge/rustc-1.70+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![oq3_semantics crate](https://img.shields.io/crates/v/furdb.svg)](https://crates.io/crates/furdb)

</div>

A minimal Database Management System that prioritizes storage space usage and fast lookup/query times. **FurDB** lets you specify the specific number of bits occupied by your data.

```
10011100 01010000
┌─┐┌───────┐┌───┐
  ^        ^    ^
  d1       d2   d3
```

## Installing

### Cargo

**FurDB** can be directly installed using `cargo`.

```sh
cargo install furdb
```

### Compiling from Source

You can clone this repository, build and run the program.

```sh
git clone https://github.com/madhavan-raja/furdb.git
cd ./furdb
cargo build --release
```

## Starting the Server

### Docker

You can pull an image and run it in a container.

```sh
docker run --name furdb -d madhavanraja/furdb:latest
```

You can clone this repository, build and run the container using `compose`.

```sh
git clone https://github.com/madhavan-raja/furdb.git
cd ./furdb
docker-compose up --build
```

You can use the image as a service in a `compose` in another application.

```yaml
version: "3"
services:
  furdb:
    image: madhavanraja/furdb:latest
    environment:
      WORKDIR: /furdb
      PORT: 5678
    restart: on-failure
```

The server can be accessed at `http://furdb:{PORT}`.

### Command Line

If the executable is present in your `PATH`, you can directly run the application.

```sh
furdb serve --port 5678 --workdir "/furdb"
```

You can run the `help` command to see all the available options.

```sh
furdb help
```

## Usage

**FurDB** provides REST API endpoints for creating, reading, and deleting databases, tables, and entries.

### Checking Server Health

`GET` `/`

### Create Database

`POST` `/:database_id`

```json
{
  "database_name": "Database Name"
}
```

### Get Database Info

`GET` `/:database_id`

### Delete Database

`DELETE` `/:database_id`

### Create Table

`POST` `/:database_id/:table_id`

```json
{
  "table_name": "Table Name",
  "table_columns": [
    {
      "name": "Column 1 Name",
      "size": 5
    },
    {
      "name": "Column 2 Name",
      "size": 3
    }
  ]
}
```

### Get Table Info

`GET` `/:database_id/:table_id`

### Delete Table

`DELETE` `/:database_id/:table_id`

### Insert Entries

`POST` `/:database_id/:table_id/data`

```json
{
  "data": [
    [15, 0],
    [20, 1]
  ]
}
```

### Get Entries

`GET` `/:database_id/:table_id/data`

#### Get All Entries

```json
{
  "entries": "All"
}
```

#### Get Entries By Indices

```json
{
  "entries": {
    "ByIndices": [2]
  }
}
```

#### Get Entries By Value

```json
{
  "entries": {
    "ByValue": {
      "column_index": 0,
      "value": 12
    }
  }
}
```

### Delete Entries

`DELETE` `/:database_id/:table_id/data`

```json
{
  "indices": [0, 10]
}
```
