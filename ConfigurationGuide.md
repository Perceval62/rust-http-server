# Configuration guide
Most of the configuration is done via the "config.json" file.
If it hasn't been generated yet, `cd` into the root of the project folder and run
~~~~
sudo cargo run generate-config.
~~~~

This will generate a template "config.json" file. Here is

~~~~JSON
{
  "ip": "127.0.0.1",
  "port": 80,
  "num_threads_max": 20,
  "root_path": "./html/"
  "microservices": [
    {
      "address": "127.0.0.1:8080",
      "name": "example"
    }
  ],
}

~~~~

## Description
Here is what each of the parameters represent.

* ip: the binding ip address of the server.
* port: the binding port of the server.
* num_threads_max: the maximum number of simultaneous threads running.
* root_path: the path where to find the html, js and images of the website.
* microservice: an array of objects composed of an address & ports and a string representing the corresponding name.



## Microservices
The list of microservices is checked when a client makes an HTTP request. A client can request a particular microservice by putting `/app/name_of_microservice` in the HTTP request. At that point, the server redirects the request to the corresponding address.

Do not give the same address to a microservice as the one you have given the server.
