FROM node:18-alpine

COPY . ./frontend

WORKDIR frontend

RUN npm install 

RUN npm run build

# RUN node --trace-warnings build

EXPOSE 3000

ENTRYPOINT [ "node", "index" ]