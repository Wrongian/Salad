# Production Setup

## Docker Terminology

`be_server`: container name of our backend server \
`fe_server`: container name of our frontend server \
`pg_db`: container name of our postgres db

## Signing into server

1. ssh into your remote virtual private server (vps).

## Setting up remote production environment (only for first time setup)

1. create a Personal Access Token with full repo control on your github account with access to this repository.
2. ssh into your remote vps, then cd into some directory (preferably `~` directory.)
3. git clone https://your-personal-access-token@github.com/Wrongian/Salad.git

## Refreshing/updating the remote production environment

1. cd into `~/Salad`
2. git pull

## Starting the server

1. cd into ~/Salad, then `docker compose up -d` (takes around 5-10mins to setup)
2. docker ps -a to check the health of each container
3. docker restart <container_name> if necessary (e.g. docker restart be_server) \
   \

## Setting up environment variables

There are 3 .env files required in the project directory:

1. `~/Salad/.env` (Root .env)
2. `~/Salad/frontend/.env` (Frontend .env)
3. `~/Salad/backend/.env`. (Backend .env) \

For each of the .env files, all fields are necessary.

## Stopping the server

1. docker compose down
2. docker image rm -f salad-frontend_server salad-backend_server postgres:alpine
3. docker system prune -f

## Monitoring the containers

1. docker ps -a to check the status of all containers\

docker log can be used if needed

## Nginx Port Mappings

For the current server:\
https://saladify.duckdns.org/ ->localhost:3000
https://api-saladify.duckdns.org/ -> localhost:8080

## Common issues

pg_db may not finish setting up before be_server is run. This causes be_server to crash on startup. Be sure to check and perform `docker restart be_server` if necessary

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
2. (Backend testing) On the 2nd terminal, cd into `./backend` and run `cargo run`.
3. (Frontend testing) On the 3rd terminal cd into `./frontend` and run `npm run dev`.

# Running Containers in Docker

## Running individual containers

## Running all of the containers

1. Run `docker compose up` in the base directory

# Credits

Used for the favicon

1. [1] Flaticon, “Flaticon, The Largest Database of Free Vector Icons,” Flaticon, 2010. https://www.flaticon.com/ (accessed June 2, 2024).
