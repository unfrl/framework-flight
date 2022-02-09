# Framework flight

## [Elixir](https://elixir-lang.org/) + [Phoenix](https://www.phoenixframework.org/)

- Used [asdf](http://asdf-vm.com/) for installing/managing erlang and elixir.
  - Took me a while to get erlang/elixir installed, because I had some missing dependencies before using `asdf`. For me it was openssl and/or libssl-dev: `sudo apt-get install openssl libssl-dev`.
- Phoenix **really** wants you to build a web front-end. The app it scaffolds is filled with templates, views, etc. -- feels very ruby on rails. Which isn't necessarily bad, it's just very much _"this is how you build your app"_.
  - The gen tool has `phx.gen.json` which will scaffold the schema, controller, json resource view, and organize it into a context.

## [Go](https://go.dev/) + [Gin](https://gin-gonic.com/)

- Using the [Gorm ORM Library](https://gorm.io/) and its [postgres driver](https://github.com/go-gorm/postgres).

## [Kotlin](https://kotlinlang.org/) + [Spring](https://spring.io/guides/tutorials/spring-boot-kotlin/) or [Ktor](https://ktor.io/) (TBD)

_WIP_

## [Rust](https://www.rust-lang.org/) + [Actix](https://actix.rs/)

- Using [Diesel ORM library](https://diesel.rs). Since I'm only using postgres, first need to install the postgres client before installing the diesel CLI otherwise it'll error out:
  - `sudo apt-get install libpq-dev`
  - `cargo install diesel_cli --no-default-features --features "postgres"`
