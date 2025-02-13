### Rosemary
**⚠️ Rosemary is very much an early work in progress.**

Rosemary is a database management GUI written in rust using egui.\
It is specifically designed for Postgres and there are no short term plans to support any other DBMS, though I'd like to add SQLite support in the long term.

## How to run
As it's in early development there are currently no pre-built binaries.\
To compile, you'll need to set a `DATABASE_URL` environment variable that points to a Postgres DB to allow SQLx to do its verification.
If you'd like to run Rosemary you can do so through cargo with `cargo run` in the project root.\
Or you can build/run your own binary from the project root with `cargo build --release` and `./target/release/rosemary`

## Special thanks
I'm very grateful for these open source crates used in this project.\
[SQLx](https://github.com/launchbadge/sqlx)\
[egui](https://github.com/emilk/egui)
