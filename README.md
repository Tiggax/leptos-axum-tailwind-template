# leptos-axum-tailwind-template
my  Leptos template with axum backend and tailwind integration with a nix flake


# General instructions

you can easily create a new project by running:

```
cargo leptos new --git https://github.com/Tiggax/leptos-axum-tailwind-template
```

then run `nix develop` to bring all the dependencies to scope, and then open three instances:

- your working instance (I use console with helix)

- instance running tailwind (started by running: `npm run watch`) to sync the tailwind changes in the project

- an instance running the cargo-leptos instance ( run: `cargo leptos watch`)

every change you do should update, and refresh the server that you are running on [localhost:3000](http://localhost:3000)

# references

this git is a personalized combination of the following repositories. If you want to search for ideas, or go deeper to how other people have made this, I suggest you look at:

- The tailwind-axum example from official leptos [git](https://github.com/leptos-rs/leptos/tree/main/examples/tailwind_axum)

- a fullstack nix implementation by [srid](https://github.com/srid/leptos-fullstack)

- official leptos git [template](https://github.com/leptos-rs/start-axum)

- full admin dashboard + prisma [repo](https://github.com/alexichepura/lapa)
