# ![RealWorld Example App](logo.png)

[![Build status](https://ci.appveyor.com/api/projects/status/8s17p2vh2f4e8a2y?svg=true)](https://ci.appveyor.com/project/davidpodhola/rust-realworld-example-app)

> ### Rust/Diesel codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld-example-apps) spec and API.


### [RealWorld](https://github.com/gothinkster/realworld)


This codebase was created to demonstrate a fully fledged fullstack application built with [Rust fast HTTP implementation Hyper](https://hyper.rs/) in including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the [Rust community styleguides & best practices](https://aturon.github.io/README.html).

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.


# How it works

This is an application written in [Rust](https://www.rust-lang.org/en-US/index.html) using these crates:

- [Hyper](https://hyper.rs/) - a fast HTTP implementation written in and for Rust
- [Serde](https://serde.rs/) - a framework for serializing and deserializing Rust data structures efficiently and generically
- [Reroute](https://github.com/gsquire/reroute) - A router for Rust's hyper framework using regular expressions
- [IIS](https://github.com/hsharpsoftware/rust-web-iis) - Set of helper functions for running web server written in Rust on Internet Information Services (IIS) 
- [Diesel](http://diesel.rs/) - Safe, Extensible ORM and Query Builder for Rust

# Getting started

- Install Rust: [https://www.rustup.rs/](https://www.rustup.rs/)
- Get [Diesel and Diesel supported database](http://diesel.rs/guides/getting-started/).
- Create database.
- Copy `conduit - sample.toml` to `conduit.toml` and set your connection string in `DATABASE_URL` there. 
- Also use `.env` and run `echo DATABASE_URL=<DATABASE_URL from conduit.toml> > .env`
- Run `diesel setup --database-url='<DATABASE_URL from conduit.toml>'` script to create the database structure and all the tables, functions etc.
- Build with `cargo build`.
- Run  with `cargo run`.
- API URL: `http://localhost:6767`, to test the requests you can you e.g. [Advanced REST Client](https://advancedrestclient.com/).

## Step by step installation on Windows

- install [chocolatey](https://chocolatey.org/install)
- install CMake `choco install cmake` (administrative cmd needed)
- install Microsoft Visual C++ Build Tools 2015 `choco install vcbuildtools` (administrative cmd needed; not possible if Visual Studio 2015 already installed, in that case but check if C++ is installed: File -> New project -> Visual C++ -> Install Visual C++ 2015 Tools for Windows Desktop)
- install [Rust](https://www.rust-lang.org/en-US/install.html), you are good to go with `stable` (or `nightly`) and `msvc`
- install [PostgreSQL](https://www.postgresql.org/download/windows/); skip Stack Builder if you do not need to install anything else
- use pgAdmin 4 from `C:\Program Files\PostgreSQL\10\pgAdmin 4\bin` to connect to the server (username `postgres`, password from the previous step)
- go to Servers -> PostgreSQL 10 -> Databases and create a new database `conduit`
- set environment variable `PQ_LIB_DIR` to `C:\Program Files\PostgreSQL\10\lib`dir (you need to call refreshenv in the opened cmd then)
- Add `C:\Program Files\PostgreSQL\10\bin` to `PATH`  (in the same windows as you set the environment variable; call refreshenv again)
- follow the steps from the Getting started sections, install `diesel` with `cargo install diesel_cli --no-default-features --features postgres`
- when running `diesel setup`, do not use the singe quotes, it needs to look like e.g. `diesel setup --database-url=postgres://postgres:Password123@localhost/conduit`
