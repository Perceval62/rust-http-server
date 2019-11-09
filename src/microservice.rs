
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::net::SocketAddr;

use crate::config::Microservice;

/*  */

/* This module gives a bunch of methods to help redirect the GET/POST to external APIs */

//stuff to do

//parse request
    //take get request
    //in the requested path, after the /app/ get the name of the microservice
    //check our list of microservices
        //take microservice description from config.json
    //look for the /app/ in path

pub fn parse_request_string(requested_path: &str) -> Result<(), ()>
{
    let output_buffer: Vec<Microservice> = Vec::new();
    let compare_string = requested_path.clone();

    let parsed_microservice_name: String = String::new();
    //get index of /app/ slice in the request string
    let start_index: usize = requested_path.find("/app/").unwrap();

    let long_name_of_service: &str = &requested_path[start_index..requested_path.len() as usize];

    println!("[microservice] Log: Client is asking for the following microservice: {}", long_name_of_service);

    //after /app/
    Ok( () )

}

//redirect request
    //open socket to microservice
    //send data get/post request


pub fn redirect_request(microservice_socket: SocketAddr) -> Result<(), ()>
{
    Ok(())
}
