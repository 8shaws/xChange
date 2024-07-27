# xChange_api

## Description
Rust based API for the xChange project. The API is built using the Actix_web framework and Diesel ORM.

## Setup
1. Install Rust
2. Install Diesel CLI
3. Install Postgres
4. Create a database in Postgres
5. Run `diesel setup` to setup the database
6. Run `diesel migration run` to run the migrations
7. Run `cargo run` to start the server

## Endpoints
1. POST /user/register - Create a new user
2. POST /users/login - Login a user
