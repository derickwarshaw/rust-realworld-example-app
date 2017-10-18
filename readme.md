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

Install Rust: [https://www.rustup.rs/](https://www.rustup.rs/)

Get [Diesel and Diesel supported database](http://diesel.rs/guides/getting-started/).

Create database.

Copy `conduit - sample.toml` to `conduit.toml` and set your connection string in `DATABASE_URL` there. 

Run `diesel setup --database-url='<DATABASE_URL from conduit.toml>'` script to create the database structure and all the tables, functions etc.

Build with `cargo build`.

API URL: `http://localhost:6767`
