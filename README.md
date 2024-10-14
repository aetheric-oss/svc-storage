![Aetheric Banner](https://github.com/aetheric-oss/.github/raw/main/assets/readme-banner.png)

# svc-storage Service

![GitHub stable release (latest by date)](https://img.shields.io/github/v/release/aetheric-oss/svc-storage?sort=semver&color=green) ![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/aetheric-oss/svc-storage?include_prereleases) [![Coverage Status](https://coveralls.io/repos/github/aetheric-oss/svc-storage/badge.svg?branch=develop)](https://coveralls.io/github/aetheric-oss/svc-storage)
![Sanity Checks](https://github.com/aetheric-oss/svc-storage/actions/workflows/sanity_checks.yml/badge.svg?branch=develop) ![Rust Checks](https://github.com/aetheric-oss/svc-storage/actions/workflows/rust_ci.yml/badge.svg?branch=develop)

## :telescope: Overview

svc-storage is responsible for storing and retrieving data from the Aetheric database and other storage.
It's meant to be used only by other internal services via gRPC interface.
- svc_storage gRPC server - (bin) target to run gRPC server
- svc_storage_client_grpc gRPC client - (lib) target for other services to import and use

Directory:
- `server/src/`: Source Code and Unit Tests of the server
- `client-grpc/src/`: Autogenerated gRPC Client Source Code and examples
- `includes/`: Shared rust modules which can be included both for the server as for the client-grpc code
- `proto/`: Types used for gRPC messaging
- `tests/`: Integration Tests
- `docs/`: Module Documentation

## Installation

Install Rust with [Rustup](https://www.rust-lang.org/tools/install).

```bash
# Adds custom pre-commit hooks to .git through cargo-husky dependency
# !! Required for developers !!
cargo test
```

## Make

### Build and Test

To ensure consistent build and test outputs, Arrow provides a Docker image with all required software installed to build and test Rust projects.
Using the Makefile, you can easily test and build your code.

```bash
# Build Locally
make rust-build

# Create Deployment Container
make build

# Run Deployment Container
make docker-run

# Stopping Deployment Container
make docker-stop

# Running examples (uses docker compose file)
make rust-example-grpc
```

### Formatting

The Arrow docker image has some formatting tools installed which can fix your code formatting for you.
Using the Makefile, you can easily run the formatters on your code.
Make sure to commit your code before running these commands, as they might not always result in a desired outcome.

```bash
# Format TOML files
make toml-tidy

# Format Rust files
make rust-tidy

# Format Python files
make python-tidy

# Format all at once
make tidy
```

### Spell check

Before being able to commit, cspell will be used as a spelling checker for all files, making sure no unintended spelling errors are found.
You can run cspell yourself by using the following make target:
```bash
make cspell-test
```

If all spelling errors are fixed, but cspell still finds words that are unknown, you can add these words to the local project words list by running the following command:
```bash
make cspell-add-words
```

### Other `make` Targets

There are additional make targets available. You can find all possible targets by running make without a target or use `make help`

## Debugging

All unit tests start with initialization of the logger, allowing for easier
debugging when tests are failing.
Since the log files are written as json, it's easy to search for log messages
using [jq](https://jqlang.github.io/jq/).
You can adjust the log level in the `log4rs.yaml` config file if needed.

Example `jq` query, to get all messages generated by the router when running
`test_vehicle_invalid_data`:

```bash
cat server/logs/backend_requests.log | jq -c '. | select(.thread=="resources::vehicle::tests::test_vehicle_invalid_data") | .message'
```

Additionally, you could grep on the function name you want to track the log
output for:
```bash
cat server/logs/backend_requests.log | jq -c '. | select(.thread=="resources::vehicle::tests::test_vehicle_invalid_data") | .message' | grep 'validate'
```

### Get all logs for a specific test
It is also possible to find all log messages generated by a specific test. For
this, you first need to find the thread_id:

```bash
cat server/logs/tests.log | jq -c '. | { message: .message, thread_id: .thread_id, date: .time}' | grep 'test_vehicle_invalid_data'
```

You can then use this thread_id (without quotes!) to find all messages in all log files generated
by this thread:

```bash
cat server/logs/*.log | jq -c '[.] | sort_by(.time) | .[] | select(.thread_id==<insert_your_thread_id>) | .message'
```

## Adding new resources
The storage module currently supports `simple` and `linked` resources types.
`simple` types are resources that reflect a table with 1 or more id fields and additional data fields.
`linked` types are resources that reflect a linked table (linking 2 resources together on a many-to-many or one-to-many relationship).

To add a new resource, you can simply pick one of the existing resources and fine all references, copying the resource specific files where needed.

### Simple resource
The following steps can be followed to create a new simple resource using an
existing  resource as a reference.

#### Example create new files and search/replace commands
```
export COPY_RESOURCE=<existing resource name you want to use as basis for your new resource>
export NEW_RESOURCE=<your new resource name>
cp proto/svc-storage-grpc-${COPY_RESOURCE}-service.proto proto/svc-storage-grpc-${NEW_RESOURCE}-service.proto
cp proto/svc-storage-grpc-${COPY_RESOURCE}.proto proto/svc-storage-grpc-${NEW_RESOURCE}.proto
cp -r includes/${COPY_RESOURCE} includes/${NEW_RESOURCE}
cp -r server/src/resources/${COPY_RESOURCE} server/src/resources/${NEW_RESOURCE}
cp client-grpc/tests/resources/${COPY_RESOURCE}.rs client-grpc/tests/resources/${NEW_RESOURCE}.rs

sed -i "s/${COPY_RESOURCE^}/${NEW_RESOURCE^}/g" proto/svc-storage-grpc-${NEW_RESOURCE}-service.proto
sed -i "s/${COPY_RESOURCE}/${NEW_RESOURCE}/g" proto/svc-storage-grpc-${NEW_RESOURCE}-service.proto

sed -i "s/${COPY_RESOURCE^}/${NEW_RESOURCE^}/g" proto/svc-storage-grpc-${NEW_RESOURCE}.proto
sed -i "s/${COPY_RESOURCE}/${NEW_RESOURCE}/g" proto/svc-storage-grpc-${NEW_RESOURCE}.proto

sed -i "s/${COPY_RESOURCE^}/${NEW_RESOURCE^}/g" includes/${NEW_RESOURCE}/mock.rs
sed -i "s/${COPY_RESOURCE}/${NEW_RESOURCE}/g" includes/${NEW_RESOURCE}/mock.rs

sed -i "s/${COPY_RESOURCE^}/${NEW_RESOURCE^}/g" server/src/resources/${NEW_RESOURCE}/mod.rs
sed -i "s/${COPY_RESOURCE}/${NEW_RESOURCE}/g" server/src/resources/${NEW_RESOURCE}/mod.rs

sed -i "s/${COPY_RESOURCE^}/${NEW_RESOURCE^}/g" client-grpc/tests/resources/${NEW_RESOURCE}.rs
sed -i "s/${COPY_RESOURCE}/${NEW_RESOURCE}/g" client-grpc/tests/resources/${NEW_RESOURCE}.rs
```

Make sure to add all newly created files to git after creation:
```
git add server/
git add proto/
git add includes/
git add client-grpc/
```

#### List of files that need to be updated

**proto**
- includes/build.rs 
  * Add the new resource name to the `get_types` function.
  * Add `derive` rules for new Enums in the `get_grpc_builder_config` function if needed. 
- proto/svc-storage-grpc-\<your new resource name\>.proto 
  * Edit the `Data` message object to reflect the correct fields.
  * Add Enums if needed

**server**
- server/src/resources/mod.rs
  * Add the new resource's module
- server/src/resources/\<your new resource name\>/mod.rs 
  * Update `Resource` `get_definition()` function to reflect the correct fields.
  * Update `GrpcDataObjectType` `get_field_value` function to reflect the correct fields.
  * Update `TryFrom<Row>` `try_from` function to reflect the correct fields.
  * Add enum `FromStr` implementations if applicable (check `vehicle` resource for example).
  * Add `test_<your new resource name>_invalid_data()` function to tests if needed (check any other resource for examples).
  * Add enum tests if applicable (check `vehicle` resource for examples).
- server/src/postgres/init.rs
  * Add your resource to the `create_db` function (make sure possible dependencies are created first).
  * Add your resource to the `drop_db` function (make sure possible dependencies
    are deleted after).
- server/src/grpc/server.rs
  * add the grpc_server macro for the new resource
  * add the new resource services to the `grpc_server` function

**client**
- client-grpc/src/lib.rs
  * Copy/paste all occurrences of the copied resource's blocks for your new resource.
- client-grpc/Cargo.toml
  * Add your resource to the `all_resources` feature.
  * Create a feature for your new resource with a `any_resource` dependency.
- client-grpc/tests/resources/\<your new resource name\>.rs
  * Update the `assert_eq` tests for the correct data fields.
- client-grpc/tests/integration_test.rs
  * Add a scenario for the new resource, testing all service functions.
- client-grpc/tests/resources/mod.rs
  * Add the new resource's module

## Cockroachdb
The database connection requires TLS.
When running cockroachdb for development, certificates will automatically be generated and used by the server.

### Cockroachdb key generation
Certificates are automatically generated by the `cockroachdb-init` container in case they're missing.
They are written to a dedicated `cockroachdb-ssl` volume so they can be shared with the services that need them.
Note that these are all snake-oil and should not be used for production.

## Example
Run the example:
```
make rust-example-grpc
```

## :scroll: Documentation
The following documents are relevant to this service:
- [Concept of Operations](./docs/conops.md)
- [Software Design Document](./docs/sdd.md)
- [Interface Control Document (ICD)](./docs/icd.md)
- [Requirements](https://nocodb.aetheric.nl/dashboard/#/nc/view/d1bb0a51-e22f-4b91-b1c5-66f11f4f861b)

## :busts_in_silhouette: About Us
Learn more about us: [Aetheric website](https://www.aetheric.nl)

## LICENSE Notice

Please note that svc-storage is under BUSL license until the Change Date, currently the earlier of two years from the release date. Exceptions to the license may be specified by Aetheric Governance via Additional Use Grants, which can, for example, allow svc-storage to be deployed for certain production uses. Please reach out to Aetheric to request a vote for exceptions to the license, or to move up the Change Date.
