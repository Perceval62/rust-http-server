# HTTP backend program with microservices redirection

This tool acts first as an HTTP server. 
It serves a website and handles http GET requests.


## Quickstart

To get started, cd into the project directory and generate the default JSON 
configuration file using the following command:
~~~~
cargo run generate-config
~~~~
After succesfully running the command, a config.json file should be located in 
the root directory of the project.

Edit it at your heart's contempt !
Feel free to remove the template microservice in the array of the JSON file.

When ready to launch the server, simply run 
~~~~
sudo cargo run start 
~~~~

It is also possible to run 
~~~~
cargo run help
~~~~