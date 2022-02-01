# Framework flight

Implementing a simple project across various languages & popular frameworks.

# Project

Create a REST API for managing a catalog of cats:

- CRUD endpoints for managing the cats.
- Each cat should have a name and a color.
- Request validation, e.g. name/color are required returning bad request.

Web framework ideals:

- OpenAPI spec integration -- i.e. code-first definitions (and/or inferred) for defining an OpenAPI specification.
- ORM -- cats should be saved/retrieved via database.

## [Elixir](https://elixir-lang.org/) + [Phoenix](https://www.phoenixframework.org/)

Notes

- I opted to use [asdf](http://asdf-vm.com/) for installing/managing erlang and elixir.
  - Took me a while to get erlang/elixir, because I had some missing dependencies before using `asdf`. For me it was openssl and/or libssl-dev: `sudo apt-get install openssl libssl-dev`.

## [Go](https://go.dev/) + [Gin](https://gin-gonic.com/)

## [Kotlin](https://kotlinlang.org/) + [Spring](https://spring.io/guides/tutorials/spring-boot-kotlin/) or [Ktor](https://ktor.io/) (TBD)

## [Rust](https://www.rust-lang.org/) + [Actix](https://actix.rs/)
