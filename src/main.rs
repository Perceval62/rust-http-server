mod config;
mod httpHandling;
#[macro_use]
extern crate serde_json;
use std::env;

/*
*   Main routine, gets command line args and launches routines accordingly.
*/
fn main() {

    /* Get command line arguments */
    let args: Vec<String> = env::args().collect();
    /* See if user wants to generate server config */
    if args.len() > 1
    {
        match args[1].as_str()
        {
            "help"              => { httpHandling::print_man(); return() },
            "generate-config"   => { config::create_default_file(); return() },
            "start"             => { bootstrap(); return() },
            /* case garbage */
            _ => {println!("[Main] Error: Unrecognised command line parameter.");},
        };
    }
    println!("Please use the following options:");
    httpHandling::print_man();
}

fn bootstrap()
{
    println!("[Main] Log: Vincent Perrier Rust Server Backend.");
    println!("[Main] Warning: Usually needs to run with root/admin privileges.");
    let pref = config::get_server_info().unwrap();
    httpHandling::start(pref.0, pref.1);
}
