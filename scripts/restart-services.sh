set -e
docker compose down
docker image rm -f salad-frontend_server salad-backend_server postgres:alpine
docker system prune -f

docker compose up -d