FROM rust:1.78.0-buster

RUN apt-get update && apt install -y build-essential postgresql libpq-dev


COPY . ./backend 
WORKDIR backend

RUN cargo build 

EXPOSE 8080


ENTRYPOINT ["cargo", "run"]






