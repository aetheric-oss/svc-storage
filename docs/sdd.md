![Aetheric Banner](https://github.com/aetheric-oss/.github/raw/main/assets/doc-banner.png)

# Software Design Document (SDD) - `svc-storage`

## :telescope: Overview

This document details the software implementation of `svc-storage`.

This process is responsible for handling interactions with clients for data storage and retrieval.

*Note: This module is intended to be used by other Aetheric micro-services via gRPC.*

*This document is under development as Aetheric operates on a pre-revenue and
pre-commercial stage. Storage requirements may evolve as per business needs,
which may result in architectural/implementation changes to the storage module.*

### Metadata

| Attribute     | Description                                                                    |
| ------------- |--------------------------------------------------------------------------------|
| Maintainer(s) | [@aetheric-oss/dev-realm](https://github.com/orgs/aetheric-oss/teams/dev-realm)|
| Stuckee       | Lotte ([@owlot](https://github.com/owlot))                                     |
| Status        | Development                                                                    |

## :books: Related Documents

| Document                                                                                                             | Description
| -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| [High-Level Concept of Operations (CONOPS)](https://github.com/aetheric-oss/se-services/blob/develop/docs/conops.md) | Overview of Realm microservices.                             |
| [High-Level Interface Control Document (ICD)](https://github.com/aetheric-oss/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Realm microservices. |
| [Requirements - `svc-storage`](https://nocodb.aetheric.nl/dashboard/#/nc/p_uyeuw6scqlnpri/table/L4/svc-storage)      | Requirements and user stories for this microservice.         |
| [Concept of Operations - `svc-storage`](./conops.md)                                                                 | Defines the motivation and duties of this microservice.      |
| [Interface Control Document (ICD) - `svc-storage`](./icd.md)                                                         | Defines the inputs and outputs of this microservice.         |

## :dna: Module Attributes

| Attribute       | Applies | Explanation                                                             |
| --------------- | ------- | ----------------------------------------------------------------------- |
| Safety Critical | No      | As of now, the storage service does not handle any safety critical data |
| Realtime        | No      | As of now, the storage service does not handle any realtime data        |

## :globe_with_meridians: Global Variables

None

## :gear: Logic

### Initialization

At initialization this service creates a GRPC server for each available resource module.
In addition, it will create a connection to the backend database service (CockroachDB).

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

For detailed sequence diagrams regarding request handlers, see [gRPC Handlers](#speech_balloon-grpc-handlers).

### Cleanup

None

## :speech_balloon: gRPC Handlers

See [the ICD](./icd.md) for this microservice.

### Storage Server

#### Database connection sequence
```mermaid
sequenceDiagram
    participant main as Main
    participant psql as psql_backend
    participant comm as common
    main->>+psql: init_psql_pool()
    alt Connection exists
        psql-->>-main: <Result>
    else new connection
        rect rgb(64,97,255)
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
    participant psql as psql_mod
    participant psql_init as psql::init
    participant grpc as grpc_server
    participant all_psql as Resources as PsqlInitResource
    participant all_grpc as Resources as GrpcServer
    participant comm as common
    main-->>main: Init loggers
    main-->>main: Parse command line arguments
    alt init_psql
        main->>+psql: init_psql_pool()
        Note over main,psql: See: Database connection sequence
        psql-->>-main: <Result>
        main->>+psql: get_psql_pool()
        psql-->>-main: <Pool>
        main->>+psql_init: create_db()
        psql_init->>+all_psql: (each resource) init_table()
        all_psql-->>-psql_init: <Result>
        psql_init-->>-main: <Result>
    else rebuild_psql
        main->>+psql: init_psql_pool()
        Note over main,psql: See: Database connection sequence
        psql-->>-main: <Result>
        main->>+psql: get_psql_pool()
        psql-->>-main: <Pool>
        main->>+psql_init: recreate_db()
        psql_init-->psql_init: drop_db()
        psql_init->>+all_psql: (each resource) drop_table()
        all_psql-->>-psql_init: <Result>
        psql_init->>+all_psql: (each resource) init_table()
        all_psql-->>-psql_init: <Result>
        psql_init-->>-main: <Result>
    end
    main->>+psql: init_psql_pool()
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

### Simple Resource

#### `get_by_id`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcSimpleService
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql as postgres
    client->>+grpc_server: get_by_id(Request<Id>)
    grpc_server->>+grpc_service: generic_get_by_id(Request<Id>)
    grpc_service->>+psql_simple: get_by_id(Uuid)
    rect rgb(64,97,255)
        critical Get DB connection from the pool
            psql_simple->>+psql: get_psql_pool()
            psql-->>psql_simple: <Pool>
        option No connection
            psql-->>-psql_simple: Database pool not initialized
        end
    end
    psql_simple-->>-grpc_service: Result<Row, Error>
    alt Err
        grpc_service-->>grpc_server: Status(Code::NotFound)
    else Ok
        grpc_service-->>grpc_service: <Object> from <Row>
        grpc_service-->>-grpc_server: Ok(tonic::Response<Object>)
    end
    grpc_server-->>-client: Result
```

#### `search`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcSimpleService
    participant psql_simple as postgres::PsqlSearch
    participant psql as postgres
    client->>+grpc_server: search(Request<AdvancedSearchFilter>)
    grpc_server->>+grpc_service: generic_search(Request<AdvancedSearchFilter>)
    grpc_service->>+psql_simple: advanced_search(AdvancedSearchFilter)
    rect rgb(64,97,255)
        critical Get DB connection from the pool
            psql_simple->>+psql: get_psql_pool()
            psql-->>psql_simple: <Pool>
        option No connection
            psql-->>-psql_simple: Database pool not initialized
        end
    end
    psql_simple-->>-grpc_service: Result<Rows, Error>
    alt Err (database error)
        grpc_service-->>grpc_server: Status(Code::Internal)
    else Ok (search success)
        grpc_service-->>grpc_service: <List> from <Rows>
        grpc_service-->>-grpc_server: Ok(tonic::Response<List>)
    end
    grpc_server-->>-client: Result
```

#### `insert`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcSimpleService
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql as postgres
    client->>+grpc_server: insert(Request<Data>)
    grpc_server->>+grpc_service: generic_insert(Request<Data>)
    grpc_service->>+psql_simple: create(GrpcDataObjectType)
    psql_simple-->>psql_simple: validate(GrpcDataObjectType)
    alt Validation errors found
        psql_simple-->>grpc_service: Ok((None, ValidationResult))
    else Validation success
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_simple-->>+psql: get_psql_pool()
                psql-->>psql_simple: <Pool>
            option No connection
                psql-->>-psql_simple: Database pool not initialized
            end
        end
        psql_simple-->>-grpc_service: Ok((Some(Uuid), ValidationResult))
    end
    alt Err (insert error)
        grpc_service-->>grpc_server: Status(Code::Internal)
    else Ok (insert success)
        grpc_service-->>grpc_service: Object from Uuid
        grpc_service-->>-grpc_server: Ok(tonic::Response<Object>)
    end
    grpc_server-->>-client: Result
```

#### `update`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcSimpleService
    participant psql_simple_object as postgres::simple_resource::PsqlObjectType
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql as postgres
    client->>+grpc_server: update(Request<UpdateObject>)
    grpc_server->>+grpc_service: generic_update(Request<UpdateObject>)
    grpc_service-->>grpc_service: UpdateObject into Object
    grpc_service->>+psql_simple_object: update(GrpcDataObjectType)
    psql_simple_object-->>+psql_simple: validate(GrpcDataObjectType)
    psql_simple-->>-psql_simple_object: Result<(Some, ValidationResult), Error>
    alt Validation errors found
        psql_simple_object-->>grpc_service: Ok((None, ValidationResult))
    else Validation success
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_simple_object->>+psql: get_psql_pool()
                psql-->>psql_simple_object: <Pool>
            option No connection
                psql-->>-psql_simple_object: Database pool not initialized
            end
        end
        psql_simple_object-->>psql_simple_object: run db update query
        psql_simple_object-->>-grpc_service: Ok((Some(Row), ValidationResult))
    end
    alt Err (update error)
        grpc_service-->>grpc_server: Status(Code::Internal)
    else Ok (might have validation errors, caller should check result)
        alt Some Row (resource successfully updated)
            grpc_service-->>grpc_service: new GenericResourceResult with resource Some(Object)
        else None (field validations did not pass)
            grpc_service-->>grpc_service: new GenericResourceResult with resource None
        end
        grpc_service-->>-grpc_server: Ok(tonic::Response<Response>)
    end
    grpc_server-->>-client: Result
```

#### `delete`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcSimpleService
    participant psql_simple_object as postgres::simple_resource::PsqlObjectType
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql as postgres
    client->>+grpc_server: delete(Request<Id>)
    grpc_server->>+grpc_service: generic_delete(Request<Id>)
    grpc_service-->>grpc_service: UpdateObject into Object
    grpc_service->>+psql_simple_object: delete()
    psql_simple_object-->>psql_simple_object: get_definition()
    alt definition.fields.contains_key("deleted_at") 
        psql_simple_object-->>psql_simple_object: set_deleted_at_now()
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_simple_object->>+psql: get_psql_pool()
                psql-->>psql_simple_object: <Pool>
            option No connection
                psql-->>-psql_simple_object: Database pool not initialized
            end
        end
        psql_simple_object-->>psql_simple_object: run db update query
        psql_simple_object-->>grpc_service: Result
    else
        psql_simple_object-->>psql_simple_object: delete_row()
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_simple_object->>+psql: get_psql_pool()
                psql-->>psql_simple_object: <Pool>
            option No connection
                psql-->>-psql_simple_object: Database pool not initialized
            end
        end
        psql_simple_object-->>psql_simple_object: run db delete query
        psql_simple_object-->>-grpc_service: Result
    end
    alt Err (database error)
        grpc_service-->>grpc_server: Status(Code::Internal)
    else Ok (delete success)
        grpc_service-->>-grpc_server: Ok(tonic::Response<()>)
    end
    grpc_server-->>-client: Result
```

### linked Resource

#### `link`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcLinkService
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql_linked as postgres::linked_resource::PsqlType
    participant psql as postgres
    client->>+grpc_server: link(Request<LinkObject>)
    grpc_server->>+grpc_service: generic_link(id, other_ids, false)
    alt Err "Could not convert provided id String [{id}] into uuid: {e}"
        grpc_service-->>grpc_server: Status(Code::NotFound)
    else
        grpc_service->>+psql_simple: get_by_id(Uuid)
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_simple->>+psql: get_psql_pool()
                psql-->>psql_simple: <Pool>
            option No connection
                psql-->>-psql_simple: Database pool not initialized
            end
        end
        psql_simple-->>-grpc_service: Result<Row, Error>
        alt Err "No [{psql_table}] found for specified uuid: {id}"
            grpc_service-->>grpc_server: Status(Code::NotFound)
        else Ok
            grpc_service->>+psql_linked: link_ids(ids, replace_id_fields)
            rect rgb(64,97,255)
                critical Get DB connection from the pool
                    psql_linked->>+psql: get_psql_pool()
                    psql-->>psql_linked: <Pool>
                option No connection
                    psql-->>-psql_linked: Database pool not initialized
                end
            end
            psql_linked-->>-grpc_service: Result<(), Error>
            alt Err (database error)
                grpc_service-->>grpc_server: Status(Code::Internal)
            else Ok (link success)
                grpc_service-->>-grpc_server: Ok(tonic::Response<()>)
            end
        end
    end
    grpc_server-->>-client: Result
```

#### `replace_linked`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcLinkService
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql_linked as postgres::linked_resource::PsqlType
    participant psql as postgres
    client->>+grpc_server: replace_linked(Request<LinkObject>)
    grpc_server->>+grpc_service: generic_link(id, other_ids, true)
    alt Err "Could not convert provided id String [{id}] into uuid: {e}"
        grpc_service-->>grpc_server: Status(Code::NotFound)
    else
        grpc_service->>+psql_simple: get_by_id(Uuid)
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_simple->>+psql: get_psql_pool()
                psql-->>psql_simple: <Pool>
            option No connection
                psql-->>-psql_simple: Database pool not initialized
            end
        end
        psql_simple-->>-grpc_service: Result<Row, Error>
        alt Err "No [{psql_table}] found for specified uuid: {id}"
            grpc_service-->>grpc_server: Status(Code::NotFound)
        else Ok
            grpc_service->>+psql_linked: link_ids(ids, replace_id_fields)
            psql_linked-->>psql_linked: delete_for_ids(replace_id_fields)
            rect rgb(64,97,255)
                critical Get DB connection from the pool
                    psql_linked->>+psql: get_psql_pool()
                    psql-->>psql_linked: <Pool>
                option No connection
                    psql-->>-psql_linked: Database pool not initialized
                end
            end
            psql_linked-->>-grpc_service: Result<(), Error>
            alt Err (database error)
                grpc_service-->>grpc_server: Status(Code::Internal)
            else Ok (link success)
                grpc_service-->>-grpc_server: Ok(tonic::Response<()>)
            end
        end
    end
    grpc_server-->>-client: Result
```

#### `unlink`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcLinkService
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql_linked as postgres::linked_resource::PsqlType
    participant psql as postgres
    client->>+grpc_server: unlink(Request<Id>)
    grpc_server->>+grpc_service: generic_unlink(Request<Id>)
    alt Err (provided Id could not be converted to Uuid
        grpc_service-->>grpc_server: Status(Code::NotFound)
    else
        grpc_service->>+psql_simple: get_by_id(Uuid)
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_simple->>+psql: get_psql_pool()
                psql-->>psql_simple: <Pool>
            option No connection
                psql-->>-psql_simple: Database pool not initialized
            end
        end
        psql_simple-->>-grpc_service: Result<Row, Error>
        alt Err "No [{psql_table}] found for specified uuid: {id}"
            grpc_service-->>grpc_server: Status(Code::NotFound)
        else Ok
            grpc_service->>+psql_linked: delete_for_ids(ids)
            rect rgb(64,97,255)
                critical Get DB connection from the pool
                    psql_linked->>+psql: get_psql_pool()
                    psql-->>psql_linked: <Pool>
                option No connection
                    psql-->>-psql_linked: Database pool not initialized
                end
            end
            psql_linked-->>-grpc_service: Result<(), Error>
            alt Err (database error)
                grpc_service-->>grpc_server: Status(Code::Internal)
            else Ok (unlink success)
                grpc_service-->>-grpc_server: Ok(tonic::Response<()>)
            end
        end
    end
    grpc_server-->>-client: Result
```

#### `get_linked_ids`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcLinkService
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql_linked as postgres::linked_resource::PsqlType
    participant psql_search as postgres::PsqlSearch
    participant psql as postgres
    client->>+grpc_server: get_linked_ids(Request<Id>)
    grpc_server->>+grpc_service: generic_get_linked_ids(Request<Id>)
    grpc_service->>+grpc_service: _get_linked(Id)
    grpc_service->>+psql_simple: get_by_id(Uuid)
    rect rgb(64,97,255)
        critical Get DB connection from the pool
            psql_simple->>+psql: get_psql_pool()
            psql-->>psql_simple: <Pool>
        option No connection
            psql-->>-psql_simple: Database pool not initialized
        end
    end
    psql_simple-->>-grpc_service: Result<Row, Error>
    alt Err "No resource found for specified uuid: {id}"
        grpc_service-->>grpc_service: Status(Code::NotFound)
    else Ok
        grpc_service->>+psql_linked: get_for_ids(ids)
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_linked->>+psql: get_psql_pool()
                psql-->>psql_linked: <Pool>
            option No connection
                psql-->>-psql_linked: Database pool not initialized
            end
        end
        psql_linked-->>-grpc_service: Result<Vec<Row>, Error>
        alt Err (database error)
            grpc_service-->>grpc_service: Err(ArrErr::Error)
        else Ok (query success)
            grpc_service-->>-grpc_service: Ok(ids)
            grpc_service->>+psql_search: advanced_search(AdvancedSearchFilter)
            rect rgb(64,97,255)
                critical Get DB connection from the pool
                    psql_search->>+psql: get_psql_pool()
                    psql-->>psql_search: <Pool>
                option No connection
                    psql-->>-psql_search: Database pool not initialized
                end
            end
            psql_search-->>-grpc_service: Result<Rows, Error>
            alt Err (database error)
                grpc_service-->>grpc_server: Status(Code::Internal)
            else Ok (search success)
                grpc_service-->>grpc_service: <IdList> from <Rows>
                grpc_service-->>-grpc_server: Ok(tonic::Response<IdList>)
            end
        end
    end
    grpc_server-->>-client: Result
```

#### `get_linked`
```mermaid
sequenceDiagram
    participant client as grpc_client
    participant grpc_server as GrpcServer
    participant grpc_service as grpc::GrpcLinkService
    participant psql_simple as postgres::simple_resource::PsqlType
    participant psql_linked as postgres::linked_resource::PsqlType
    participant psql_search as postgres::PsqlSearch
    participant psql as postgres
    client->>+grpc_server: get_linked(Request<Id>)
    grpc_server->>+grpc_service: generic_get_linked(Request<Id>)
    grpc_service->>+grpc_service: _get_linked(Id)
    grpc_service->>+psql_simple: get_by_id(Uuid)
    rect rgb(64,97,255)
        critical Get DB connection from the pool
            psql_simple->>+psql: get_psql_pool()
            psql-->>psql_simple: <Pool>
        option No connection
            psql-->>-psql_simple: Database pool not initialized
        end
    end
    psql_simple-->>-grpc_service: Result<Row, Error>
    alt Err "No resource found for specified uuid: {id}"
        grpc_service-->>grpc_service: Status(Code::NotFound)
    else Ok
        grpc_service->>+psql_linked: get_for_ids(ids)
        rect rgb(64,97,255)
            critical Get DB connection from the pool
                psql_linked->>+psql: get_psql_pool()
                psql-->>psql_linked: <Pool>
            option No connection
                psql-->>-psql_linked: Database pool not initialized
            end
        end
        psql_linked-->>-grpc_service: Result<Vec<Row>, Error>
        alt Err (database error)
            grpc_service-->>grpc_service: Err(ArrErr::Error)
        else Ok (query success)
            grpc_service-->>-grpc_service: Ok(ids)
            grpc_service->>+psql_search: advanced_search(AdvancedSearchFilter)
            rect rgb(64,97,255)
                critical Get DB connection from the pool
                    psql_search->>+psql: get_psql_pool()
                    psql-->>psql_search: <Pool>
                option No connection
                    psql-->>-psql_search: Database pool not initialized
                end
            end
            psql_search-->>-grpc_service: Result<Rows, Error>
            alt Err (database error)
                grpc_service-->>grpc_server: Status(Code::Internal)
            else Ok (search success)
                grpc_service-->>grpc_service: <List> from <Rows>
                grpc_service-->>-grpc_server: Ok(tonic::Response<List>)
            end
        end
    end
    grpc_server-->>-client: Result
```

## Data model CockroachDB

| Value (left) | Value (right) | Meaning                       |
| ------------ | ------------- | ----------------------------- |
| \|o          | o\|           | Zero or one                   |
| \|\|         | \|\|          | Exactly one                   |
| }o           | o{            | Zero or more (no upper limit) |
| }\|          | \|{           | One or more (no upper limit)  |


```mermaid
erDiagram
    user {
        uuid user_id PK
        text auth_method "ENUM (OAUTH_GOOGLE,OAUTH_FACEBOOK,OAUTH_AZURE_AD,LOCAL)"
        text display_name
        text email
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
```

### Itinerary and Flight Plan schema
```mermaid
erDiagram
    pilot {
        uuid pilot_id PK
        uuid user_id FK
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }

    vehicle {
        uuid vehicle_id PK
        uuid vehicle_model_id
        text serial_number
        text registration_number
        text description "Optional"
        text schedule "Optional"
        timestamp last_maintenance "Optional"
        timestamp next_maintenance "Optional"
        uuid hangar_id FK "Optional"
        uuid hangar_bay_id FK "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    vertipad {
        uuid vertipad_id PK
        uuid vertiport_id FK
        text name
        geometry geo_location "POINT"
        text schedule
        bool enabled "Default true"
        bool occupied "Default false"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    vertiport {
        uuid vertiport_id PK
        text name
        text description
        geometry geo_location "POLYGON"
        text schedule
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    vertiport ||--o{ vertipad : vertiport_id
    vertiport ||--o{ vehicle : hangar_id
    vertipad ||--o{ vehicle : hangar_bay_id

    flight_plan {
        uuid flight_plan_id PK
        uuid pilot_id FK
        uuid vehicle_id FK
        geometry path "LINESTRING"
        text weather_conditions "Optional"
        uuid origin_vertipad_id FK
        uuid target_vertipad_id FK
        timestamp origin_timeslot_start
        timestamp origin_timeslot_end
        timestamp target_timeslot_start
        timestamp target_timeslot_end
        timestamp actual_departure_time "Optional"
        timestamp actual_arrival_time "Optional"
        timestamp flight_release_approval "Optional"
        timestamp flight_plan_submitted "Optional"
        uuid approved_by FK "Optional"
        text flight_status "Default DRAFT"
        text flight_priority "Default LOW"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    flight_plan }o--|| vertipad : origin_vertipad_id
    flight_plan }o--|| vertipad : target_vertipad_id
    flight_plan }o--|| pilot : pilot_id
    flight_plan }o--|| vehicle : vehicle_id
    user ||--o{ pilot : user_id

    itinerary {
        uuid itinerary_id PK
        uuid user_id
        text status "Default ACTIVE"
    }
    itinerary_flight_plan {
        combined itinerary_id_flight_plan_id PK
        uuid itinerary_id FK
        uuid flight_plan_id FK
    }
    itinerary_flight_plan |o--|{ flight_plan : flight_plan_id
    itinerary_flight_plan |o--|| itinerary : itinerary_id
```

### Parcel schema

```mermaid
erDiagram
    scanner {
        uuid scanner_id PK
        uuid organization_id
        text scanner_type "ENUM(MOBILE,LOCKER,FACILITY,UNDERBELLY)"
        text scanner_status "ENUM(ACTIVE,DISABLED)"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }

    parcel {
        uuid parcel_id PK
        uuid user_id FK
        uint weight_grams
        text status "ENUM(NOTDROPPEDOFF,DROPPEDOFF,ENROUTE,ARRIVED,PICKEDUP,COMPLETE)"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    user }o--o{ parcel : user_id

    parcel_scan {
        uuid parcel_id FK
        uuid scanner_id FK
        geometry geo_location "POINT"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    parcel ||--o{ parcel_scan : parcel_id
    scanner ||--o{ parcel_scan : scanner_id

    flight_plan {
        uuid flight_plan_id FK
        _ _ "flight_plan fields"
    }
    flight_plan_parcel {
        combined flight_plan_id_parcel_id PK
        uuid flight_plan_id FK
        uuid parcel_id FK
        bool acquire
        bool deliver
    }
    parcel }|--o| flight_plan_parcel : parcel_id
    flight_plan ||--o| flight_plan_parcel : flight_plan_id

```

### Group schema for users and assets

Groups can have multiple functions:
 - providing a way to organize
 - assign ACLs to a group of users
 - provide default settings 

To distinguish the group's purpose, each group has a group `type`:
 - `acl` - The group can be linked to the `group_acl` table.
 - `settings` - The group can be linked to the `group_field_value` table.
 - `display` - The group is only used to allow for an organized view in the frontend.

```mermaid
erDiagram
    group {
        uuid group_id PK
        text name
        text description
        text type "ENUM (acl, settings, display)"
        uuid parent_group_id FK "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }

    user_group {
        uuid user_id FK
        uuid group_id FK
    }
    group ||--o{ user_group : multiple
    user ||--o{ user_group : multiple

    supplier_group {
        uuid supplier_id FK
        uuid group_id FK
    }
    supplier ||--o{ supplier_group : multiple
    group ||--o{ supplier_group : multiple

    vehicle_group {
        uuid vehicle_id FK
        uuid group_id FK
    }
    group ||--o{ vehicle_group : multiple
    vehicle ||--o{ vehicle_group : multiple

    vertipad_group {
        uuid vertipad_id FK
        uuid group_id FK
    }
    group ||--o{ vertipad_group : multiple
    vertipad ||--o{ vertipad_group : multiple

    vertiport_group {
        uuid vertiport_id FK
        uuid group_id FK
    }
    group ||--o{ vertiport_group : multiple
    vertiport ||--o{ vertiport_group : multiple
```

#### Example data

**groups**

| group_id | name           | description                                    | type     | parent_group_id |
|----------|----------------|------------------------------------------------|----------|-----------------|
| group_1  | Countries      | Parent group for country specific settings     | settings | NULL            |
| group_2  | Assets NL      | Default settings for assets in The Netherlands | settings | group_1         |
| group_3  | Assets US      | Default settings for assets in the US          | settings | group_1         |
| group_4  | Assets US TX   | Default settings for assets in US Texas        | settings | group_3         |
| group_5  | Assets US WA   | Default settings for assets in US Texas        | settings | group_3         |
| group_6  | Assets US CA   | Default settings for assets in US Texas        | settings | group_3         |
| group_8  | Supplier 1     | Parent group for Supplier 1                    | display  | NULL            |
| group_9  | Favorites      | My favorite assets                             | display  | group_8         |
| group_10 | Administrator  | Group for users with administrator privileges  | acl      | group_8         |
| group_11 | Asset manager  | Group for users with asset manager privileges  | acl      | group_8         |
| group_12 | Super Admin    | Group for users with super admin privileges    | acl      | NULL            |
| group_13 | Supplier Admin | Group for users with supplier admin privileges | acl      | NULL            |

**users**
| user_id | display_name | email |
|---------|--------------| ----- |
| user_1  | Thomasg      | thomasg@arrowair.com |
| user_2  | A.M. Smith   | amsmith@aetheric.nl |
| user_3  | MissQueen    | missqueen@aetheric.nl |
| user_4  | Owlot        | owlot@aetheric.nl | 

**vehicles**
| vehicle_id | vehicle_model | description                |
|------------|---------------|----------------------------|
| vehicle_1  | FEATHER1      | First Project Feather VTOL |

**asset suppliers**
| supplier_id | name  |
|-------------------|-------|
| supplier_1  | Arrow |
| supplier_2  | Aetheric |

**acl**
| acl_id  | code            | description                |
|---------|-----------------|----------------------------|
| acl_0   | SUPERUSER       | All permissions            |
| acl_1   | ASSET_CREATE    | Create new assets          |
| acl_2   | ASSET_EDIT      | Edit existing assets       |
| acl_3   | ASSET_DELETE    | Delete assets              |
| acl_4   | ASSET_VIEW      | View assets                |
| acl_5   | USER_CREATE     | Create new users           |
| acl_6   | USER_EDIT       | Edit existing users        |
| acl_7   | USER_DELETE     | Delete users               |
| acl_8   | USER_VIEW       | View users                 |
| acl_9   | SUPPLIER_CREATE | Create new suppliers       |
| acl_10  | SUPPLIER_EDIT   | Edit existing supplier     |
| acl_11  | SUPPLIER_DELETE | Delete suppliers           |
| acl_12  | SUPPLIER_VIEW   | View suppliers             |

We can now have the groups linked as followed:

```mermaid
flowchart TB
    subgraph groups
        subgraph group_1[group_1 Countries]
            group_2[group_2 Assets NL]
            subgraph group_3[group_3 Assets US]
                group_4[group_4 Assets US TX]
                group_5[group_5 Assets US WA]
                group_6[group_6 Assets US CA]
            end
        end
        subgraph group_8[group_8 Supplier 1]
            group_9[group_9 Favorites]
            group_10[group_10 Administrator]
            group_11[group_11 Asset manager]
        end
        group_12[group_12 Super Admin]
        group_13[group_13 Supplier Admin]
    end
    subgraph acl
        acl_0[acl_0 SUPERUSER]
        acl_1[acl_1 ASSET_CREATE]
        acl_2[acl_2 ASSET_EDIT]
        acl_3[acl_3 ASSET_DELETE]
        acl_4[acl_4 ASSET_VIEW]
        acl_5[acl_5 USER_CREATE]
        acl_6[acl_6 USER_EDIT]
        acl_7[acl_7 USER_DELETE]
        acl_8[acl_8 USER_VIEW]
        acl_9[acl_9 SUPPLIER_CREATE]
        acl_10[acl_10 SUPPLIER_EDIT]
        acl_11[acl_11 SUPPLIER_DELETE]
        acl_12[acl_12 SUPPLIER_VIEW]
    end
    subgraph user
        user_1[user_1 Thomasg]
        user_2[user_2 A.M. Smith]
        user_3[user_3 MissQueen]
        user_4[user_4 Owlot]
    end
    subgraph vehicle
        vehicle_1[FEATHER1]
    end
    subgraph supplier[supplier]
        supplier_1[supplier_1 Arrow]
    end
    group_12-->acl_0
    group_10-->acl_1
    group_10-->acl_2
    group_10-->acl_3
    group_10-->acl_4
    group_10-->acl_5
    group_10-->acl_6
    group_10-->acl_7
    group_10-->acl_8
    group_10-->acl_9
    group_11-->acl_1
    group_11-->acl_2
    group_11-->acl_3
    group_11-->acl_4
    group_13-->acl_9
    group_13-->acl_10
    group_13-->acl_11
    group_13-->acl_12
    user_2-->supplier_1
    user_3-->supplier_1
    vehicle_1-->group_2
    supplier_1-->group_8
    user_1-->group_12
    user_2-->group_10
    user_3-->group_11
    user_3-->group_9
    user_4-->group_13
```


### Field schema for settings

Certain resources may possess settings that are not classified as properties defining the resource itself; instead, they are settings that can undergo frequent changes, possibly on a daily or even faster basis. These settings will be stored in a separate table to enhance caching mechanisms. This optimization will be applied at both the database engine level as within the Realm services responsible for offering and accessing these settings. A generic data model facilitates the straightforward management of these fields within both the code and the database itself.

The `field` table provides information about the type of field. This allows for proper input validation. If a field is of the `integer` type, a min and or max value can be configured. In addition, regular expressions can be defined to allow for specific input validation (eg; `email` or `postal codes` inputs). The `value_length_min` and `value_length_max` columns can be used to provide a minimum or maximum length of the input text.

Each resource will have a combined table, defining the fields that are linked to the resource. Additionally, a `category` can be provided, allowing grouping of settings which can be used in a frontend view.


```mermaid
erDiagram
    field {
        serial field_id PK
        text field
        text type "ENUM (list, string, float, boolean, integer)"
        text regexp "Optional"
        integer min "Optional"
        integer max "Optional"
        integer value_length_min "Optional"
        integer value_length_max "Optional"
    }

    field_list_option {
        integer field_id FK "combined key"
        text key "combined key"
        text value
    }

    user_field {
        serial user_field_id PK
        integer field_id FK
        text name "eg: avatar"
        bool is_mandatory "Default false"
        text category "ENUM (additional_info, settings)"
    }
    user_field_value {
        integer user_field_id FK
        uuid user_id PK
        text value
        timestamp updated_at "Default NOW"
    }
    field ||--o{ user_field : multiple
    user_field ||--o{ user_field_value : multiple
    user ||--o{ user_field_value : multiple

    group_field {
        serial group_field_id PK
        integer field_id FK
        text name "eg: avatar"
        bool is_mandatory "Default false"
        text category "ENUM (settings)"
    }
    group_field_value {
        integer group_field_id FK
        uuid group_id PK
        text value
        timestamp updated_at "Default NOW"
    }
    field ||--o{ group_field : multiple
    group_field ||--o{ group_field_value : multiple
    group ||--o{ group_field_value : multiple

    vertiport_field {
        serial vertiport_field_id PK
        integer field_id PK
        text name "eg: schedule, status"
        boolean is_mandatory "Default false"
        text category "ENUM (additional_info, settings)"
    }
    vertiport_field_value {
        integer vertiport_field_id FK
        uuid vertiport_id PK
        text value
        timestamp updated_at "Default NOW"
    }
    field ||--o{ vertiport_field : multiple
    vertiport_field ||--o{ vertiport_field_value : multiple
    vertiport ||--o{ vertiport_field_value : multiple

    supplier_field {
        serial supplier_field_id PK
        integer field_id PK
        text name "eg: main_email, main_phone_number, website, logo_path"
        boolean is_mandatory "Default false"
        text category "ENUM (additional_info, settings)"
    }
    supplier_field_value {
        integer supplier_field_id FK
        uuid supplier_id PK
        text value
        timestamp updated_at "Default NOW"
    }
    field ||--o{ supplier_field : multiple
    supplier_field ||--o{ supplier_field_value : multiple
    supplier ||--o{ supplier_field_value : multiple

```
