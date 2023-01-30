![Arrow Banner](https://github.com/Arrow-air/.github/raw/main/profile/assets/arrow_v2_twitter-banner_neu.png)

# svc-storage Service

![GitHub stable release (latest by date)](https://img.shields.io/github/v/release/Arrow-air/svc-storage?sort=semver&color=green)
![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/Arrow-air/svc-storage?include_prereleases)
![Sanity Checks](https://github.com/arrow-air/svc-storage/actions/workflows/sanity_checks.yml/badge.svg?branch=main)
![Rust
Checks](https://github.com/arrow-air/svc-storage/actions/workflows/rust_ci.yml/badge.svg?branch=main)
![Python Flake8](https://github.com/arrow-air/svc-storage/actions/workflows/python_ci.yml/badge.svg?branch=main)
![Arrow DAO
Discord](https://img.shields.io/discord/853833144037277726?style=plastic)

## Overview

svc-storage is responsible for storing and retrieving data from the Arrow database and other storage.
It's meant to be used only by other internal services via GRPC interface.
- svc_storage gRPC server - (bin) target to run gRPC server
- svc_storage_client_grpc gRPC client - (lib) target for other services to import and use
