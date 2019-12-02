# HTTP backend program with microservices redirection

## Description
This a personal hobby project. I wanted to learn about backend web development. I will use it on my own platform when I get around to build it.

This neat little utility serves a website and handles "HTPP - GET" requests like any
other HTTP server. It can be useful to isolate and modularize the backend of a web app.

It can redirect requests to microservices. If the path of the HTTP request contains /app/Name_of_app and this particular microservice has been put in the "config.json" file, the whole request is forwarded.

In the generated "config.json" file, it is possible to write a list of microservices objects. In the .json file, under the microservices field, simply add the services:
 [{"name":"Name_of_app", "address": "127.0.0.1:8080"}, ...]

## Use cases

This utility can be useful for people looking to:

* build a web app.
* containerize their app's backend infrastructure.
* build a personal platform using with microservice.
* future proof their website's backend.

## Prerequisites

This project was built on the Rust programming language and its package manager: cargo.
To install Rust and cargo, please visit their [Link to the rust website](official website).

## Quick start

clone the repo and cd into the project's folder.

Build the project with:
~~~~
cargo build --release
~~~~
The binary file is located in target/release/

Create a basic JSON configuration file using the following command:
~~~~
./target/release/backend generate-config
~~~~

Start the server with
~~~~
./target/release/backend start
~~~~
Note that you might have to launch it in sudo.

For more information about the "config.json", refer to the configuration guide (W.I.P).
