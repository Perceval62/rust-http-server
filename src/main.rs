mod config;
#[macro_use]
extern crate serde_json;

use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Shutdown;

use std::thread;


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
            /* case garbage */
            _ => {println!("[Main] Error: Unrecognised command line parameter.");},
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

fn print_man()
{
    println!(   "Rust Server Backend: \n
    cargo-run                  -> Starts the server with the IP and port specified in \"config.json\".
    cargo-run generate-config  -> generates a default \"config.json\" file.
    cargo-run help             -> Prints program usage."
);

}

fn handle_client(mut stream: std::net::TcpStream)
{
    let thread = std::thread::spawn(move || {
        println!("[Main] Log: Incoming connections from: {}", stream.peer_addr().unwrap());
        /* handle client */
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        /*let mut request: String = String::new();
        println!("HTTP REQUEST:\n{}", stream.set_nonblocking(true));*/
        stream.write(   "HTTP/1.1 200 OK\n\r\n\r
                        <html>
                            <body>
                                <h1>Vincent Perrier backend test</h1>
                            </body>
                        </html>".as_bytes()).unwrap();
    });
}
