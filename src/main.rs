mod config;
#[macro_use]
extern crate serde_json;

use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;

use std::thread;
use std::env;

use threadpool::ThreadPool;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;

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
            "help"              => { print_man(); return() },
            "generate-config"   => { config::create_default_file(); return() },
            "start"             => { bootstrap(); return() },
            /* case garbage */
            _ => {println!("[Main] Error: Unrecognised command line parameter.");},
        };
    }
    println!("Please use the following options:");
    print_man();
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
    //let mut thread_list = Vec::with_capacity(num_threads_max as usize);
    println!("[Main] Log: Started the listener thread pool with {} maximum threads", num_threads_max);

    let pool = ThreadPool::new(num_threads_max as usize);
    for tcp_streams in listener.incoming()
    {
        pool.execute(move ||
            {
                handle_client(tcp_streams.unwrap()).unwrap();
            }
        );
    }
}

fn print_man()
{
    println!(   "Rust Server Backend: Vincent Perrier\n
        cargo-run generate-config  -> Generates a default \"config.json\" file.\n
        cargo-run help             -> Prints program usage.\n
        cargo-run start            -> Starts the web server\n"
);

}

fn handle_client(mut stream: std::net::TcpStream) -> Result<(), &'static str>
{
    println!("[Main] Log: Incoming connections from: {}", stream.peer_addr().unwrap());
    /* handle client */
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();


    /* If request contains /app/ */
    // call corresponding microservice
    /* Else, basic web server */
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));
    stream.write(   "HTTP/1.1 200 OK\n\r\n\r
                    <html>
                        <body>
                            <h1>Backend test</h1>
                        </body>
                    </html>".as_bytes()).unwrap();
    Ok(())
}
