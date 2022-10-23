![Arrow Banner](https://github.com/Arrow-air/.github/raw/main/profile/assets/arrow_v2_twitter-banner_neu.png)

# svc-storage Service

![Rust
Checks](https://github.com/arrow-air/svc-storage/actions/workflows/rust_ci.yml/badge.svg?branch=main)
![Python Flake8](https://github.com/arrow-air/svc-storage/actions/workflows/python_ci.yml/badge.svg?branch=main)
![Arrow DAO
Discord](https://img.shields.io/discord/853833144037277726?style=plastic)

## :telescope: Overview
svc-storage is responsible for storing and retrieving data from the Arrow database and other storage. 
It's meant to be used only by other internal services via GRPC interface.
- server - [bin] target to run gRPC server 
- client - [lib] target for other services to import and use

Directory:
- `server/src/`: Source Code and Unit Tests of the server
- `client/src/`: Source Code and Unit Tests of the client
- `tests/`: Integration Tests
- `docs/`: Module Documentation

## Installation

Install Rust with [Rustup](https://www.rust-lang.org/tools/install).

```bash
# After cloning the repository
python3 -m pip install -r requirements.txt

# Adds custom pre-commit hooks to .git through cargo-husky dependency
# !! Required for developers !!
cargo test
```

## :scroll: Documentation
The following documents are relevant to this service:
- [Concept of Operations](./docs/conops.md)
- [Requirements & User Stories](./docs/requirements.md)
- [ICD](./docs/icd.md)
- [SDD](./docs/sdd.md)

## :busts_in_silhouette: Arrow DAO
Learn more about us:
- [Website](https://www.arrowair.com/)
- [Arrow Docs](https://www.arrowair.com/docs/intro)
- [Discord](https://discord.com/invite/arrow)

## :exclamation: Treatment of `Cargo.lock`
If you are building a non-end product like a library, include `Cargo.lock` in `.gitignore`.

If you are building an end product like a command line tool, check `Cargo.lock` to the git. 

Read more about it [here](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html);

## Cockroachdb
The database connection requires TLS. When running cockroachdb in development, you will need to generate certificates which can be used by the server.

### Cockroachdb key generation
TODO: automate this so developers don't need to think about it. Can be a make target to check if the certs and keys dir exist and creates them if not present.

```
mkdir keys certs
cockroach cert create-ca --certs-dir=certs --ca-key=keys/ca.key
cockroach cert create-client root --certs-dir=certs --ca-key=keys/ca.key
cockroach cert create-node localhost $(hostname) --certs-dir=certs --ca-key=keys/ca.key
```

### Required environment
The following Cockroachdb related environment variables need to be exposed to the server

```
PG__USER=svc_storage
PG__DBNAME=arrow
PG__PASSWORD=arrow-dev
PG__HOST=localhost
PG__PORT=26257
PG__SSLMODE=require
DB_CA_CERT=certs/ca.crt
```

They will be read from a `.env` file if present.
