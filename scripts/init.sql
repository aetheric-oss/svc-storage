/* Initializes empty realm database with svc_storage user, only on first database run */
CREATE DATABASE realm;
CREATE USER svc_storage;
GRANT ALL PRIVILEGES ON DATABASE realm TO svc_storage;
