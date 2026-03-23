# Rust Store

## Getting Started

Create a `.env` file in the root of the project with the following content:

```env
STAGE=Local

SERVER_PORT=8080
SERVER_BODY_LIMIT=10 #MB
SERVER_TIME_OUT=30 #Seconds

DATABASE_URL=postgres://<username>:<password>@<host>:<port>/<database>
```

## Migrations

Before running the application, you need to run the database migrations to set up the necessary tables. The migrations are located in the `migrations` directory. We use `diesel` for managing database migrations.

To run the migrations, use the following command:

```bash
diesel setup # This will initialize migrations folder
diesel generate <migration_name> # This will create a new migration with up.sql and down.sql files
diesel migration run # This will run all pending migrations
diesel migration revert # This will revert the last run migration
```
