
# Database Setup: PostgreSQL

This application requires a PostgreSQL database. You may use an existing PostgreSQL instance or set up a new one quickly using a container, as detailed in the steps below.

## Existing PostgreSQL Instance

If you are using an existing PostgreSQL installation, you must ensure a database reflecting the configuration in the project's `.env` file is created.

For instance, if your `.env` contains:

`DATABASE_URL="postgres://postgres:mysecretpassword@localhost/postgres"`

The parameters break down as follows:

*    `postgres:mysecretpassword`: The username and password used for authentication.

*    `@localhost`: The hostname (or IP address) of the database server.

*    `/postgres`: The target database name.

Note on Port: PostgreSQL defaults to port 5432. If your instance uses a different port (e.g., 5444), you must explicitly include it, like: @localhost:5444.

Action: Adjust the DATABASE_URL in your .env file to correctly reflect your PostgreSQL instance parameters, then proceed to the Database Setup section.


## PostgreSQL in a Container

Running PostgreSQL in a container using Docker or Podman is the easiest method to quickly set up the required database instance.


### Pulling the Image

Pull the latest PostgreSQL image:

```bash
podman pull postgres:latest
# OR
docker pull postgres:latest
```
### Verify the image is available:

```bash
podman images
# OR
docker images
```
### Running the Container

Run the container, setting the password and mapping the default port:

```bash
podman run -it --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 postgres
```
Command Breakdown:

*    `--name my-postgres`: Assigns a readable name to the container.

*    `-e POSTGRES_PASSWORD=mysecretpassword`: Sets the environment variable for the database password.

*    `-p 5432:5432`: Maps the container's listening port (5432) to the host machine's port (5432).

*    `postgres`: Specifies the image to use.

User Note: By default, the database username is set to `postgres`. If a non-default user is required, add `-e POSTGRES_USER=newuser` to the run command.

For more details on configuring the PostgreSQL container image, please refer to the official [documentation here](https://hub.docker.com/_/postgres).

### Verification
Confirm the container is running successfully and proceed to the next section:

```bash
podman ps -a
# OR
docker ps -a
```

---

# Database Setup

Let's use the sqlx-cli command-line utility to help us easily drop, create, or reset the database specified by the `DATABASE_URL` in the `.env` file.

It is critical to set the `DATABASE_URL` environment variable before running any sqlx commands, as they operate directly on its value. This is typically done by running the `source .env` command from the project root folder.

To install sqlx-cli, run the command below. This assumes you already have the Rust toolchains installed; if not, please refer to the Rust toolchain section in the main [README](https://github.com/santhosh7403/axum-session-auth-realworld-app-leptos-postgres/blob/main/README.md).

`cargo install sqlx-cli`  - this installs sqlx utility

Now, from the project root folder, run the following commands to create the database and run the initialization SQL scripts located in the migrations folder.

```
cd axum-session-auth-realworld-app-leptos-postgres

source .env

sqlx database setup
# The command above creates the DB and runs the migrations.
```

## Other Useful Commands

Here is a quick reference for other commands available with the sqlx utility:

```
santhosh@fedora:~/axum-session-auth-realworld-app-leptos-postgres$ sqlx 
Command-line utility for SQLx, the Rust SQL toolkit.

Usage: sqlx [OPTIONS] <COMMAND>

Commands:
  database     Group of commands for creating and dropping your database
  prepare      Generate query metadata to support offline compile-time verification
  migrate      Group of commands for creating and running migrations
  completions  Generate shell completions for the specified shell
  help         Print this message or the help of the given subcommand(s)

Options:
      --no-dotenv  Do not automatically load `.env` files
  -h, --help       Print help
  -V, --version    Print version
santhosh@fedora:~/axum-session-auth-realworld-app-leptos-postgres$ sqlx database
Group of commands for creating and dropping your database

Usage: sqlx database <COMMAND>

Commands:
  create  Creates the database specified in your DATABASE_URL
  drop    Drops the database specified in your DATABASE_URL
  reset   Drops the database specified in your DATABASE_URL, re-creates it, and runs any pending migrations
  setup   Creates the database specified in your DATABASE_URL and runs any pending migrations
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
