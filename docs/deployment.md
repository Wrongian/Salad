# To Deploy on your own machine

## Docker Terminologies we use

`be_server`: container name of our backend server \
`fe_server`: container name of our frontend server \
`pg_db`: container name of our postgres db

## Signing into server

1. ssh into your remote virtual private server (vps).

## Setting up remote production environment (only for first time setup)

1. Fork the repo
1. Create a Personal Access Token with full repo control on your github account with access to this repository.
1. ssh into your remote vps, then cd into some directory (preferably `~` directory.)
1. Run `git clone https://<your-github-username>:<your-personal-access-token>@github.com/<your-github-username-containing-forked-repo>/<forked-repo-name>.git Salad`

## Refreshing/updating the remote production environment

We assume that the `Salad` repository is located under `~`.

1. Change directory into `~/Salad`
1. Run `git pull`
1. Checkout to the production branch. (Run: `git checkout production`)

## Starting the server

1. cd into ~/Salad, then `docker compose up -d` (takes around 5-10mins to setup)
1. docker ps -a to check the health of each container
1. docker restart <container_name> if necessary (e.g. docker restart be_server) \

## Setting up environment variables

There are 3 .env files required in the project directory:

1. `~/Salad/.env` (Root .env)
1. `~/Salad/frontend/.env` (Frontend .env)
1. `~/Salad/backend/.env`. (Backend .env) \

For each of the .env files, all fields are necessary.

## Stopping the server

1. docker compose down
1. docker image rm -f salad-frontend_server salad-backend_server postgres:alpine
1. docker system prune -f

## Monitoring the containers

1. docker ps -a to check the status of all containers\

docker log can be used if needed

## Nginx Port Mappings

For the current server, we conventionally map the url to \
https://saladify.duckdns.org/ ->localhost:3000

## Common issues

pg_db may not finish setting up before be_server is run. This causes be_server to crash on startup. Be sure to check and perform `docker restart be_server` if necessary
