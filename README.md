# Sitter

A library for managing user registration, login, and the user's things.

## Installation

Sitter is currently tightly coupled with the Neighbor application. To create the necessary schema, follow these directions:
https://github.com/jeremyandrews/neighbor/blob/main/README.md#installation

## Architecture

Sitter is inspired by the architecture of the Drupal 6 User module. It aims to provide a flexible and extensible mechanism for managing user registration, login, and permissions.

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

 - MySQL, PostgreSQL, and/or SQLite;
 - customized user table/fields (ie, email only?);
 - customized paths;
 - passwords/bcrypt;
 - actix, or agnostic server? (Rocket 0.5?);
 - api versioning;
 - ui: Vue.js?;
 - email: email validation, password resets;
