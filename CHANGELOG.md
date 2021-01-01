# Changelog

## 0.1.7-dev
 - remove `name` from default `Person` struct, only require `email` and `password`

## 0.1.6 Dec 16, 2020
 - introduce `PersonHook`: `validate`, `prepare`, `prepare_id` and `processed`
 - introduce `PersonHooks` allowing Neighbor to register hooks
 - in password hook_validate enforce minimum password length
 - in email hook_validate run simplistic regex on email address
 - in password hook_prepare hash password with Argon2
 - in logger hook_processed log Create and Update events.

## 0.1.5, 0.1.4 Dec 15, 2020
 - implement all `CRUD` queries
 - properly receive database from Neighbor

## 0.1.3, 0.1.2 Dec 14, 2020
 - implement `create()` and `read()` inside `Person` struct
 - move database management to Neighbor

## 0.1.1 Dec 12, 2020
 - initial `Create` and `Read` queries

## 0.1.0 Dec 10, 2020
 - initial commit