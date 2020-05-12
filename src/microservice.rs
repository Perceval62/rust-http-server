/* File Microservice.rs */

/* Import modules */

use serde::Deserialize;
use serde::Serialize;

use std::io::Write;
use std::io::Read;
use std::net::SocketAddr;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Clone)]
pub struct Microservice {
    pub address: SocketAddr,
    pub name: String,
}

/* Parses the the GET request and returns a socketAddr to it */
pub fn parse_request_string(
    requested_path: &str,
    mut microservice_list: Vec<Microservice>,
) -> Result<SocketAddr, &'static str> {

    /* Check the if at least 1 microservice exists in the config.json file. */
    if microservice_list.is_empty() {
        return Err("[microservice] Error: There is no microservices in the configuration file.");
    }

    /* Parse the requested_path and get the name of the service that was requested. */
    let start_index: usize = requested_path.find("/app/").unwrap();
    let name_of_service: &str = &requested_path[(start_index + 5)..requested_path.len() as usize];

    println!(
        "[microservice] Log: Client is asking for the following microservice: {}",
        name_of_service
    );

    /* Go through the list and look for a match with what was requested. */
    let microservice_list_iter = microservice_list.iter_mut();
    for i in microservice_list_iter {
        if i.name == name_of_service {
            let return_address: SocketAddr = i.address;
            println!(
                "[microservice] Log: Microservice with name {} was found at adress {}",
                name_of_service, i.address
            );
            return Ok(return_address);
        }
    }

    /* If no matching elements were found in the list return an error. */
    Err("[microservice] Error: There is no microservices of that name configured.")
}


/* Sends a string through a socket, returns a response */
/* Todo: add timeout if response never comes */
pub fn redirect_request(microservice_socket: SocketAddr, http_request: String) -> Result<String, ()> {

    /* Open a socket to the address given in the parameters */
    let mut socket = match TcpStream::connect(microservice_socket) {

        Ok(socket) => {
            println!("[microservice] Log: Connected to microservice");
            socket
        }

        Err(err) => {
            println!("[microservice] Log: Couldn't connect.");
            panic!("{}", err)
        }

    };
    /* Write the string in the socket */
    socket.write(http_request.as_bytes()).unwrap();

    /* Wait for a response */
    let mut response: String = String::new();
    socket.read_to_string(&mut response).unwrap();
    Ok(response)
}
