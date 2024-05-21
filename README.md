# Dev environment Setup

## 1. Installing the dependencies
1. Clone the repo and cd into the base directory.
2. cd into `./backend` and run `cargo build`.
3. cd back into `./frontend` and run `npm i`.

## 2. Setting up the environment variables
1. Copy `.env.sample` file into `.env` and fill in the POSTGRES_USER, POSTGRES_PASSWORD fields.
2. In `frontend/src` directory, copy `.env.local.sample` into `.env.local` and make the necessary modifications to the value.

## 3. Running the app for development
We will require 3 separate terminals to track the database, backend and frontend respectively.
1. (Database testing) On the first terminal, run `docker compose up` from the base directory to set up the postgres server.
2. (Backend testing) On the 2nd terminal, cd into `./backend` and run `cargo run`.
3. (Frontend testing) On the 3rd terminal cd into `./frontend` and run `npm run dev`.

# Common Issues

## Migrations being bricked
delete the _sqlx_migrations table and try again

# Random notes

## need to install diesel cli
cargo install diesel_cli --no-default-features --features postgres