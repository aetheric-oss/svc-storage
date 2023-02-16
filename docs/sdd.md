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

### Data model CockroachDB

| Value (left) | Value (right) | Meaning                       |
| ------------ | ------------- | ----------------------------- |
| \|o          | o\|           | Zero or one                   |
| \|\|         | \|\|          | Exactly one                   |
| }o           | o{            | Zero or more (no upper limit) |
| }\|          | \|{           | One or more (no upper limit)  |

```mermaid
erDiagram
    flight_plan {
        uuid flight_plan_id PK
        uuid pilot_id FK
        uuid vehicle_id FK
        integer flight_distance_meters
        text weather_conditions
        uuid departure_vertipad_id FK
        uuid destination_vertipad_id FK
        timestamp scheduled_departure
        timestamp scheduled_arrival
        timestamp actual_departure "Optional"
        timestamp actual_arrival "Optional"
        timestamp flight_release_approval "Optional"
        timestamp flight_plan_submitted "Optional"
        uuid approved_by FK "Optional"
        json cargo_weight_g "Optional"
        text flight_status "Default DRAFT"
        text flight_priority "Default LOW"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    itinerary {
        uuid itinerary_id PK
        uuid user_id
        text status "Default ACTIVE"
    }
    %% itinerary_flight_plan {
    %%     combined itinerary_id_flight_plan_id PK
    %%     uuid itinerary_id
    %%     uuid flight_plan_id
    %% }
    asset_group {
        uuid asset_group_id PK
        text name
        text description
        uuid owner FK
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    vertiport {
        uuid vertiport_id PK
        text name
        text description
        float longitude
        float latitude
        text schedule "Optional"
        uuid asset_group_id FK "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    vertipad {
        uuid vertipad_id PK
        uuid vertiport_id FK
        text name
        float longitude
        float latitude
        text schedule "Optional"
        bool enabled "Default true"
        bool occupied "Default false"
        uuid asset_group_id FK "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    user {
        uuid user_id PK
        text auth_method "Default GOOGLE_SSO"
        text auth_username "Unique"
        timestamp last_logged_in
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    contact {
        uuid contact_id PK
        text first_name
        text last_name
        text email "Optional"
        text phone_number "Optional"
        uuid address_id "Optional"
    }
    user_contact {
        uuid user_id FK
        uuid contact_id FK
    }
    pilot {
        uuid pilot_id PK
        uuid user_id FK
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    pilot_certificate {
        uuid pilot_id FK
        uuid certificate_id FK
        timestamp obtained_at
        timestamp expires_at "Default NULL"
    }
    certificate {
        uuid certificate_id PK
        text name
        text authority_name
        text certificate_code "Unique"
    }
    address {
        uuid address_id PK
        text country
        text postal_code "Unique with house_nr + house_nr_add"
        integer house_nr
        text house_nr_add "Optional"
        text street
        text city
        text state "Optional"
        decimal longitude "Optional"
        decimal latitude "Optional"
    }
    asset_supplier {
        uuid asset_supplier_id PK
        text name
        text description
        uuid main_address_id FK
        uuid main_contact_id FK
        text main_email "Optional"
        text main_phone_number "Optional"
        text website "Optional"
        text logo_path "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    asset_supplier_user {
        uuid asset_supplier_id FK
        uuid user_id FK
        string user_type "Default RO_USER"
    }
    asset_supplier_address {
        uuid asset_supplier_id FK
        uuid address_id FK
        string description "Optional"
    }
    vehicle {
        uuid vehicle_id PK
        text vehicle_model_id FK
        text serial_number
        text registration_number
        text description "Optional"
        uuid asset_group_id FK "Optional"
        text schedule "Optional"
        timestamp last_maintenance "Optional"
        timestamp next_maintenance "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    vehicle_field {
        uuid field_id
        text field
        text type
        bool mandatory "Default false"
    }
    vehicle_field_value {
        uuid vehicle_id PK
        uuid vehicle_field_id FK
        text value
        timestamp updated_at "Default NOW"
    }
    vehicle_model {
        uuid vehicle_model_id PK
        uuid manufacturer_id FK
        text model
        text type
        float max_payload_kg
        float max_range_km
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    vehicle_model_field {
        uuid field_id
        text field
        text type
        bool mandatory "Default false"
    }
    vehicle_model_field_value {
        uuid vehicle_model_id PK
        uuid vehicle_model_field_id FK
        text value
    }
    manufacturer {
        uuid manufacturer_id PK
        text name
        text logo_path
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Default NULL"
    }
    manufacturer_address {
        uuid manufacturer_id PK
        uuid address_id FK
        string description "Optional"
    }

    flight_plan |{--|| vertipad : departure_vertipad_id
    flight_plan |{--|| vertipad : destination_vertipad_id
    flight_plan |{--|| pilot : pilot_id
    flight_plan o{--|| user : approved_by

    vertipad |{--|| vertiport : vertiport_id

    pilot |{--|| user : user_id
    pilot_certificate o{--}| pilot : pilot_id
    pilot_certificate o{--}o certificate : certificate_id

    user_contact o{--}o contact : contact_id
    user_contact o{--}o user : user_id
    contact o{--|| address : address_id

    asset_supplier |{--|| contact : main_contact_id
    asset_supplier |{--|| address : main_address_id
    asset_supplier_user |{--}| asset_supplier : asset_supplier_id
    asset_supplier_user |{--}| user : user_id
    asset_supplier_address o{--}o asset_supplier : asset_supplier_id
    asset_supplier_address o{--}o address : address_id
```
