mod config;
#[macro_use]
extern crate serde_json;

use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

use std::env;

fn main() {

    /* Get command line arguments */
    let args: Vec<String> = env::args().collect();
    /* See if user wants to generate server config */
    if args.len() > 1
    {
        match args[1].as_str()
        {
            "help"              => {print_man(); return()},
            "generate-config"   => {config::create_default_file(); return()},
            /* case nothing */
            _ => {bootstrap()},
        };
    }
    bootstrap();
}

fn bootstrap()
{
    println!("[Main] Log: Vincent Perrier Rust Server Backend.");
    println!("[Main] Warning: Usually needs to run with root/admin privileges.");
    let pref = config::get_server_info().unwrap();
    start(pref.0, pref.1);
}

fn start(address: SocketAddr, max_thread_count: u16)
{
    println!("[Main] Log: Got {} maximum thread.", max_thread_count);
    let num_threads_max: u16 = max_thread_count;
    println!("[Main] Log: Binding the server to {}", address);
    let ret = std::net::TcpListener::bind(address);
    let listener = match ret
    {
        Ok(listener) => {listener},
        Err(err) => {println!("[Main] Log: could not bind address. Check for admin privileges.");
        panic!(err)}
    };
    for tcp_streams in listener.incoming()
    {
        handle_client(tcp_streams.unwrap());
    }
}

fn handle_client(stream: std::net::TcpStream)
{
    println!("[Main] Log: Incoming connections from: {}", stream.peer_addr().unwrap());
}

fn print_man()
{
    println!(   "Rust Server Backend: \n
                 cargo-run                  -> Starts the server with the IP and port specified in \"config.json\".
                 cargo-run generate-config  -> generates a default \"config.json\" file.
                 cargo-run help             -> Prints program usage."
             );

}
