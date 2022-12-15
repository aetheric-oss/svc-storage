# `svc-storage` - Software Design Document (SDD)

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

## Overview

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Stuckee       | Lotte ([@owlot](https://github.com/owlot))                        |
| Status        | Development                                                       |

This document details the software implementation of `svc-storage`.

This process is responsible for handling interactions with clients for data storage and retrieval.

*Note: This module is intended to be used by other Arrow micro-services via gRPC.*

*This document is under development as Arrow operates on a pre-revenue and pre-commercial stage. Storage requirements may evolve as per business needs, which may result in architectural/implementation changes to the storage module.*

## Related Documents

| Document                                                                                                          | Description
| ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| [High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.                             |
| [High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Arrow microservices. |
| [Concept of Operations - `svc-storage`](./conops.md)                                                              | Defines the motivation and duties of this microservice.      |
| [Interface Control Document (ICD) - `svc-storage`](./icd.md)                                                      | Defines the inputs and outputs of this microservice.         |

## Frameworks

See the [High-Level Services ICD](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md).

## Location

Server-side service.

## Module Attributes

| Attribute       | Applies | Explanation                                                             |
| --------------- | ------- | ----------------------------------------------------------------------- |
| Safety Critical | No      | As of now, the storage service does not handle any safety critical data |
| Realtime        | No      | As of now, the storage service does not handle any realtime data        |

## Global Variables

None

## Logic 

### Initialization

At initialization this service creates a GRPC server for each resource module available.
In addition, it will create a connection to the backend database service (CockroachDB) and allocate internal memory HashMaps for each resource module for local caching purposes.

The GRPC server expects the following environment variables to be set:
- `DOCKER_PORT_GRPC` (default: `50051`)

### Control Loop

As a GRPC server, this service awaits requests and executes handlers.

All handlers **require** the following environment variables to be set:
- `PG__USER`
- `PG__DBNAME`
- `PG__HOST`
- `PG__PORT`
- `PG__SSLMODE`
- `DB_CA_CERT`
- `DB_CLIENT_CERT`
- `DB_CLIENT_KEY`

This information allows `svc-storage` to connect to the CockroachDB database backend.

:exclamation: These environment variables will *not* default to anything if not found. In this case, requests involving the handler will result in a server panic.

For detailed sequence diagrams regarding request handlers, see [GRPC Handlers](#grpc-handlers).

### Cleanup

None

## GRPC Handlers

See [the ICD](./icd.md) for this microservice.

### Storage Server

#### Database connection sequence
```mermaid
sequenceDiagram
    participant main as Main
    participant psql as psql_backend
    participant comm as common
    main->>+psql: Init connection pool
    alt Connection exists
        psql-->>-main: <Result>
    else new connection
        rect rgb(247, 161, 161)
            critical Get configuration from environment
                psql->>+psql: from_config
                psql->>+comm: configuration
                comm-->>-psql: <Config>
                psql-->>main: <result>
            option Can't read root cert
                psql-->main: Panic<Unable to read db_ca_cert file>
            option Can't create cert from pem file
                psql-->main: Panic<Unable to load Certificate from pem file>
            option Can't read client cert file
                psql-->main: Panic<Unable to read client certificate db_client_cert file>
            option Can't read client key file
                psql-->main: Panic<Unable to read client key db_client_key file>
            option Can't use client cert and key for identity creation
                psql-->main: Panic<Unable to create identity from specified cert and key>
            option Can't connect with client certs
                psql-->-main: Panic<Unable to connect build connector custom ca and client certs>
            end
        end
    end
```

#### Server startup
```mermaid
sequenceDiagram
    participant main as Main
    participant psql as psql_backend
    participant grpc as grpc_server
    participant all_psql as Each Resource psql
    participant all_grpc as Each Resource grpc
    participant comm as common
    main-->>main: Init loggers
    main-->>main: Parse command line arguments
    alt init_psql
        main->>+psql: Init connection pool
        Note over main,psql: See: Database connection sequence
        psql-->>-main: <Result>
        main->>+psql: Get pool handle
        psql-->>-main: <Pool>
        main->>+psql: Create Database
        psql->>+all_psql: (each resource) Create Table
        all_psql-->>-psql: <Result>
        psql-->>-main: <Result>
    else rebuild_psql
        main->>+psql: Init connection pool
        Note over main,psql: See: Database connection sequence
        psql-->>-main: <Result>
        main->>+psql: Get pool handle
        psql-->>-main: <Pool>
        main->>+psql: Recreate Database
        psql->>+all_psql: (each resource) Drop Table
        all_psql-->>-psql: <Result>
        psql->>+all_psql: (each resource) Create Table
        all_psql-->>-psql: <Result>
        psql-->>-main: <Result>
    else populate_psql
        main->>+psql: Init connection pool
        Note over main,psql: See: Database connection sequence
        psql-->>-main: <Result>
        main->>+psql: Get pool handle
        psql-->>-main: <Pool>
        main->>+psql: Populate Database
        psql->>+all_psql: (each resource) Insert mock data
        all_psql-->>-psql: <Result>
        psql-->>-main: <Result>
    end
    main->>+psql: Init connection pool
    Note over main,psql: See: Database connection sequence
    psql-->>-main: <Result>
    main->>+grpc: grpc_server
    grpc->>+comm: Get configuration from_config
    alt Environment vars found
        comm-->>grpc: <Config>
    else Config error
        comm-->>grpc: <ConfigError>
        deactivate comm
        grpc-->grpc: <new Config<default>>
    end
    grpc->>+all_grpc: new RpcServer
    all_grpc-->>-grpc: <Result>
    grpc-->>-main: <Result>
    loop Tokio spawn
        main-->main: grpc listen
    end
```

### Vertipad

#### `vertipads`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertipad_rpc_server
    participant psql as postgres
    participant psql_vp as vertipad/psql
    client->>+grpc: Get vertipads
    rect rgb(247, 161, 161)
        critical Get DB connection from the pool
            grpc->>+psql: Get pool handle
            psql-->>grpc: <Pool>
        option No connection
            psql-->>-grpc: Database pool not initialized
        end
    end
    grpc->>+psql_vp: Query vertipads with <SearchFilter>
    psql_vp-->>-grpc: <Vec<Row>>
    grpc-->>grpc: <Vertipads> from <Vec<Row>>
    grpc-->>-client: <Vertipads>
```

#### `vertipad_by_id`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertipad_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertipad/psql
    client->>+grpc: Get vertipad by ID
    grpc->>+memdb: Get vertipad from cache?
    alt is found in cache
        memdb-->>-grpc: <Vertipad>
    else not found in cache
        rect rgb(247, 161, 161)
            critical Get DB connection from the pool
                grpc->>+psql: Get pool handle
                psql-->>grpc: <Pool>
            option No connection
                psql-->>-grpc: Database pool not initialized
            end
        end
        grpc->>+psql_vp: Query vertipad for ID
        psql_vp-->>-grpc: <VertipadPsql>
        grpc-->>grpc: <Vertipad> from <VertipadPsql>
    end
    grpc-->>-client: <Vertipad>
```

#### `insert_vertipad`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertipad_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertipad/psql
    client->>+grpc: Get vertipad by ID
    rect rgb(247, 161, 161)
        critical Get DB connection from the pool
            grpc->>+psql: Get pool handle
            psql-->>grpc: <Pool>
        option No connection
            psql-->>-grpc: Database pool not initialized
        end
    end
    grpc->>+psql_vp: Create vertipad with data
    psql_vp-->>-grpc: <VertipadPsql>
    grpc-->>grpc: <Vertipad> from <VertipadPsql>
    grpc->>memdb: insert Vertipad
    grpc-->>-client: <Vertipad>
```

#### `update_vertipad`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertipad_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertipad/psql
    client->>+grpc: Update vertipad with data
    grpc-->>grpc: <UpdateVertipad> from <Request>
    alt No data provided for update
        grpc-->>client: <Status<Cancelled>>
    else vertipad data found in request
        grpc-->>grpc: <VertipadData>
        rect rgb(247, 161, 161)
            critical Get DB connection from the pool
                grpc->>+psql: Get pool handle
                psql-->>grpc: <Pool>
            option No connection
                psql-->>-grpc: Database pool not initialized
            end
        end
        grpc->>+psql_vp: Query vertipad for ID
        alt No vertipad found
            psql_vp-->>grpc: <ArrErr>
            grpc-->>client: <Status<not found>>
        else vertipad exists
            psql_vp-->>-grpc: <VertipadPsql>
            grpc->>+psql_vp: Update vertipad with data
            alt Update failed
                psql_vp-->>grpc: <ArrErr>
                grpc-->>client: <Status<Internal>>
            else Update success
                psql_vp-->>psql_vp: read
                psql_vp-->>-grpc: <VertipadPsql>
                grpc-->>grpc: <Vertipad> from <VertipadPsql>
                grpc->>memdb: insert Vertipad
                grpc-->>-client: <Vertipad>
            end
        end
    end
```

#### `delete_vertipad`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertipad_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertipad/psql
    client->>+grpc: Delete vertipad with ID
    rect rgb(247, 161, 161)
        critical Get DB connection from the pool
            grpc->>+psql: Get pool handle
            psql-->>grpc: <Pool>
        option No connection
            psql-->>-grpc: Database pool not initialized
        end
    end
    grpc->>+psql_vp: Query vertipad for ID
    alt No vertipad found
        psql_vp-->>grpc: <ArrErr>
        grpc-->>client: <Status<not found>>
    else vertipad exists
        psql_vp-->>-grpc: <VertipadPsql>
        grpc->>+psql_vp: Delete vertipad
        alt Delete failed
            psql_vp-->>grpc: <ArrErr>
            grpc-->>client: <Status<Internal>>
        else Delete success
            psql_vp-->>-grpc: 
            grpc->>memdb: delete with Id
            grpc-->>-client: <Response>
        end
    end
```

### Vertiport

#### `vertiports`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertiport_rpc_server
    participant psql as postgres
    participant psql_vp as vertiport/psql
    client->>+grpc: Get vertiports
    rect rgb(247, 161, 161)
        critical Get DB connection from the pool
            grpc->>+psql: Get pool handle
            psql-->>grpc: <Pool>
        option No connection
            psql-->>-grpc: Database pool not initialized
        end
    end
    grpc->>+psql_vp: Query vertiports with <SearchFilter>
    psql_vp-->>-grpc: <Vec<Row>>
    grpc-->>grpc: <Vertiports> from <Vec<Row>>
    grpc-->>-client: <Vertiports>
```

#### `vertiport_by_id`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertiport_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertiport/psql
    client->>+grpc: Get vertiport by ID
    grpc->>+memdb: Get vertiport from cache?
    alt is found in cache
        memdb-->>-grpc: <Vertiport>
    else not found in cache
        rect rgb(247, 161, 161)
            critical Get DB connection from the pool
                grpc->>+psql: Get pool handle
                psql-->>grpc: <Pool>
            option No connection
                psql-->>-grpc: Database pool not initialized
            end
        end
        grpc->>+psql_vp: Query vertiport for ID
        psql_vp-->>-grpc: <VertiportPsql>
        grpc-->>grpc: <Vertiport> from <VertiportPsql>
    end
    grpc-->>-client: <Vertiport>
```

#### `insert_vertiport`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertiport_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertiport/psql
    client->>+grpc: Get vertiport by ID
    rect rgb(247, 161, 161)
        critical Get DB connection from the pool
            grpc->>+psql: Get pool handle
            psql-->>grpc: <Pool>
        option No connection
            psql-->>-grpc: Database pool not initialized
        end
    end
    grpc->>+psql_vp: Create vertiport with data
    psql_vp-->>-grpc: <VertiportPsql>
    grpc-->>grpc: <Vertiport> from <VertiportPsql>
    grpc->>memdb: insert Vertiport
    grpc-->>-client: <Vertiport>
```

#### `update_vertiport`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertiport_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertiport/psql
    client->>+grpc: Update vertiport with data
    grpc-->>grpc: <UpdateVertiport> from <Request>
    alt No data provided for update
        grpc-->>client: <Status<Cancelled>>
    else vertiport data found in request
        grpc-->>grpc: <VertiportData>
        rect rgb(247, 161, 161)
            critical Get DB connection from the pool
                grpc->>+psql: Get pool handle
                psql-->>grpc: <Pool>
            option No connection
                psql-->>-grpc: Database pool not initialized
            end
        end
        grpc->>+psql_vp: Query vertiport for ID
        alt No vertiport found
            psql_vp-->>grpc: <ArrErr>
            grpc-->>client: <Status<not found>>
        else vertiport exists
            psql_vp-->>-grpc: <VertiportPsql>
            grpc->>+psql_vp: Update vertiport with data
            alt Update failed
                psql_vp-->>grpc: <ArrErr>
                grpc-->>client: <Status<Internal>>
            else Update success
                psql_vp-->>psql_vp: read
                psql_vp-->>-grpc: <VertiportPsql>
                grpc-->>grpc: <Vertiport> from <VertiportPsql>
                grpc->>memdb: insert Vertiport
                grpc-->>-client: <Vertiport>
            end
        end
    end
```

#### `delete_vertiport`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc as vertiport_rpc_server
    participant memdb as memdb
    participant psql as postgres
    participant psql_vp as vertiport/psql
    client->>+grpc: Delete vertiport with ID
    rect rgb(247, 161, 161)
        critical Get DB connection from the pool
            grpc->>+psql: Get pool handle
            psql-->>grpc: <Pool>
        option No connection
            psql-->>-grpc: Database pool not initialized
        end
    end
    grpc->>+psql_vp: Query vertiport for ID
    alt No vertiport found
        psql_vp-->>grpc: <ArrErr>
        grpc-->>client: <Status<not found>>
    else vertiport exists
        psql_vp-->>-grpc: <VertiportPsql>
        grpc->>+psql_vp: Delete vertiport
        alt Delete failed
            psql_vp-->>grpc: <ArrErr>
            grpc-->>client: <Status<Internal>>
        else Delete success
            psql_vp-->>-grpc: 
            grpc->>memdb: delete with Id
            grpc-->>-client: <Response>
        end
    end
```
