module := "battle-with-friends"
live-module := "battle-with-friends-maincloud"

# Build and publish Rust backend to local SpacetimeDB
server:
    spacetime publish -p ./server-rust {{module}} -y -c

# Delete the local database
delete:
    spacetime delete {{module}} -y

# Generate TypeScript bindings for main client
bindings:
    spacetime generate --lang typescript --out-dir ./client/src/autobindings --project-path server-rust

# Generate TypeScript bindings for admin panel
admin-bindings:
    spacetime generate --lang typescript --out-dir ./admin-panel/src/autobindings --project-path server-rust

# Run everything: server + bindings + client
all: server bindings client

# Install dependencies for main client
setup:
    cd client && bun install

# Install dependencies for admin panel
admin-setup:
    cd admin-panel && bun install

# Run main client dev server
client:
    cd client && bun run dev

# Build main client for production
build:
    cd client && bun run build

# Run admin panel: build server, generate bindings, start admin client
admin: server admin-bindings
    cd admin-panel && bun run dev

# Publish to production SpacetimeDB cloud
publish: server
    spacetime publish -p ./server-rust -s maincloud {{live-module}} --delete-data