mod config;
mod http_handling;
mod microservice;

#[macro_use]
extern crate serde_json;
use std::env;
use std::net::SocketAddr;

/*
*   Main routine, gets command line args and launches routines accordingly.
*/

fn main() {
    /* Get command line arguments */
    let args: Vec<String> = env::args().collect();
    /* See if user wants to generate server config */
    if args.len() > 1 {        match args[1].as_str() {
            "help" => {
                http_handling::print_man();
                return ();
            }

            "generate-config" => {
                config::create_default_file();
                return ();
            }

            "start" => {
                boot();
                return ();
            }

            "start-with" => {
                if args.len() > 2 {
                    let address = args[2].clone();
                    boot_with_args(address);
                    return ();
                }
            }
            /* case garbage is given */
            _ => {
                println!("[Main] Error: Unrecognised command line parameter.");
            }
        };
    }
    println!("Please use the following options:");
    http_handling::print_man();
}

/* Starts the server*/
fn boot_with_args(address_string: String) {
    println!("[Main] Log: Vincent Perrier Rust Server Backend.");
    println!("[Main] Warning: Usually needs to run with root/admin privileges.");
    /* Get the preferences in a tuple */
    let configuration = config::get_server_settings().unwrap();
    let address: SocketAddr = address_string.parse().unwrap();
    http_handling::start(address, configuration.1, configuration.2, configuration.3);
}

/* Starts the server*/
fn boot() {
    println!("[Main] Log: Vincent Perrier Rust Server Backend.");
    println!("[Main] Warning: Usually needs to run with root/admin privileges.");
    /* Get the preferences in a tuple */
    let configuration = config::get_server_settings().unwrap();
    http_handling::start(
        configuration.0,
        configuration.1,
        configuration.2,
        configuration.3,
    );
}
