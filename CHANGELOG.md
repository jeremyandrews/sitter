# Changelog

## 0.1.5
 - introduce `PersonHooks`: `validate`, `presave`, and `postsave`
 - in password hook_validate enforce minimum password length
 - in password hook_presave hash password with Argon2
 - in email hook_validate run simplistic regex on email address
 - in logger hook_postsave log Create and Update events.

## 0.1.4 Dec 15, 2020
 - implement all `CRUD` queries
 - properly receive database from Neighbor

## 0.1.3 Dec 14, 2020
## 0.1.2 Dec 14, 2020
 - implement `create()` and `read()` inside `Person` struct
 - move database management to Neighbor

## 0.1.1 Dec 12, 2020
 - initial `Create` and `Read` queries

## 0.1.0 Dec 10, 2020
 - initial commit