# Dev environment Setup

## 1. Installing the dependencies

1. Clone the repo and cd into the base directory.
2. Make sure you have rust(our versions are 1.78.0) installed and node(our versions are 18.17.0) installed
3. cd into `./backend` and run `cargo build`.
4. cd back into `./frontend` and run `npm i`.
5. If needed install a postgres server and run a server instance

## 2. Setting up the environment variables

1. Copy `.env.sample` file into `.env` and fill in the POSTGRES_USER, POSTGRES_PASSWORD fields.
2. In `frontend/src` directory, copy `.env.local.sample` into `.env.local` and make the necessary modifications to the value.

## 3. Running the app for development

We will require 3 separate terminals to track the database, backend and frontend respectively.

1. (Database testing) On the first terminal, run `docker compose up` from the base directory to set up the postgres server.
1. (Backend testing) On the 2nd terminal, cd into `./backend` and run `cargo run`.
1. (Frontend testing) On the 3rd terminal cd into `./frontend` and run `npm run dev`.

1. Run `docker compose up` in the base directory
