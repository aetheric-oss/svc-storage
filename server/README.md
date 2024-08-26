![Aetheric Banner](https://github.com/aetheric-oss/.github/blob/main/assets/readme-banner.png)

# svc-storage Service

![GitHub stable release (latest by date)](https://img.shields.io/github/v/release/aetheric-oss/svc-storage?sort=semver&color=green) ![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/aetheric-oss/svc-storage?include_prereleases) [![Coverage Status](https://coveralls.io/repos/github/aetheric-oss/svc-storage/badge.svg?branch=develop)](https://coveralls.io/github/aetheric-oss/svc-storage)
![Sanity Checks](https://github.com/aetheric-oss/svc-storage/actions/workflows/sanity_checks.yml/badge.svg?branch=develop) ![Python PEP8](https://github.com/aetheric-oss/svc-storage/actions/workflows/python_ci.yml/badge.svg?branch=develop) ![Rust Checks](https://github.com/aetheric-oss/svc-storage/actions/workflows/rust_ci.yml/badge.svg?branch=develop)
![Arrow DAO Discord](https://img.shields.io/discord/853833144037277726?style=plastic)

## Overview

svc-storage is responsible for storing and retrieving data from the Realm database and other storage.
It's meant to be used only by other internal services via gRPC interface.
- svc_storage gRPC server - (bin) target to run gRPC server
- svc_storage_client_grpc gRPC client - (lib) target for other services to import and use
