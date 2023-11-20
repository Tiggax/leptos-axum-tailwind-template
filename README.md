# leptos-axum-tailwind-template
my  Leptos template with axum backend and tailwind integration with a nix flake


# General instructions

run `nix develop` to bring all the dependencies to scope, and then open three instances:

- your working instance (I use console with helix)

- instance running tailwind (started by running: `npm run watch`) to sync the tailwind changes in the project

- an instance running the cargo-leptos instance ( run: `cargo leptos watch`)

every change you do should update, and refresh the server that you are running on [localhost:3000](http://localhost:3000)
