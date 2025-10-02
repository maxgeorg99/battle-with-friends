module := "battle-with-friends"

server:
    spacetime publish -p ./server-rust {{module}} -y -c

delete:
    spacetime delete {{module}} -y

bindings:
    spacetime generate --lang typescript --out-dir ./client/src/autobindings --project-path server-rust

all: server bindings client

setup:
    cd client && bun install

client:
    cd client && bun run dev

build:
    cd client && bun run build