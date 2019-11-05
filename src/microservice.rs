
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::net::SocketAddr;

mod config;

/*  */

/* This module gives a bunch of methods to help redirect the GET/POST to external APIs */

//stuff to do

//parse request
    //take get request
    //in the requested path, after the /app/ get the name of the microservice
    //check our list of microservices
        //take microservice description from config.json
    //look for the /app/ in path
fn parse_request_string()
{
    let output_buffer: config::Microservice = Vec::new(config::Microservice);
}

//redirect request
    //open socket to microservice
    //send data get/post request


pub fn redirect_request() -> Result<(), ()>
{

}
