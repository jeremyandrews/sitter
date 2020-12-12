# Sitter

A library for managing user registration, login, and the user's things.

## Installation

Sitter currently uses the SQLx ORM to manage connections to a Postgres database. The CLI interface to SQLx an be used to install the Sitter schema.

```sh
cargo install --version=0.2.0 sqlx-cli --no-default-features --features postgres
```

In a development environment, you can configure database access and install the Sitter schema as follows:

```sh
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel setup
sqlx database create
sqlx migrate run
```

## Architecture

Sitter is inspired by the architecture of the Drupal 6 User module. It aims to provide a flexible and extensible mechanism for managing user registration, login, and permissions.

## Paths

The following paths are the default endpoints provided by Sitter, but they can be overridden. (Actual paths TBD)

 - api
 - api/user
 - api/user/register
 - api/user/login
 - api/user/view
 - api/user/edit
 - api/user/logout
 - api/user/delete

or, CRUD-based naming?

 - api
 - api/user
 - api/user/create (register)
 - api/user/read (login and view)
 - api/user/update (edit)
 - api/user/delete (delete)

## Notes

 - decoupled; backend-only functionality for now (no forms/HTML);
 - data and functionality exposed via a JSON api;
 - replace uid with uuid;
 - allow manually reserved names and email address;
 - allow manual blocking of users;
 - pluggable user registration;
 - pluggable user login/validation;
 - track when users log-in (logging and timestamp);
 - hook_login;
 - roles and permissions;
 - test coverage;

## Add-ons (used for blog series; proves out extensibility)

 - two-factor authentication functionality;
 - flood/throttling functionality;
 - api tokens;
 - JWT;

## Research

 - ORM (Diesel?); MySQL, PostgreSQL, and/or SQLite;
 - customized user table/fields (ie, email only?);
 - customized paths;
 - passwords/bcrypt;
 - actix, or agnostic server? (Rocket 0.5?);
 - api versioning;
 - ui: Vue.js?;
 - email: email validation, password resets;