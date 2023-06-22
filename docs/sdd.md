![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# Software Design Document (SDD) - `svc-storage`

## :telescope: Overview

This document details the software implementation of `svc-storage`.

This process is responsible for handling interactions with clients for data storage and retrieval.

*Note: This module is intended to be used by other Arrow micro-services via gRPC.*

*This document is under development as Arrow operates on a pre-revenue and
pre-commercial stage. Storage requirements may evolve as per business needs,
which may result in architectural/implementation changes to the storage module.*

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Stuckee       | Lotte ([@owlot](https://github.com/owlot))                        |
| Status        | Development                                                       |

## :books: Related Documents

| Document                                                                                                          | Description
| ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| [High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices. |
| [High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Arrow microservices. |
| [Requirements - `svc-storage`](https://nocodb.arrowair.com/dashboard/#/nc/p_uyeuw6scqlnpri/table/L4/svc-storage)  | Requirements and user stories for this microservice. |
| [Concept of Operations - `svc-storage`](./conops.md)                                                              | Defines the motivation and duties of this microservice. |
| [Interface Control Document (ICD) - `svc-storage`](./icd.md)                                                      | Defines the inputs and outputs of this microservice. |

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

### Data model CockroachDB

| Value (left) | Value (right) | Meaning                       |
| ------------ | ------------- | ----------------------------- |
| \|o          | o\|           | Zero or one                   |
| \|\|         | \|\|          | Exactly one                   |
| }o           | o{            | Zero or more (no upper limit) |
| }\|          | \|{           | One or more (no upper limit)  |

```mermaid
erDiagram
    %field {
    %    serial field_id PK
    %    text field_name
    %    text field_type "string / int / bool / datetime / etc..."
    %    text re_validation "Optional - regex validation string"
    %    text validation_message "Optional"
    %}
    flight_plan {
        uuid flight_plan_id PK
        uuid pilot_id FK
        uuid vehicle_id FK
        json cargo_weight_grams
        geometry path "LINESTRING"
        text weather_conditions "Optional"
        uuid departure_vertipad_id FK
        uuid destination_vertipad_id FK
        timestamp scheduled_departure
        timestamp scheduled_arrival
        timestamp actual_departure "Optional"
        timestamp actual_arrival "Optional"
        timestamp flight_release_approval "Optional"
        timestamp flight_plan_submitted "Optional"
        uuid approved_by FK "Optional"
        text flight_status "Default DRAFT"
        text flight_priority "Default LOW"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
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
    pilot {
        uuid pilot_id PK
        text first_name
        text last_name
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
        uuid asset_group_id "Optional"
        text schedule "Optional"
        timestamp last_maintenance "Optional"
        timestamp next_maintenance "Optional"
        uuid last_vertiport_id FK "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
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
        uuid itinerary_id FK
        text status "ENUM(NOTDROPPEDOFF,DROPPEDOFF,ENROUTE,ARRIVED,PICKEDUP,COMPLETE)"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    parcel_scan {
        uuid parcel_id FK
        uuid scanner_id FK
        geometry geo_location "POINT"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    user {
        uuid user_id PK
        text auth_method "ENUM (OAUTH_GOOGLE,OAUTH_FACEBOOK,OAUTH_AZURE_AD,LOCAL)"
        text display_name
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    %user_field {
    %    serial user_field_id PK
    %    serial field_id FK
    %    bool is_mandatory "Default false"
    %    text category "ENUM (additional_info, settings)"
    %}
    %user_field_value {
    %    serial user_field_id PK
    %    uuid user_id PK
    %    text value
    %    timestamp updated_at "Default NOW"
    %}
    group {
        uuid group_id PK
        text name
        text description
        uuid parent_group_id FK "Optional"
        timestamp created_at "Default NOW"
        timestamp updated_at "Default NOW"
        timestamp deleted_at "Optional Default NULL"
    }
    user_group {
        uuid user_id FK
        uuid group_id FK
    }

    %field }o--o{ user_field : field_id
    %user_field }o--o{ user_field_value : user_field_id
    %user }o--|| user_field_value : user_id

    user }o--o{ user_group : user_id
    group }o--o{ user_group : group_id

    flight_plan }o--|| vertipad : departure_vertipad_id
    flight_plan }o--|| vertipad : destination_vertipad_id
    flight_plan }o--|| pilot : pilot_id
    flight_plan }o--|| vehicle : vehicle_id

    itinerary_flight_plan |o--|{ flight_plan : flight_plan_id
    itinerary_flight_plan |o--|| itinerary : itinerary_id

    vertipad }o--|| vertiport : vertiport_id

    parcel }o--o{ parcel_scan : parcel_id
    parcel }o--o{ itinerary : itinerary_id
    scanner }o--o{ parcel_scan : scanner_id
```
