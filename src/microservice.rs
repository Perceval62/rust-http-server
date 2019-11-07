
use std::io::Write;

use std::net::SocketAddr;
use std::net::TcpStream;

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

pub fn parse_request_string(requested_path: &str, mut microservice_list: Vec<Microservice>) -> Result<SocketAddr, &'static str>
{
    let start_index: usize = requested_path.find("/app/").unwrap();

    let name_of_service: &str = &requested_path[(start_index + 5)..requested_path.len() as usize];

    println!("[microservice] Log: Client is asking for the following microservice: {}", name_of_service);

    if microservice_list.is_empty()
    {
        return Err("[microservice] Error: There is no microservices in the configuration file.");
    }

    let microservice_list_iter = microservice_list.iter_mut();
    /* Check the list of microservices */
    for i in microservice_list_iter
    {
        /* if a name of a microservice in the list matches the requested microservice*/
        if i.name == name_of_service
        {
            let return_address: SocketAddr = i.address;
            println!("[microservice] Log: Microservice with name {} was found at adress {}", name_of_service, i.address);
            return Ok(return_address);
        }
    }
    //after /app/
    Err("[microservice] Error: There is no microservices of that name configured.")
}

//redirect request
    //open socket to microservice
    //send data get/post request

pub fn redirect_request(microservice_socket: SocketAddr, http_request: String) -> Result<(), ()>
{
    let socket = match TcpStream::connect(microservice_socket)
    {
        Ok(socket) => {println!("[microservice] Log: Connected to microservice"); socket}
        Err(err) => {println!("[microservice] Log: Couldn't connect."); panic!("{}", err)},
    };
    let mut writer = std::io::BufWriter::new(socket);
    writer.write(http_request.as_bytes()).unwrap();
    Ok(())
}
