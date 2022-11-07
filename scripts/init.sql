/* Initializes empty arrow database with svc_storage user, only on first database run */
CREATE DATABASE arrow;
CREATE USER svc_storage;
GRANT ALL PRIVILEGES ON DATABASE arrow TO svc_storage;
